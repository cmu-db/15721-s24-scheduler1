use crate::parser::{parse_into_fragments_wrapper, PhysicalPlanFragment, QueryFragmentId};
use crate::queue::{add_fragments_to_scheduler, finish_fragment};
use crate::scheduler_interface::*;
use datafusion::datasource::physical_plan::FileScanConfig;
use datafusion::physical_plan::joins::HashBuildResult;
use datafusion::physical_plan::ExecutionPlan;

extern crate lazy_static;
use lazy_static::lazy_static;
use tokio::sync::RwLock;

use std::collections::HashMap;
use std::sync::atomic::{AtomicU64, Ordering};
use std::time::SystemTime;

static QUERY_ID_GENERATOR: AtomicU64 = AtomicU64::new(0);
use super::debug_println;
use std::sync::Arc;
use tokio::sync::mpsc::Sender;

#[derive(Debug)]
struct ExecutorHandle {
    port: i32,
}

#[derive(Debug)]
pub struct Scheduler {
    pub all_fragments: RwLock<HashMap<QueryFragmentId, PhysicalPlanFragment>>,
    pub pending_fragments: RwLock<Vec<QueryFragmentId>>,
    pub job_status: RwLock<HashMap<i32, Sender<Vec<u8>>>>,
    executors: RwLock<Vec<ExecutorHandle>>,
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

pub struct ScheduleResult {
    pub query_id: i32,
    pub enqueue_time: SystemTime,
}

pub enum QueryResult {
    ArrowExec(FileScanConfig),
    HashBuildExec(HashBuildResult),
    ParquetExec(FileScanConfig),
}

pub struct IntermediateNode {}

impl Scheduler {
    pub async fn schedule_query(
        &self,
        physical_plan: Arc<dyn ExecutionPlan>,
        _query_info: QueryInfo,
        pipelined: bool,
    ) -> ScheduleResult {
        let query_id = QUERY_ID_GENERATOR.fetch_add(1, Ordering::SeqCst);
        let fragments = parse_into_fragments_wrapper(physical_plan, query_id, 1, pipelined).await;
        add_fragments_to_scheduler(fragments).await;
        ScheduleResult {
            query_id: query_id.try_into().unwrap(),
            enqueue_time: SystemTime::now(),
        }
    }

    pub async fn finish_fragment(
        &self,
        child_fragment_id: QueryFragmentId,
        fragment_result: QueryResult,
    ) {
        finish_fragment(child_fragment_id, fragment_result).await
    }

    pub fn query_job_status(&self, _query_id: i32) -> QueryStatus {
        unimplemented!()
    }

    pub fn query_execution_done(&self, _fragment_id: i32, _query_status: QueryStatus) {
        unimplemented!()
    }

    pub fn parse_physical_plan(&self, _physical_plan: &dyn ExecutionPlan) {}

    pub async fn register_executor(&self, port: i32) {
        self.executors.write().await.push(ExecutorHandle { port });
        debug_println!("Executor registered; port={port}");
    }

    pub async fn get_plan_from_queue(&self) -> Option<PhysicalPlanFragment> {
        crate::queue::get_plan_from_queue().await
    }
}

lazy_static! {
    pub static ref SCHEDULER_INSTANCE: Scheduler = Scheduler {
        all_fragments: RwLock::new(HashMap::new()),
        pending_fragments: RwLock::new(vec![]),
        job_status: RwLock::new(HashMap::<i32, Sender<Vec<u8>>>::new()),
        executors: RwLock::new(vec![]),
    };
}
