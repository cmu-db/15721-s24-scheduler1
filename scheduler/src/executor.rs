use async_recursion::async_recursion;
use chronos::scheduler_interface::scheduler_client::SchedulerClient;
use datafusion::datasource::physical_plan::FileScanConfig;
use datafusion::execution::memory_pool::MemoryConsumer;
use datafusion::execution::TaskContext;
use datafusion::physical_expr::PhysicalExprRef;
use datafusion::physical_plan::coalesce_partitions::CoalescePartitionsExec;
use datafusion::physical_plan::joins::utils::JoinHashMap;
use datafusion::physical_plan::joins::{update_hash, HashBuildExec};
use datafusion::physical_plan::joins::{HashProbeExec, JoinLeftData};
use datafusion_common::arrow::compute::concat_batches;

use datafusion_common::DataFusionError;
use datafusion_proto::bytes::physical_plan_from_bytes;
use datafusion_proto::protobuf::FileScanExecConf;

use chronos::scheduler_interface::{
    GetQueryArgs, GetQueryRet, QueryExecutionDoneArgs, QueryStatus,
};
use futures::TryStreamExt;
use tokio::sync::RwLock;
use tonic::transport::Channel;
use tonic::Code;

use ahash::RandomState;
use chronos::utils::{local_file_config, spill_records_to_disk};
use core::time;
use datafusion::physical_plan::ExecutionPlanProperties;
use datafusion::physical_plan::{self, ExecutionPlan};
use datafusion::prelude::*;

// use lib::integration::{local_file_config, local_filegroup_config, spill_records_to_disk};
use chronos::utils::local_filegroup_config;
use prost::Message;
use std::collections::HashMap;
use std::env;
use std::sync::Arc;
use std::thread::sleep;

use chronos::debug_println;

enum QueryResult {
    Config(FileScanConfig),
    HashTable,
}

/// An entity that executes query plans.
#[derive(Debug, Default)]
pub struct Executor {
    random_state: RandomState,
    generated_hash_tables: Arc<RwLock<HashMap<u64, JoinLeftData>>>,
}

impl Executor {
    async fn process_fragment(
        &self,
        get_query_response: GetQueryRet,
        ctx: &SessionContext,
    ) -> QueryResult {
        let wd = env::current_dir().unwrap();
        let wd_str = wd.to_str().unwrap();

        let query_details = get_query_response.query_details.unwrap();

        let query_id = query_details.query_id;
        let fragment_id = query_details.fragment_id;
        let intermediate_output = format!(
            "/{wd_str}/scheduler/src/example_data/query_{query_id}/fragment_{fragment_id}.parquet"
        );
        let mut process_plan = physical_plan_from_bytes(&query_details.physical_plan, ctx).unwrap();
        let output_schema = process_plan.schema();

        if get_query_response.aborted {
            return QueryResult::Config(local_file_config(output_schema, ""));
        }
        let context = ctx.state().task_ctx();

        // If we need to add a precomputed hash table to a hash probe exec node
        for hash_build_info in query_details.hash_build_data {
            let path_from_parent = hash_build_info.path_from_parent;
            let build_fragment_id = hash_build_info.build_fragment_id;
            debug_println!(
                "Need to add precomputed hash table computed in fragment {build_fragment_id}"
            );
            let join_data = self
                .generated_hash_tables
                .read()
                .await
                .get(&build_fragment_id)
                .expect("Unable to find the built hash table")
                .clone();
            process_plan = self
                .add_hash_table_to_hash_probe(join_data, process_plan.clone(), &path_from_parent)
                .await;
            debug_println!(
                "Successfully added precomputed hash table computed in fragment {build_fragment_id}"
            );
        }

        // If this plan requires us to build a hash table.
        if let Some(node) = process_plan.as_any().downcast_ref::<HashBuildExec>() {
            debug_println!("Need to build a hash table for fragment {fragment_id}");
            let input = node.input().clone();
            if input.as_any().downcast_ref::<HashProbeExec>().is_some() {
                debug_println!(
                    "Input for the hash table is a HashProbeExec for fragment {fragment_id}"
                );
            }
            let on = node.on.clone();
            let join_data = self
                .build_hash_table(None, input, context.clone(), on)
                .await
                .unwrap_or_else(|_| panic!("Failed to build a hash table: {:#?} {:#?}",
                    node.schema(),
                    node.on()));
            self.generated_hash_tables
                .write()
                .await
                .insert(fragment_id, join_data);
            debug_println!("Generated hash table for fragment {fragment_id}");
            return QueryResult::HashTable;
        }

        if get_query_response.aborted {
            return QueryResult::Config(local_file_config(output_schema, ""));
        }

        let context = ctx.state().task_ctx();

        let output_stream = physical_plan::execute_stream(process_plan, context).unwrap();

        let fg = spill_records_to_disk(
            &intermediate_output,
            output_stream,
            output_schema.clone(),
            100000,
            4,
            get_query_response.root,
        )
        .await
        .unwrap();

        QueryResult::Config(local_filegroup_config(output_schema, fg))
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
            .await?;

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
        let _reservation =
            MemoryConsumer::new("HashJoinProbe".to_string()).register(context.memory_pool());
        let data = JoinLeftData::new(
            hashmap,
            single_batch,
            node_input.output_partitioning().clone(),
        );

        Ok(data)
    }

    #[async_recursion]
    async fn add_hash_table_to_hash_probe(
        &self,
        join_data: JoinLeftData,
        plan: Arc<dyn ExecutionPlan>,
        path_from_parent: &[u32],
    ) -> Arc<dyn ExecutionPlan> {
        // Found the HashProbeExec node (hopefully). Let's populate it with the join data we have pre-computed.
        if path_from_parent.is_empty() {
            if let Some(hash_probe) = plan.as_any().downcast_ref::<HashProbeExec>() {
                let mut new_node = HashProbeExec::try_new(
                    hash_probe.left.clone(),
                    hash_probe.right.clone(),
                    hash_probe.on.clone(),
                    hash_probe.filter.clone(),
                    &hash_probe.join_type,
                    hash_probe.projection.clone(),
                    *hash_probe.partition_mode(),
                    hash_probe.null_equals_null(),
                    None,
                )
                .unwrap();
                new_node.set_join_data(join_data);
                let arc_node: Arc<dyn ExecutionPlan> = Arc::new(new_node);
                return arc_node;
            } else {
                panic!(
                    "Expected to reach a hash probe, instead reached: {:#?}",
                    plan
                );
            }
        }

        let mut new_children = Vec::new();
        let children = plan.children();

        for (child_num, child) in (0_u32..).zip(children.into_iter()) {
            if child_num != path_from_parent[0] {
                new_children.push(child);
                continue;
            }
            new_children.push(
                self.add_hash_table_to_hash_probe(join_data.clone(), child, &path_from_parent[1..])
                    .await,
            );
        }
        plan.with_new_children(new_children).unwrap()
    }

    /// Initialize the executor with `worker_id` by connecting to the scheduler. `delete_intermediate` indicates
    /// whether intermediate files should be deleted.
    async fn initialize(&self, worker_id: i32, delete_intermediate: bool) {
        let scheduler_service_port = env::var("SCHEDULER_PORT").unwrap_or_else(|_error| {
            panic!("Scheduler port environment variable not set");
        });
        let uri = format!("http://[::1]:{scheduler_service_port}");
        let client = SchedulerClient::connect(uri.clone())
            .await
            .unwrap_or_else(|error| {
                panic!("Unable to connect to the scheduler instance: {:?}", error);
            });

        debug_println!(
            "executor {worker_id} connected to the scheduler at {}",
            &uri
        );

        self.executor_loop(client, delete_intermediate, worker_id)
            .await;
    }

    /// Run the executor loop for the executor to poll the `client` for new query fragments to execute.
    ///
    /// `delete_intermediate` indicates whether intermediate files should be deleted.
    async fn executor_loop(
        &self,
        mut client: SchedulerClient<Channel>,
        delete_intermediate: bool,
        worker_id: i32,
    ) {
        let ctx = SessionContext::new();
        loop {
            let get_request = tonic::Request::new(GetQueryArgs {});
            match client.get_query(get_request).await {
                Ok(response) => {
                    let response = response.into_inner();
                    if response.query_details.is_none() {
                        sleep(time::Duration::from_millis(500));
                        continue;
                    }

                    let query_details = response.query_details.clone().unwrap();
                    let result = self.process_fragment(response.clone(), &ctx).await;

                    let finished_request: tonic::Request<QueryExecutionDoneArgs>;

                    match result {
                        QueryResult::Config(config) => {
                            let interm_proto = FileScanExecConf::try_from(&config).unwrap();
                            finished_request = tonic::Request::new(QueryExecutionDoneArgs {
                                fragment_id: query_details.fragment_id,
                                status: QueryStatus::Done.into(),
                                file_scan_config: interm_proto.encode_to_vec(),
                                root: response.root,
                                query_id: query_details.query_id,
                                generated_hash_table: false,
                            });
                        }
                        QueryResult::HashTable => {
                            finished_request = tonic::Request::new(QueryExecutionDoneArgs {
                                fragment_id: query_details.fragment_id,
                                status: QueryStatus::Done.into(),
                                file_scan_config: vec![],
                                root: response.root,
                                query_id: query_details.query_id,
                                generated_hash_table: true,
                            });
                        }
                    };

                    match client.query_execution_done(finished_request).await {
                        Err(e) => {
                            debug_println!("Finished reply unsuccessful: {:?}", e);
                            //client.kill_query_execution(); TODO
                        }
                        Ok(finished_response) => {
                            debug_println!("reply for finishing query frag received");
                            debug_println!("response : {:?}", finished_response);
                            if delete_intermediate {
                                let mut response = finished_response.into_inner();

                                for file in &mut response.intermediate_files {
                                    file.insert(0, '/');
                                }

                                let handles = response
                                    .intermediate_files
                                    .into_iter()
                                    .map(tokio::fs::remove_file)
                                    .map(tokio::spawn)
                                    .collect::<Vec<_>>();

                                let _results = futures::future::join_all(handles).await;
                            }
                        }
                    }
                }

                Err(e) => match e.code() {
                    Code::Unavailable => {
                        debug_println!("get_query rpc unsuccessful: {:?}", e);
                        debug_println!("executor {worker_id} is exiting");
                        break;
                    }
                    _ => {
                        debug_println!("unhandled status {:?}", e);
                        debug_println!("go implement handler, sleeping for 500ms...");
                        sleep(time::Duration::from_millis(500));
                    }
                },
            }
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    debug_println!("{:?}", args);

    let num_workers: i32 = args[1].parse().unwrap();

    let delete_intermediate: bool = if args.len() > 2 {
        args[2].parse().unwrap_or_default()
    } else {
        false
    };

    let mut handles = Vec::new();

    let generated_hash_tables = Arc::new(RwLock::new(HashMap::new()));

    let rt = tokio::runtime::Builder::new_multi_thread()
        .thread_stack_size(10 * 1024 * 1024)
        .enable_all()
        .build()
        .unwrap();
    for i in 0..num_workers {
        let generated_hash_tables = generated_hash_tables.clone();
        handles.push(rt.spawn(async move {
            let executor = Executor {
                random_state: RandomState::with_seeds(0, 0, 0, 0),
                generated_hash_tables,
            };
            executor.initialize(i, delete_intermediate).await;
        }));
    }
    // for i in 0..num_workers {
    //     let generated_hash_tables = generated_hash_tables.clone();
    //     handles.push(tokio::spawn(async move {
    //         let executor = Executor {
    //             random_state: RandomState::with_seeds(0, 0, 0, 0),
    //             generated_hash_tables,
    //         };
    //         executor.initialize(i, delete_intermediate).await;
    //     }));
    // }

    for handle in handles {
        let _ = handle.await;
    }

    Ok(())
}

#[cfg(test)]
mod tests {

    #[tokio::test]
    #[ignore] // add end to end later
    async fn end2end() -> Result<(), Box<dyn std::error::Error>> {
        std::process::Command::new("cargo run")
            .env("SCHEDULER_PORT", "50051")
            .arg("--bin")
            .arg("scheduler-service")
            .output()?;

        std::thread::sleep(std::time::Duration::from_millis(2000));

        Ok(())
    }
}
