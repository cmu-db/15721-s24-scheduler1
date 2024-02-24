use datafusion::physical_plan::ExecutionPlan;

type QueryID = u32;
type QueryFragmentID = u32;

pub struct QueryFragment{
    pub execution_plan: Box<dyn ExecutionPlan>,
    pub priority: u32,
    pub query_id: QueryID,
    pub query_fragment_id: QueryFragmentID,
    pub dependent: Vec<QueryFragmentID>,  
    pub dependee: Vec<QueryFragmentID>,


}