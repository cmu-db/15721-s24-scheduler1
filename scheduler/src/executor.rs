use chronos::executor_interface::executor_service_server::ExecutorService;
use chronos::executor_interface::{ExecuteQueryArgs, ExecuteQueryRet};
use chronos::scheduler_interface::scheduler_client::SchedulerClient;
use datafusion::datasource::physical_plan::FileScanConfig;
use datafusion::execution::memory_pool::MemoryConsumer;
use datafusion::execution::TaskContext;
use datafusion::physical_expr::PhysicalExprRef;
use datafusion::physical_plan::coalesce_partitions::CoalescePartitionsExec;
use datafusion::physical_plan::joins::utils::JoinHashMap;
use datafusion::physical_plan::joins::JoinLeftData;
use datafusion::physical_plan::joins::{update_hash, HashBuildExec};
use datafusion_common::arrow::compute::concat_batches;

use datafusion_common::arrow::record_batch::RecordBatch;
use datafusion_common::DataFusionError;
use datafusion_proto::bytes::physical_plan_from_bytes;
use datafusion_proto::protobuf::FileScanExecConf;

use chronos::scheduler_interface::{
    GetQueryArgs, GetQueryRet, QueryExecutionDoneArgs, QueryStatus,
};
use futures::TryStreamExt;
use tokio::runtime::Handle;
use tokio::sync::RwLock;
use tonic::{Code, Request, Response, Status};

use ahash::RandomState;
use chronos::integration::{local_file_config, spill_records_to_disk};
use core::time;
use datafusion::physical_plan::ExecutionPlanProperties;
use datafusion::prelude::*;
use prost::Message;
use std::collections::HashMap;
use std::sync::Arc;
use std::thread::sleep;

use std::env;

use chronos::debug_println;

enum QueryResult {
    Config(FileScanConfig),
    Data(JoinLeftData),
}

/// An entity that executes query plans.
#[derive(Debug, Default)]
pub struct Executor {
    random_state: RandomState,
}
use datafusion::physical_plan::{self, ExecutionPlan};

#[tonic::async_trait]
impl ExecutorService for Executor {
    async fn execute_query(
        &self,
        request: Request<ExecuteQueryArgs>,
    ) -> Result<Response<ExecuteQueryRet>, Status> {
        let _request_content = request.into_inner();

        let reply = ExecuteQueryRet {};
        Ok(Response::new(reply))
    }
}

impl Executor {
    async fn process_fragment(
        &self,
        get_query_response: GetQueryRet,
        ctx: &SessionContext,
    ) -> QueryResult {
        let wd = env::current_dir().unwrap();
        let wd_str = wd.to_str().unwrap();

        let query_id = get_query_response.query_id;
        let fragment_id = get_query_response.fragment_id;
        let process_plan =
            physical_plan_from_bytes(&get_query_response.physical_plan, ctx).unwrap();
        let output_schema = process_plan.schema();
        let context = ctx.state().task_ctx();

        // If this plan requires us to build a hash table.
        if let Some(node) = process_plan.as_any().downcast_ref::<HashBuildExec>() {
            let input = node.input().clone();
            let on = node.on.clone();
            let join_data = self
                .build_hash_table(None, input, context.clone(), on)
                .await
                .expect("Failed to build a hash table");
            // self.hash_tables
            //     .write()
            //     .await
            //     .insert(fragment_id, hash_table);
        }
        let output_stream = physical_plan::execute_stream(process_plan, context).unwrap();

        let intermediate_output = format!(
            "{wd_str}/scheduler/src/example_data/query_{query_id}_fragment_{fragment_id}.parquet"
        );

        spill_records_to_disk(
            &intermediate_output,
            output_stream,
            output_schema.clone(),
            get_query_response.root,
        )
        .await
        .unwrap();
        QueryResult::Config(local_file_config(
            output_schema,
            intermediate_output.as_str(),
        ))
    }

    async fn build_hash_table(
        &self,
        partition: Option<usize>,
        node: Arc<dyn ExecutionPlan>,
        context: Arc<TaskContext>,
        on: Vec<PhysicalExprRef>,
    ) -> Result<JoinLeftData, DataFusionError> {
        let schema = node.schema();

        let (node_input, node_input_partition) = if let Some(partition) = partition {
            (node, partition)
        } else if node.output_partitioning().partition_count() != 1 {
            (Arc::new(CoalescePartitionsExec::new(node)) as _, 0)
        } else {
            (node, 0)
        };

        // Depending on partition argument load single partition or whole left side in memory
        let stream = node_input.execute(node_input_partition, context.clone())?;

        // This operation performs 2 steps at once:
        // 1. creates a [JoinHashMap] of all batches from the stream
        // 2. stores the batches in a vector.
        let initial = (Vec::new(), 0);

        let (batches, num_rows) = stream
            .try_fold(initial, |mut acc, batch| async {
                // Update rowcount
                acc.1 += batch.num_rows();
                // Push batch to output
                acc.0.push(batch);
                Ok(acc)
            })
            .await
            .unwrap();

        let mut hashmap = JoinHashMap::with_capacity(num_rows);
        let mut hashes_buffer = Vec::new();
        let mut offset = 0;

        // Updating hashmap starting from the last batch
        let batches_iter = batches.iter().rev();
        for batch in batches_iter.clone() {
            hashes_buffer.clear();
            hashes_buffer.resize(batch.num_rows(), 0);
            let _ = update_hash(
                &on,
                batch,
                &mut hashmap,
                offset,
                &self.random_state,
                &mut hashes_buffer,
                0,
                true,
            );
            offset += batch.num_rows();
        }
        let single_batch = concat_batches(&schema, batches_iter)?;
        let reservation =
            MemoryConsumer::new(format!("HashJoinProbe")).register(context.memory_pool());
        let data = JoinLeftData::new(
            hashmap,
            single_batch,
            node_input.output_partitioning().clone(),
        );

        Ok(data)
    }

    async fn initialize(&self, port: i32) {
        let scheduler_service_port = env::var("SCHEDULER_PORT").unwrap_or_else(|_error| {
            panic!("Scheduler port environment variable not set");
        });
        let uri = format!("http://[::1]:{scheduler_service_port}");
        let mut client = SchedulerClient::connect(uri.clone())
            .await
            .unwrap_or_else(|error| {
                panic!("Unable to connect to the scheduler instance: {:?}", error);
            });

        debug_println!(
            "executor at port {port} connected to the scheduler at {}",
            &uri
        );
        let ctx = SessionContext::new();

        loop {
            let get_request = tonic::Request::new(GetQueryArgs {});
            match client.get_query(get_request).await {
                Ok(response) => {
                    let response = response.into_inner();
                    if response.query_id < 0 {
                        sleep(time::Duration::from_millis(500));
                        continue;
                    }

                    let result = self.process_fragment(response.clone(), &ctx).await;

                    let mut finished_request: tonic::Request<QueryExecutionDoneArgs>;

                    match result {
                        QueryResult::Config(config) => {
                            let interm_proto = FileScanExecConf::try_from(&config).unwrap();
                            finished_request = tonic::Request::new(QueryExecutionDoneArgs {
                                fragment_id: response.fragment_id,
                                status: QueryStatus::Done.into(),
                                file_scan_config: interm_proto.encode_to_vec(),
                                root: response.root,
                                query_id: response.query_id,
                                join_data_ptr: 0,
                            });
                        }
                        QueryResult::Data(data) => {
                            finished_request = tonic::Request::new(QueryExecutionDoneArgs {
                                fragment_id: response.fragment_id,
                                status: QueryStatus::Done.into(),
                                file_scan_config: vec![],
                                root: response.root,
                                query_id: response.query_id,
                                join_data_ptr: Box::into_raw(Box::new(data)) as u64,
                            });
                        }
                    };

                    match client.query_execution_done(finished_request).await {
                        Err(e) => {
                            debug_println!("Finished reply unsuccessful: {:?}", e);
                            //client.kill_query_execution(); TODO
                        }
                        Ok(_finished_response) => {
                            debug_println!("reply for finishing query frag received");
                            debug_println!("response : {:?}", _finished_response);
                        }
                    }
                }

                Err(e) => match e.code() {
                    Code::Unavailable => {
                        debug_println!("get_query rpc unsuccessful: {:?}", e);
                        debug_println!("executor on port {port} is exiting");
                        break;
                    }
                    _ => {
                        debug_println!("unhandled status {:?}", e);
                        debug_println!("go implement handler, sleeping for 500ms...");
                        sleep(time::Duration::from_millis(500));
                    }
                },
            };
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    debug_println!("{:?}", args);

    let num_workers: i32 = args[1].parse().unwrap();

    let mut handles = Vec::new();
    let base_port = 5555;

    for i in 0..num_workers {
        handles.push(tokio::spawn(async move {
            let executor = Executor {
                random_state: RandomState::with_seeds(0, 0, 0, 0),
            };
            executor.initialize(base_port + i).await;
        }));
    }

    for handle in handles {
        let _ = handle.await;
    }

    Ok(())
}
