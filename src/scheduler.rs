pub struct Scheduler {}

use crate::scheduler_interface::*;
use crate::substrait;

impl Scheduler {
    pub fn schedule_query(
        &self,
        physical_plan: substrait::Plan,
        query_info: QueryInfo) -> i32 {
        unimplemented!()
    }

    pub fn query_job_status(&self, query_id: i32) -> QueryStatus {
        unimplemented!()
    }

    pub fn query_execution_done(
        &self,
        fragment_id: i32,
        query_status: QueryStatus) {
        unimplemented!()
    }
}

pub static SCHEDULER_INSTANCE: Scheduler = Scheduler { } ;
