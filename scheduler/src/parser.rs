use async_recursion::async_recursion;
use datafusion::datasource::{empty::EmptyTable, DefaultTableSource};
use datafusion::execution::context::SessionState;
use datafusion::logical_expr::LogicalPlanBuilder;
use datafusion::physical_plan::joins::{HashBuildExec, HashJoinExec, HashProbeExec};
use datafusion::physical_plan::ExecutionPlan;
use datafusion::physical_planner::{DefaultPhysicalPlanner, PhysicalPlanner};
use datafusion_common::Result;

use std::collections::HashMap;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Arc;
use std::time::SystemTime;

static FRAGMENT_ID_GENERATOR: AtomicU64 = AtomicU64::new(0);

type QueryId = u64;
pub type QueryFragmentId = u64;

// Metadata struct for now
#[derive(Debug, Clone)]
pub struct QueryFragment {
    // The id assigned with this [`QueryFragment`]
    pub fragment_id: QueryFragmentId,

    // The id of the query which this plan fragment belongs to
    pub query_id: QueryId,

    // the entry into this [`QueryFragment`]
    pub root: Option<Arc<dyn ExecutionPlan>>,

    // convenience pointers into the parent [`QueryFragment`]
    pub parent_path_from_root: Vec<Vec<u32>>,

    // vector of dependant fragment ids
    pub child_fragments: Vec<QueryFragmentId>,

    // Vector of dependee Fragments
    pub parent_fragments: Vec<QueryFragmentId>,

    // Query level priority provided with the query
    pub query_priority: i64,

    // Time when this fragment was enqueued
    pub enqueued_time: Option<SystemTime>,

    // Cost of running this fragment
    pub fragment_cost: Option<usize>,
}

// Function to populate the cost of running a fragment.
//
// Currently it goes through all the execution plan nodes in the fragment
// and sums up the number of rows based on provided statistics.
// It can later used for sophisticated costs provided by the optimizer.
async fn populate_fragment_cost(fragment: &mut QueryFragment) {
    let mut cur_cost = 0;
    let root = fragment.root.clone().unwrap();

    let mut queue = vec![root];
    while let Some(node) = queue.pop() {
        let stats_option = node.statistics();
        match stats_option {
            Ok(stats) => match stats.total_byte_size {
                datafusion_common::stats::Precision::Exact(val) => {
                    cur_cost += val;
                }
                datafusion_common::stats::Precision::Inexact(val) => {
                    cur_cost += val;
                }
                datafusion_common::stats::Precision::Absent => {}
            },
            Err(_) => {}
        }
        for child in node.children() {
            queue.push(child);
        }
    }
    if cur_cost != 0 {
        fragment.fragment_cost = Some(cur_cost);
    }
}

// Wrapper function for parsing into fragments
pub async fn parse_into_fragments_wrapper(
    root: Arc<dyn ExecutionPlan>,
    query_id: u64,
    priority: i64,
    pipelined: bool,
) -> HashMap<QueryFragmentId, QueryFragment> {
    let fragment_id = FRAGMENT_ID_GENERATOR.fetch_add(1, Ordering::SeqCst);

    let mut output = HashMap::<QueryFragmentId, QueryFragment>::new();

    let root_fragment = QueryFragment {
        query_id,
        fragment_id,
        root: None,
        parent_path_from_root: vec![],
        child_fragments: vec![],
        parent_fragments: vec![],
        query_priority: priority,
        enqueued_time: None,
        fragment_cost: None,
    };
    let path = Vec::<u32>::new();
    output.insert(root_fragment.fragment_id, root_fragment);
    let new_root = if pipelined {
        parse_into_fragments(root, fragment_id, &mut output, query_id, path, priority).await
    } else {
        parse_into_fragments_naive(root, fragment_id, &mut output, query_id, path, priority).await
    };
    output.get_mut(&fragment_id).unwrap().root = Some(new_root);
    populate_fragment_cost(output.get_mut(&fragment_id).unwrap()).await;

    output
}

// Turn this into DAG traversal with book keeping
// Recursively return child node to the parent node
//  A child node is either a:
//   dummy scan node (to be modified data is executed and returned) or
//   a valid execution node
// When a node has siblings, it save itself into the output vector (since it is the start of a
//  fragment) and returns a dummy scan node
#[async_recursion]
pub async fn parse_into_fragments(
    root: Arc<dyn ExecutionPlan>,
    fragment_id: QueryFragmentId,
    output: &mut HashMap<QueryFragmentId, QueryFragment>,
    query_id: u64,
    mut path: Vec<u32>,
    priority: i64,
) -> Arc<dyn ExecutionPlan> {
    let children = root.children();

    // Trivial case of no children
    if children.is_empty() {
        return root;
    }

    // Single child just go down
    if children.len() == 1 {
        path.push(0);
        let new_child = parse_into_fragments(
            children[0].clone(),
            fragment_id,
            output,
            query_id,
            path,
            priority,
        )
        .await;
        return root.with_new_children(vec![new_child]).unwrap();
    }

    let mut new_children = Vec::<Arc<dyn ExecutionPlan>>::new();

    if let Some(node) = root.as_any().downcast_ref::<HashJoinExec>() {
        let build_side = node.left.clone();

        let build_fragment_id = FRAGMENT_ID_GENERATOR.fetch_add(1, Ordering::SeqCst);
        let build_fragment = QueryFragment {
            query_id,
            fragment_id: build_fragment_id,
            root: None,
            parent_path_from_root: Vec::new(),
            child_fragments: Vec::new(),
            parent_fragments: vec![fragment_id],
            enqueued_time: None,
            fragment_cost: None,
            query_priority: 0,
        };
        output.insert(build_fragment_id, build_fragment);

        let parsed_build_side = parse_into_fragments(
            build_side,
            build_fragment_id,
            output,
            query_id,
            Vec::new(),
            priority,
        )
        .await;

        let build_side_new = HashBuildExec::try_new(
            parsed_build_side.clone(),
            node.on.iter().map(|on| on.0.clone()).collect(),
            None,
            &node.join_type,
            None,
            node.mode,
            node.null_equals_null,
        )
        .unwrap();

        let build_fragment_ref = output.get_mut(&build_fragment_id).unwrap();
        let mut new_path = path.clone();
        new_path.push(0);
        build_fragment_ref.parent_path_from_root.push(new_path);
        build_fragment_ref.root = Some(Arc::new(build_side_new));

        let probe_side = node.right.clone();
        let probe_fragment_id = FRAGMENT_ID_GENERATOR.fetch_add(1, Ordering::SeqCst);
        let parsed_probe_side = parse_into_fragments(
            probe_side,
            probe_fragment_id,
            output,
            query_id,
            Vec::new(),
            priority,
        )
        .await;

        let new_root = HashProbeExec::try_new(
            parsed_build_side,
            parsed_probe_side,
            node.on.clone(),
            node.filter.clone(),
            &node.join_type,
            None,
            node.mode,
            node.null_equals_null,
        );
        return Arc::new(new_root.unwrap());
    }

    for (child_num, child) in (0_u32..).zip(children.into_iter()) {
        let child_fragment_id = FRAGMENT_ID_GENERATOR.fetch_add(1, Ordering::SeqCst);
        let child_query_fragment = QueryFragment {
            query_id,
            fragment_id: child_fragment_id,
            root: None,
            parent_path_from_root: vec![],
            child_fragments: vec![],
            parent_fragments: vec![fragment_id],
            query_priority: priority,
            enqueued_time: None,
            fragment_cost: None,
        };
        output.insert(child_fragment_id, child_query_fragment);

        let new_child =
            parse_into_fragments(child, child_fragment_id, output, query_id, vec![], priority)
                .await;

        let dummy_scan_node = create_dummy_scans(&new_child).await.unwrap();
        new_children.push(dummy_scan_node);

        // Get a reference to the newly created child fragment
        let child_fragment_ref = output.get_mut(&child_fragment_id).unwrap();
        let mut new_path = path.clone();
        new_path.push(child_num);
        child_fragment_ref.parent_path_from_root.push(new_path);
        child_fragment_ref.root = Some(new_child);
        populate_fragment_cost(output.get_mut(&child_fragment_id).unwrap()).await;

        output
            .get_mut(&fragment_id)
            .unwrap()
            .child_fragments
            .push(child_fragment_id);
    }

    let new_root = root.with_new_children(new_children);
    new_root.unwrap()
}

#[async_recursion]
pub async fn parse_into_fragments_naive(
    root: Arc<dyn ExecutionPlan>,
    fragment_id: QueryFragmentId,
    output: &mut HashMap<QueryFragmentId, QueryFragment>,
    query_id: u64,
    mut path: Vec<u32>,
    priority: i64,
) -> Arc<dyn ExecutionPlan> {
    let children = root.children();

    // Trivial case of no children
    if children.is_empty() {
        return root;
    }

    // Single child just go down
    if children.len() == 1 {
        path.push(0);
        let new_child = parse_into_fragments_naive(
            children[0].clone(),
            fragment_id,
            output,
            query_id,
            path,
            priority,
        )
        .await;
        return root.with_new_children(vec![new_child]).unwrap();
    }

    let mut new_children = Vec::<Arc<dyn ExecutionPlan>>::new();

    for (child_num, child) in (0_u32..).zip(children.into_iter()) {
        let child_fragment_id = FRAGMENT_ID_GENERATOR.fetch_add(1, Ordering::SeqCst);
        let child_query_fragment = QueryFragment {
            query_id,
            fragment_id: child_fragment_id,
            root: None,
            parent_path_from_root: vec![],
            child_fragments: vec![],
            parent_fragments: vec![fragment_id],
            query_priority: priority,
            enqueued_time: None,
            fragment_cost: None,
        };
        output.insert(child_fragment_id, child_query_fragment);

        let new_child = parse_into_fragments_naive(
            child,
            child_fragment_id,
            output,
            query_id,
            vec![],
            priority,
        )
        .await;

        let dummy_scan_node = create_dummy_scans(&new_child).await.unwrap();
        new_children.push(dummy_scan_node);

        // Get a reference to the newly created child fragment
        let child_fragment_ref = output.get_mut(&child_fragment_id).unwrap();
        let mut new_path = path.clone();
        new_path.push(child_num);
        child_fragment_ref.parent_path_from_root.push(new_path);
        child_fragment_ref.root = Some(new_child);
        populate_fragment_cost(output.get_mut(&child_fragment_id).unwrap()).await;

        output
            .get_mut(&fragment_id)
            .unwrap()
            .child_fragments
            .push(child_fragment_id);
    }

    let new_root = root.with_new_children(new_children);
    new_root.unwrap()
}

// Dummy Scan nodes will created using [`plan`], attached to its parents.
// Update these dummy nodes as results are produced by the execution team.
async fn create_dummy_scans(plan: &Arc<dyn ExecutionPlan>) -> Result<Arc<dyn ExecutionPlan>> {
    let empty_table = Arc::new(EmptyTable::new(plan.schema()));
    let table_source = Arc::new(DefaultTableSource::new(empty_table));

    // create a LogicalPlanBuilder for a table scan
    let builder = LogicalPlanBuilder::scan("dummy", table_source, None)?;
    let plan = builder.build()?;

    // Create the physical plan from the logical plan
    let physical_planner = DefaultPhysicalPlanner::default();
    let session_state = SessionState::new_with_config_rt(Default::default(), Default::default());

    let execution_plan = physical_planner
        .create_physical_plan(&plan, &session_state)
        .await
        .unwrap();

    Ok(execution_plan)
}

#[cfg(test)]
mod tests {
    use crate::parser::*;
    use datafusion::arrow::datatypes::{DataType, Field, Schema, SchemaRef};
    use datafusion::execution::context::SessionContext;
    use datafusion::execution::options::CsvReadOptions;

    use datafusion::logical_expr::JoinType;

    use datafusion::physical_plan::sorts::sort::SortExec;
    use datafusion::physical_plan::sorts::sort_preserving_merge::SortPreservingMergeExec;

    use datafusion::physical_plan::{
        coalesce_batches::CoalesceBatchesExec, empty::EmptyExec, filter::FilterExec,
        joins::NestedLoopJoinExec,
    };

    use datafusion_expr::{col, lit, LogicalPlan};
    use more_asserts as ma;

    async fn create_physical_plan(logical_plan: LogicalPlan) -> Result<Arc<dyn ExecutionPlan>> {
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

        create_physical_plan(logical_plan).await
    }

    fn validate_toy_physical_plan_structure(root_node: &Arc<dyn ExecutionPlan>) {
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
        validate_toy_physical_plan_structure(&physical_plan);

        // Returns a hash map from query fragment ID to physical plan fragment structs
        let fragments = parse_into_fragments_wrapper(physical_plan, 0, 0, true).await;

        assert_eq!(fragments.len(), 1);
        let plan_fragment = fragments.iter().next().unwrap().1;

        ma::assert_ge!(plan_fragment.query_id, 0);
        ma::assert_ge!(plan_fragment.fragment_id, 0);
        assert!(plan_fragment.root.is_some());

        let frag_node0 = plan_fragment.root.clone().unwrap();
        validate_toy_physical_plan_structure(&frag_node0);

        assert!(plan_fragment.child_fragments.is_empty());
        assert!(plan_fragment.parent_fragments.is_empty());
        assert!(plan_fragment.parent_path_from_root.is_empty());
    }

    async fn build_basic_physical_plan() -> Result<Arc<dyn ExecutionPlan>> {
        // create a logical table source
        let price_schema = Schema::new(vec![
            Field::new("item_id", DataType::Int32, true),
            Field::new("price", DataType::Utf8, true),
        ]);
        let order_schema = Schema::new(vec![
            Field::new("order_id", DataType::Int32, true),
            Field::new("item_id", DataType::Int32, true),
            Field::new("quantity", DataType::Int32, true),
        ]);
        let price_table = Arc::new(EmptyTable::new(SchemaRef::new(price_schema)));
        let price_table_source = Arc::new(DefaultTableSource::new(price_table));

        let order_table = Arc::new(EmptyTable::new(SchemaRef::new(order_schema)));
        let order_table_source = Arc::new(DefaultTableSource::new(order_table));

        let exprs = vec![col("price.item_id").eq(col("order.item_id"))];

        // create a LogicalPlanBuilder for a table scan
        let right_plan = LogicalPlanBuilder::scan("price", price_table_source, None)?.build()?;

        let plan = LogicalPlanBuilder::scan("order", order_table_source, None)?
            .join_on(right_plan, JoinType::Inner, exprs)?
            .build()?;

        create_physical_plan(plan).await
    }

    fn validate_basic_physical_plan_structure(root_node: &Arc<dyn ExecutionPlan>) {
        root_node
            .as_any()
            .downcast_ref::<NestedLoopJoinExec>()
            .unwrap();

        let node0_children = root_node.children();
        assert_eq!(node0_children.len(), 2);

        let node1 = node0_children.first().unwrap();
        node1.as_any().downcast_ref::<EmptyExec>().unwrap();

        let node2 = node0_children.get(1).unwrap();
        node2.as_any().downcast_ref::<EmptyExec>().unwrap();

        assert_eq!(node1.children().len(), 0);
        assert_eq!(node2.children().len(), 0);
    }

    #[tokio::test]
    async fn basic_test() {
        let physical_plan = build_basic_physical_plan().await.unwrap();
        println!("Physical Plan: {:#?}", physical_plan);
        validate_basic_physical_plan_structure(&physical_plan);

        // Returns a hash map from query fragment ID to physical plan fragment structs
        let fragments = parse_into_fragments_wrapper(physical_plan, 0, 0, true).await;

        assert_eq!(fragments.len(), 3);

        let mut root_fragment = None;
        let mut child_fragment_vec = Vec::<QueryFragment>::new();
        for (_, fragment) in fragments {
            assert!(fragment.root.is_some());
            if fragment.root.as_ref().unwrap().children().len() == 2 {
                root_fragment = Some(fragment);
            } else {
                child_fragment_vec.push(fragment);
            }
        }
        assert_eq!(child_fragment_vec.len(), 2);

        let root_fragment = root_fragment.unwrap();

        let root_child_fragments: &Vec<QueryFragmentId> = root_fragment.child_fragments.as_ref();

        for fragment in child_fragment_vec {
            // Check that each fragment that is not the root is included as a
            // child fragment of the root
            assert!(root_child_fragments
                .iter()
                .any(|&x| x == fragment.fragment_id));

            // Each child fragment should have one parent fragment
            assert_eq!(fragment.parent_fragments.len(), 1);

            // The parent fragment should be the root
            assert_eq!(
                *fragment.parent_fragments.first().unwrap(),
                root_fragment.fragment_id
            );

            assert_eq!(fragment.parent_path_from_root.len(), 1);

            let path_vec = fragment.parent_path_from_root.first().unwrap();

            assert_eq!(path_vec.len(), 1);

            let child_index = path_vec.iter().next().unwrap();

            let fragment_root = fragment.root.as_ref().unwrap();

            // The left plan is the one that scans the order table. Check if
            // they are assigned the correct path.
            if fragment_root
                .schema()
                .column_with_name("quantity")
                .is_some()
            {
                assert_eq!(child_index, &0);
            } else {
                assert_eq!(child_index, &1);
            }
        }
    }

    async fn build_plan_with_hash_join() -> Result<Arc<dyn ExecutionPlan>> {
        let ctx = SessionContext::new();
        ctx.register_csv(
            "orders",
            "src/example_data/orders.csv",
            CsvReadOptions::new(),
        )
        .await?;
        ctx.register_csv(
            "prices",
            "src/example_data/prices.csv",
            CsvReadOptions::new(),
        )
        .await?;

        // create a plan to run a SQL query
        let sql = "SELECT a.*, b.price from orders a inner join prices b on a.item_id = b.item_id order by a.order_id";
        let logical_plan = ctx.state().create_logical_plan(sql).await?;
        let physical_plan = ctx.state().create_physical_plan(&logical_plan).await?;
        Ok(physical_plan)
    }

    async fn validate_hash_join_plan(root_node: &Arc<dyn ExecutionPlan>) {
        root_node
            .as_any()
            .downcast_ref::<SortPreservingMergeExec>()
            .unwrap();
        let node0_children = root_node.children();

        assert_eq!(node0_children.len(), 1);

        let node1 = node0_children.first().unwrap();
        node1.as_any().downcast_ref::<SortExec>().unwrap();

        let node1_children = node1.children();
        assert_eq!(node1_children.len(), 1);

        let node2 = node1_children.first().unwrap();
        node2
            .as_any()
            .downcast_ref::<CoalesceBatchesExec>()
            .unwrap();

        let node2_children = node2.children();
        assert_eq!(node2_children.len(), 1);

        let node3 = node2_children.first().unwrap();
        node3.as_any().downcast_ref::<HashJoinExec>().unwrap();

        let node3_children = node3.children();
        assert_eq!(node3_children.len(), 2);
    }

    #[tokio::test]
    async fn test_left_deep_join() {
        let plan = build_plan_with_hash_join().await.unwrap();
        validate_hash_join_plan(&plan).await;

        let fragments = parse_into_fragments_wrapper(plan, 0, 0, true).await;
        print!("{:?}", fragments);

        let mut found_hash_build = false;

        for (_, fragment) in fragments {
            assert!(fragment.root.is_some());
            if let Some(_) = fragment
                .root
                .clone()
                .unwrap()
                .as_any()
                .downcast_ref::<HashBuildExec>()
            {
                found_hash_build = true;
            }
        }

        assert!(found_hash_build);
    }
}
