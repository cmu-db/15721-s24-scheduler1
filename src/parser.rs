use datafusion::datasource::{empty::EmptyTable, DefaultTableSource};
use datafusion::execution::context::SessionState;
use datafusion::logical_expr::LogicalPlanBuilder;
use datafusion::physical_plan::ExecutionPlan;
use datafusion::physical_planner::{DefaultPhysicalPlanner, PhysicalPlanner};
use datafusion_common::Result;
use std::collections::HashMap;

use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Arc;

static QUERY_ID_GENERATOR: AtomicU64 = AtomicU64::new(0);
static FRAGMENT_ID_GENERATOR: AtomicU64 = AtomicU64::new(0);

type QueryId = u64;
pub type QueryFragmentId = u64;

// Metadata struct for now
pub struct PhysicalPlanFragment {
    // The id assigned with this [`PhysicalPlanFragment`]
    pub fragment_id: QueryFragmentId,

    // The id of the query which this plan fragment belongs to
    pub query_id: QueryId,

    // the entry into this [`PhysicalPlanFragment`]
    pub root: Option<Arc<dyn ExecutionPlan>>,

    // convenience pointers into the parent [`PhysicalPlanFragment`]
    pub parent_path_from_root: Vec<Vec<u32>>,

    // vector of dependant fragment ids
    pub child_fragments: Vec<QueryFragmentId>,

    pub parent_fragments: Vec<QueryFragmentId>,
}

// Wrapper function for parsing into fragments
fn parse_into_fragments_wrapper(
    root: Arc<dyn ExecutionPlan>,
) -> HashMap<QueryFragmentId, PhysicalPlanFragment> {
    let query_id = QUERY_ID_GENERATOR.fetch_add(1, Ordering::SeqCst);
    let fragment_id = FRAGMENT_ID_GENERATOR.fetch_add(1, Ordering::SeqCst);

    let mut output = HashMap::<QueryFragmentId, PhysicalPlanFragment>::new();

    let root_fragment = PhysicalPlanFragment {
        query_id,
        fragment_id,
        root: None,
        parent_path_from_root: vec![],
        child_fragments: vec![],
        parent_fragments: vec![],
    };
    let path = Vec::<u32>::new();
    output.insert(root_fragment.fragment_id, root_fragment);
    let new_root = parse_into_fragments(root, fragment_id, &mut output, query_id, path);
    output.get_mut(&fragment_id).unwrap().root = Some(new_root);

    output
}

// Turn this into DAG traversal with book keeping
// Recursively return child note to the parent node
//  A child node is either a:
//   dummy scan node (to be modified data is executed and returned) or
//   a valid execution node
// When a node has siblings, it save itself into the output vector (since it is the start of a
//  fragment) and returns a dummy scan node
fn parse_into_fragments(
    root: Arc<dyn ExecutionPlan>,
    fragment_id: QueryFragmentId,
    output: &mut HashMap<QueryFragmentId, PhysicalPlanFragment>,
    query_id: u64,
    mut path: Vec<u32>,
) -> Arc<dyn ExecutionPlan> {
    let children = root.children();

    // Trivial case of no children
    if children.is_empty() {
        return root;
    }

    // Single child just go down
    if children.len() == 1 {
        path.push(0);
        let new_child =
            parse_into_fragments(children[0].clone(), fragment_id, output, query_id, path);
        return root.with_new_children(vec![new_child]).unwrap();
    }

    let mut new_children = Vec::<Arc<dyn ExecutionPlan>>::new();

    let mut child_num: u32 = 0;

    for child in children {
        let child_fragment_id = FRAGMENT_ID_GENERATOR.fetch_add(1, Ordering::SeqCst);
        let child_query_fragment = PhysicalPlanFragment {
            query_id,
            fragment_id: child_fragment_id,
            root: None,
            parent_path_from_root: vec![],
            child_fragments: vec![],
            parent_fragments: vec![fragment_id],
        };
        output.insert(child_fragment_id, child_query_fragment);

        let new_child = parse_into_fragments(child, child_fragment_id, output, query_id, vec![]);

        let dummy_scan_node = create_dummy_scans(&new_child).unwrap();
        new_children.push(dummy_scan_node);

        let child_fragment_ref = output.get_mut(&child_fragment_id).unwrap();
        let mut new_path = path.clone();
        new_path.push(child_num);
        child_fragment_ref.parent_path_from_root.push(new_path);
        child_fragment_ref.root = Some(new_child);

        output
            .get_mut(&fragment_id)
            .unwrap()
            .child_fragments
            .push(child_fragment_id);

        child_num += 1;
    }

    let new_root = root.with_new_children(new_children);
    new_root.unwrap()
}

// Dummy Scan nodes will created using [`plan`], attached to its parents.
// Update these dummy nodes as results are produced by the execution team.
#[tokio::main]
async fn create_dummy_scans(plan: &Arc<dyn ExecutionPlan>) -> Result<Arc<dyn ExecutionPlan>> {
    let empty_table = Arc::new(EmptyTable::new(plan.schema()));
    let table_source = Arc::new(DefaultTableSource::new(empty_table));
    let projection = None;

    // create a LogicalPlanBuilder for a table scan
    let builder = LogicalPlanBuilder::scan("", table_source, projection)?;
    let plan = builder.build()?;

    // Create the physical plan from the logical plan
    let physical_planner = DefaultPhysicalPlanner::default();
    let session_state = SessionState::new_with_config_rt(Default::default(), Default::default());

    let execution_plan = physical_planner
        .create_physical_plan(&plan, &session_state)
        .await?;

    Ok(execution_plan)
}

#[cfg(test)]
mod tests {
    use crate::parser::*;
    use datafusion::arrow::datatypes::{DataType, Field, Schema, SchemaRef};
    use datafusion::physical_plan::{
        coalesce_batches::CoalesceBatchesExec, empty::EmptyExec, filter::FilterExec,
    };
    use datafusion_expr::{col, lit};

    async fn build_toy_physical_plan() -> Result<Arc<dyn ExecutionPlan>> {
        // create a logical table source
        let schema = Schema::new(vec![
            Field::new("id", DataType::Int32, true),
            Field::new("name", DataType::Utf8, true),
        ]);
        let empty_table = Arc::new(EmptyTable::new(SchemaRef::new(schema)));
        let table_source = Arc::new(DefaultTableSource::new(empty_table));

        // optional projection
        let projection = None;

        // create a LogicalPlanBuilder for a table scan
        let builder = LogicalPlanBuilder::scan("person", table_source, projection)?;

        // perform a filter operation and build the plan
        let logical_plan = builder
            .filter(col("id").gt(lit(500)))? // WHERE id > 500
            .build()?;
        // Set default for all context
        let physical_planner = DefaultPhysicalPlanner::default();
        let session_state =
            SessionState::new_with_config_rt(Default::default(), Default::default());

        // Create the physical plan from the logical plan
        let execution_plan = physical_planner
            .create_physical_plan(&logical_plan, &session_state)
            .await?;

        Ok(execution_plan)
    }

    fn validate_physical_plan_structure(root_node: &Arc<dyn ExecutionPlan>) {
        root_node
            .as_any()
            .downcast_ref::<CoalesceBatchesExec>()
            .unwrap();

        let node0_children = root_node.children();
        assert_eq!(node0_children.len(), 1);

        let node1 = node0_children.first().unwrap();
        node1.as_any().downcast_ref::<FilterExec>().unwrap();

        let node1_children = node1.children();
        assert_eq!(node1_children.len(), 1);

        let node2 = node1_children.first().unwrap();
        node2.as_any().downcast_ref::<EmptyExec>().unwrap();

        let node2_children = node2.children();
        assert!(node2_children.is_empty());
    }

    #[tokio::test]
    async fn sanity_check() {
        let physical_plan = build_toy_physical_plan().await.unwrap();
        println!("Physical Plan: {:#?}", physical_plan);
        assert_eq!(physical_plan.children().len(), 1);
        validate_physical_plan_structure(&physical_plan);

        // Returns a hash map from query fragment ID to physical plan fragment structs
        let fragments = parse_into_fragments_wrapper(physical_plan);

        assert_eq!(fragments.len(), 1);
        assert!(fragments.get(&0).is_some());

        let plan_fragment = fragments.get(&0).unwrap();

        assert_eq!(plan_fragment.query_id, 0);
        assert_eq!(plan_fragment.fragment_id, 0);
        assert!(plan_fragment.root.is_some());

        let frag_node0 = plan_fragment.root.clone().unwrap();
        validate_physical_plan_structure(&frag_node0);

        assert!(plan_fragment.child_fragments.is_empty());
        assert!(plan_fragment.parent_fragments.is_empty());
        assert!(plan_fragment.parent_path_from_root.is_empty());
    }
}
