use datafusion::datasource::empty::EmptyTable;
use datafusion::datasource::DefaultTableSource;
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

type QueryID = u64;
type QueryFragmentID = u64;

// Meta data struct for now
pub struct PhysicalPlanFragments {
    // The id of the query which this plan fragment belongs to
    query_id: QueryID,
    // The id assigned to this [`PhysicalPlanFragments`]
    fragment_id: QueryFragmentID,
    // the entry into the this [`PhysicalPlanFragments`]
    root: Option<Arc<dyn ExecutionPlan>>,
    // convenience pointers into the parent [`PhysicalPlanFragments`]
    parent_path_from_root: Vec<Vec<u32>>,
    // vector of dependant fragment ids
    child_fragments: Vec<QueryFragmentID>,
    parent_fragments: Vec<QueryFragmentID>,
}

// Wrapper function for parsing into fragments
fn parse_into_fragments_wrapper(
    root: Arc<dyn ExecutionPlan>,
) -> HashMap<QueryFragmentID, PhysicalPlanFragments> {
    let query_id = QUERY_ID_GENERATOR.fetch_add(1, Ordering::SeqCst);
    let _fragment_id_generator = AtomicU64::new(0);
    let mut output = HashMap::<QueryFragmentID, PhysicalPlanFragments>::new();
    let fragment_id = FRAGMENT_ID_GENERATOR.fetch_add(1, Ordering::SeqCst);
    let root_fragment = PhysicalPlanFragments {
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
//  fragment) and return a dummy scan node
fn parse_into_fragments(
    root: Arc<dyn ExecutionPlan>,
    fragment_id: QueryFragmentID,
    output: &mut HashMap<QueryFragmentID, PhysicalPlanFragments>,
    query_id: u64,
    path: Vec<u32>,
) -> Arc<dyn ExecutionPlan> {
    let children = root.children();

    // Trivial case of no children
    if children.is_empty() {
        return root;
    }

    // Single child just go down
    if children.len() == 1 {
        let mut new_path = path.clone();
        new_path.push(0);
        return parse_into_fragments(children[0].clone(), fragment_id, output, query_id, new_path);
    }

    let mut new_children = Vec::<Arc<dyn ExecutionPlan>>::new();

    let mut child_num: u32 = 0;

    for child in children.into_iter() {
        let child_fragment_id = FRAGMENT_ID_GENERATOR.fetch_add(1, Ordering::SeqCst);
        let child_query_fragment = PhysicalPlanFragments {
            query_id,
            fragment_id: child_fragment_id,
            root: None,
            parent_path_from_root: vec![],
            child_fragments: vec![],
            parent_fragments: vec![fragment_id],
        };
        output.insert(child_fragment_id, child_query_fragment);

        let new_child = parse_into_fragments(child, child_fragment_id, output, query_id, vec![]);

        let new_dummy_node = create_dummy_scans(new_child.clone()).unwrap();
        new_children.push(new_dummy_node);

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

// Dummy Scan nodes will created usuing [`plan`], attached to its parents
//  Update these dummy nodes as results are produced by the execution team
#[tokio::main]
async fn create_dummy_scans(plan: Arc<dyn ExecutionPlan>) -> Result<Arc<dyn ExecutionPlan>> {
    let empty_table = Arc::new(EmptyTable::new(plan.schema()));
    let table_source = Arc::new(DefaultTableSource::new(empty_table));
    let projection = None;

    // create a LogicalPlanBuilder for a table scan
    let builder = LogicalPlanBuilder::scan("", table_source, projection)?;
    let plan = builder.build()?;

    // Create the physical plan from the logical plan
    let physical_planner = DefaultPhysicalPlanner::default();
    let session_state = SessionState::new_with_config_rt(Default::default(), Default::default());

    // Create the physical plan from the logical plan
    let execution_plan = physical_planner
        .create_physical_plan(&plan, &session_state)
        .await?;

    Ok(execution_plan)
}
