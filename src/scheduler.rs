use crate::scheduler_interface::*;
use datafusion::physical_plan::ExecutionPlan;

use std::sync::Arc;

pub struct Scheduler {}

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

pub static SCHEDULER_INSTANCE: Scheduler = Scheduler {};
