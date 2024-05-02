use crate::parser::{parse_into_fragments_wrapper, QueryFragment, QueryFragmentId};
use crate::queue::{abort_query, add_fragments_to_scheduler, finish_fragment};
use crate::scheduler_interface::*;
use datafusion::datasource::listing::PartitionedFile;
use datafusion::datasource::physical_plan::FileScanConfig;
use datafusion::physical_plan::ExecutionPlan;

extern crate lazy_static;
use lazy_static::lazy_static;
use tokio::sync::RwLock;

use std::collections::HashMap;

use std::sync::atomic::{AtomicU64, Ordering};
use std::time::SystemTime;

use std::sync::Arc;
use tokio::sync::mpsc::Sender;

/// Generator for query ids.
static QUERY_ID_GENERATOR: AtomicU64 = AtomicU64::new(0);

/// A database query scheduler.
///
/// Stores the metadata needed for query scheduling like the fragments that have been submitted to the scheduler for
/// execution.
#[derive(Debug)]
pub struct Scheduler {
    /// Map from query fragment id to fragment.
    pub all_fragments: RwLock<HashMap<QueryFragmentId, QueryFragment>>,

    /// Query fragments pending execution.
    pub pending_fragments: RwLock<Vec<QueryFragmentId>>,

    /// Map from query id to a [`Sender`] for sending the [`FileScanConfig`] for the query result.
    pub query_result_senders: RwLock<HashMap<u64, Sender<Vec<u8>>>>,

    pub intermediate_files: RwLock<HashMap<String, i32>>,
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

/// Information received at the time the query is scheduled for execution.
pub struct ScheduleResult {
    /// The query id.
    pub query_id: u64,
    /// The time the query was enqueued.
    pub enqueue_time: SystemTime,
}

/// The result of query execution.
pub enum QueryResult {
    ArrowExec(FileScanConfig),
    HashBuild,
    ParquetExec(FileScanConfig),
}

impl Scheduler {
    /// Schedule `physical_plan` for execution with `query_info`. `pipelined` indicates whether hash join execution
    /// plan nodes should be split into a build and probe phase.
    pub async fn schedule_query(
        &self,
        physical_plan: Arc<dyn ExecutionPlan>,
        query_info: QueryInfo,
        pipelined: bool,
    ) -> ScheduleResult {
        let query_id = QUERY_ID_GENERATOR.fetch_add(1, Ordering::SeqCst);
        let fragments = parse_into_fragments_wrapper(
            physical_plan,
            query_id,
            query_info.priority.into(),
            pipelined,
        )
        .await;
        add_fragments_to_scheduler(fragments).await;
        ScheduleResult {
            query_id: query_id.try_into().unwrap(),
            enqueue_time: SystemTime::now(),
        }
    }

    /// Marks the completion of the execution of the query fragment with
    /// `fragment_id` with result `fragment_result`.
    pub async fn finish_fragment(
        &self,
        child_fragment_id: QueryFragmentId,
        fragment_result: QueryResult,
    ) -> Vec<String> {
        finish_fragment(child_fragment_id, fragment_result).await
    }

    pub fn query_job_status(&self, _query_id: i32) -> QueryStatus {
        unimplemented!()
    }

    /// Marks the completion of the execution of `fragment_id` belonging to `query_id`.
    ///
    /// `is_root_fragment` indicates that the fragment is the root fragment of the query, implying the completion of
    /// the entire query. `file_scan_config` and `file_scan_config_bytes` contain information to retrieve the output
    /// of the query.
    pub async fn query_execution_done(
        &self,
        query_id: u64,
        fragment_id: u64,
        file_scan_config: Option<FileScanConfig>,
        file_scan_config_bytes: Vec<u8>,
        is_root_fragment: bool,
    ) -> Vec<String> {
        let mut to_delete = Vec::new();
        if let Some(file_scan_config) = &file_scan_config {
            to_delete = self
                .finish_fragment(
                    fragment_id.try_into().unwrap(),
                    QueryResult::ParquetExec(file_scan_config.clone()),
                )
                .await;
        } else {
            to_delete = self
                .finish_fragment(fragment_id.try_into().unwrap(), QueryResult::HashBuild)
                .await;
        }

        if is_root_fragment {
            assert!(file_scan_config.is_some());
            if let Some(tx) = self.query_result_senders.write().await.remove(&query_id) {
                tx.send(file_scan_config_bytes).await.unwrap();
            }
        }
        to_delete
    }

    pub fn parse_physical_plan(&self, _physical_plan: &dyn ExecutionPlan) {}

    pub async fn abort_query(&self, query_id: i32) {
        abort_query(query_id.try_into().unwrap()).await;
    }

    /// Get a query fragment from the scheduler for execution.
    pub async fn get_next_query_fragment(&self) -> Option<QueryFragment> {
        crate::queue::get_plan_from_queue().await
    }
}

lazy_static! {
    pub static ref SCHEDULER_INSTANCE: Scheduler = Scheduler {
        all_fragments: RwLock::new(HashMap::new()),
        pending_fragments: RwLock::new(vec![]),
        query_result_senders: RwLock::new(HashMap::<u64, Sender<Vec<u8>>>::new()),
        intermediate_files: RwLock::new(HashMap::<String, i32>::new()),
    };
}
