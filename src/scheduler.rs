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
        physical_plan: Arc<dyn ExecutionPlan>,
        query_info: QueryInfo,
    ) -> i32 {
        unimplemented!()
    }

    pub fn query_job_status(&self, query_id: i32) -> QueryStatus {
        unimplemented!()
    }

    pub fn query_execution_done(&self, fragment_id: i32, query_status: QueryStatus) {
        unimplemented!()
    }

    pub fn parse_physical_plan(&self, physical_plan: &dyn ExecutionPlan) {}
}

pub static SCHEDULER_INSTANCE: Scheduler = Scheduler {};
