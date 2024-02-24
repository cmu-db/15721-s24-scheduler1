use std::sync::Arc;
use std::sync::atomic::{ AtomicU64, Ordering };
use datafusion::physical_plan::ExecutionPlan;
use datafusion::physical_planner;
use datafusion_proto::generated::datafusion::PhysicalPlanNode;
use std::collections::HashMap;

static QUERY_ID : AtomicU64 = AtomicU64::new(0);

type QueryID = u32;
type QueryFragmentID = u32;

// Meta data struct for now
pub struct PhysicalPlanFragments {
    // The id of the query which this plan fragment belongs to
    query_id: QueryID,
    // The id assigned to this [`PhysicalPlanFragments`]
    fragment_id: QueryFragmentID,
    // the entry into the this [`PhysicalPlanFragments`]
    root: Option<PhysicalPlanNode>,
    // convenience pointers into the parent [`PhysicalPlanFragments`]
    parent_pointers: Vec<Arc<PhysicalPlanFragments>>,
    // vector of dependant fragment ids
    child_fragments: Vec<QueryFragmentID>,
    parent_fragments: Vec<QueryFragmentID>,

}

// Wrapper function for parsing into fragments
fn parse_into_fragments_wrapper(root: Arc<dyn ExecutionPlan>) -> Vec<PhysicalPlanFragments> {
    let query_id = QUERY_ID.fetch_add(1, Ordering::SeqCst);
    let fragment_id = AtomicU64::new(0);
    let mut output = HashMap::<QueryFragmentID, PhysicalPlanFragments>::new();
    let mut rootFragment = PhysicalPlanFragments{query_id,fragment_id, None, !vec[],!vec[],!vec[] };
    let _ = parse_into_fragments(root, true, &mut output, query_id, fragment_id);
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
    siblings: bool, 
    output: &mut HashMap::<QueryFragmentID, PhysicalPlanFragments>::new(),
    query_id: u64,
    fragment_id_generator: AtomicU64,
    ) -> PhysicalPlanFragments {
    
    let children = root.children();
    let children_sib = children.len() > 1;

    let mut new_children = Vec::<Arc<dyn ExecutionPlan>>::new();
    let mut physical_plan_fragments = Vec::<PhysicalPlanFragments>::new();

    // generates 
    for child in children.into_iter() {
        physical_plan_fragments.push(parse_into_fragments(child, children_sib, output, query_id, fragment_id_generator));
    }

    let mut return_fragment = PhysicalPlanFragments::Default();

    // do stuff with physical_plan_fragments
    for frags in physical_plan_fragments.into_iter() {
        new_children.push(frags.root.clone());
        return_fragment.parent_pointers.push
    }
    
    let new_root = root.with_new_children(new_children);

    if siblings {
        let fragment_id = fragment_id_generator.fetch_add(1, Ordering::SeqCst);
        output.push(create_physical_plan_fragments(new_root, query_id, fragment_id, ));
        return create_dummy_scans(root);
    }
    new_root

}

fn create_physical_plan_fragments(
    root: Arc<dyn ExecutionPlan>,
    query_id: u64,
    fragment_id: u64,

    num_dependencies: AtomicU64,
    ) -> PhysicalPlanFragments {
    return PhysicalPlanFragments { 
        query_id: (),
        fragment_id: (), 
        root: (), 
        parent_pointers: (), 
        child_stat: () 
    }

}

// Dummy Scan nodes will created usuing [`plan`], attached to its parents
//  Update these dummy nodes as results are produced by the execution team
fn create_dummy_scans(plan: Arc<dyn ExecutionPlan>) -> Result<Arc<dyn ExecutionPlan>> {

    let empty_table = Arc::new(EmptyTable::new(plan.schema()));
    let table_source = Arc::new(DefaultTableSource::new(empty_table));
    let projection = None;

    // create a LogicalPlanBuilder for a table scan
    let builder = LogicalPlanBuilder::scan(format!("{}", input_id), table_source, projection)?;
    let plan = builder.build()?;

    // Create the physical plan from the logical plan
    let session_state = SessionState::new_with_config_rt(Default::default(), Default::default());
    let execution_plan = physical_planner::DefaultPhysicalPlanner
        .create_physical_plan(&logical_plan, &session_state)
        .await?;

    Ok(execution_plan)
}

// 
