use crate::parser::{PhysicalPlanFragment, QueryFragmentId};
use crate::scheduler_interface::*;
use datafusion::physical_plan::ExecutionPlan;

extern crate lazy_static;
use lazy_static::lazy_static;
use std::collections::HashMap;
use std::sync::Arc;
use std::sync::Mutex;

#[derive(Debug)]
pub struct Scheduler {
    pub all_fragments: HashMap<QueryFragmentId, PhysicalPlanFragment>,
    pub pending_fragments: Vec<QueryFragmentId>,
}

pub enum PipelineBreakers {
    Aggregate,
    Sort,
    Join,
    Set,
    Cross,
    Reference,
    Write,
    Ddl,
    HashJoin,
    MergeJoin,
    NestedLoopJoin,
    Window,
    Exchange,
    Expand,
}

pub struct IntermediateNode {}

impl Scheduler {
    pub fn schedule_query(
        &self,
        _physical_plan: Arc<dyn ExecutionPlan>,
        _query_info: QueryInfo,
    ) -> i32 {
        unimplemented!()
    }

    pub fn query_job_status(&self, _query_id: i32) -> QueryStatus {
        unimplemented!()
    }

    pub fn query_execution_done(&self, _fragment_id: i32, _query_status: QueryStatus) {
        unimplemented!()
    }

    pub fn parse_physical_plan(&self, _physical_plan: &dyn ExecutionPlan) {}
}

lazy_static! {
    pub static ref SCHEDULER_INSTANCE: Mutex<Scheduler> = Mutex::new(Scheduler {
        all_fragments: HashMap::new(),
        pending_fragments: vec![],
    });
}
