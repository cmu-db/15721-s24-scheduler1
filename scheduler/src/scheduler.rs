use crate::parser::{parse_into_fragments_wrapper, PhysicalPlanFragment, QueryFragmentId};
use crate::queue::add_fragments_to_scheduler;
use crate::scheduler_interface::*;
use datafusion::physical_plan::ExecutionPlan;

extern crate lazy_static;
use lazy_static::lazy_static;
use tokio::sync::Mutex;

use std::collections::HashMap;
use std::sync::Arc;
use std::sync::atomic::{AtomicU64, Ordering};

static QUERY_ID_GENERATOR: AtomicU64 = AtomicU64::new(0);

#[derive(Debug)]
struct ExecutorHandle {
    port: i32
}

#[derive(Debug)]
pub struct Scheduler {
    pub all_fragments: HashMap<QueryFragmentId, PhysicalPlanFragment>,
    pub pending_fragments: Vec<QueryFragmentId>,
    executors: Vec<ExecutorHandle>,
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
    pub async fn schedule_query(
        &self,
        physical_plan: Arc<dyn ExecutionPlan>,
        _query_info: QueryInfo,
    ) -> i32 {
        let query_id = QUERY_ID_GENERATOR.fetch_add(1, Ordering::SeqCst);
        let fragments = parse_into_fragments_wrapper(physical_plan, query_id, 1).await;
        add_fragments_to_scheduler(fragments);
        return 0;
    }

    pub fn query_job_status(&self, _query_id: i32) -> QueryStatus {
        unimplemented!()
    }

    pub fn query_execution_done(&self, _fragment_id: i32, _query_status: QueryStatus) {
        unimplemented!()
    }

    pub fn parse_physical_plan(&self, _physical_plan: &dyn ExecutionPlan) {}

    pub fn register_executor(&mut self, port: i32) {
        self.executors.push(ExecutorHandle { port });
        println!("Executor registered; port={port}");
    }

    pub async fn get_plan_from_queue(&self) -> Option<PhysicalPlanFragment> {
        crate::queue::get_plan_from_queue().await
    }
}

lazy_static! {
    pub static ref SCHEDULER_INSTANCE: Mutex<Scheduler> = Mutex::new(Scheduler {
        all_fragments: HashMap::new(),
        pending_fragments: vec![],
        executors: vec![],
    });
}
