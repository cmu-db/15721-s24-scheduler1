use chronos::scheduler_interface::scheduler_client::SchedulerClient;
use chronos::utils::scan_from_parquet;
use datafusion_proto::protobuf::FileScanExecConf;

use bytes::IntoBuf;

use chronos::debug_println;
use chronos::scheduler_interface::{QueryInfo, ScheduleQueryArgs};
use datafusion::physical_plan;
use datafusion::prelude::*;
use datafusion_proto::bytes::physical_plan_to_bytes;
use datafusion_proto::physical_plan::from_proto;
use futures::stream::TryStreamExt;
use prost::Message;

use std::time::Instant;
use std::{env, fs};
use tokio::runtime;

async fn register_bench_table(ctx: &SessionContext, table: &str) {
    let wd = std::env::current_dir().unwrap();
    let wd_str = wd.to_str().unwrap();
    ctx.register_parquet(
        table,
        &format!("{}/scheduler/src/tpch-datafusion/{}", wd_str, table),
        ParquetReadOptions::default(),
    )
    .await
    .expect("Table Registration Error");
}

async fn register_all_bench_tables(ctx: &SessionContext) {
    register_bench_table(ctx, "answers").await;
    register_bench_table(ctx, "customer").await;
    register_bench_table(ctx, "lineitem").await;
    register_bench_table(ctx, "nation").await;
    register_bench_table(ctx, "orders").await;
    register_bench_table(ctx, "part").await;
    register_bench_table(ctx, "partsupp").await;
    register_bench_table(ctx, "region").await;
    register_bench_table(ctx, "supplier").await;
}

async fn run_and_time_bench(query_num: u32, num_runs: u32) -> f64 {
    let ctx = SessionContext::new();
    let wd = std::env::current_dir().unwrap();
    let wd_str = wd.to_str().unwrap();
    register_all_bench_tables(&ctx).await;

    // register the scheduler api server
    let scheduler_service_port = std::env::var("SCHEDULER_PORT").unwrap_or_else(|_error| {
        panic!("Scheduler port environment variable not set");
    });
    let uri = format!("http://[::1]:{scheduler_service_port}");
    let mut client: SchedulerClient<tonic::transport::Channel> =
        SchedulerClient::connect(uri.clone())
            .await
            .unwrap_or_else(|error| {
                panic!("Unable to connect to the scheduler instance: {:?}", error);
            });
    debug_println!("bench connected to scheduler at {}", uri);

    let mut millis = vec![];

    let sql: Vec<String> = fs::read_to_string(format!(
        "{}/scheduler/src/tpch-datafusion/queries/q{}.sql",
        wd_str, query_num
    ))
    .expect("Should have been able to read the file")
    .split(';')
    .map(|s| s.trim())
    .filter(|s| !s.is_empty())
    .map(|s| s.to_string())
    .collect();

    // run benchmark
    for i in 0..num_runs {
        let start = Instant::now();

        for sql_statement in sql.clone() {
            let logical_plan = match ctx.state().create_logical_plan(&sql_statement).await {
                Ok(plan) => plan,
                Err(e) => {
                    println!("error: {:?}", e);
                    continue;
                }
            };

            let physical_plan = match ctx.state().create_physical_plan(&logical_plan).await {
                Ok(plan) => plan,
                Err(e) => {
                    println!("error: {:?}", e);
                    continue;
                }
            };

            let physical_plan_bytes = match physical_plan_to_bytes(physical_plan.clone()) {
                Ok(plan_bytes) => plan_bytes,
                Err(e) => {
                    println!("error: {:?}", e);
                    continue;
                }
            };

            let get_request = tonic::Request::new(ScheduleQueryArgs {
                physical_plan: physical_plan_bytes.to_vec(),
                metadata: Some(QueryInfo {
                    priority: 0,
                    cost: 0, // from optd
                }),
            });

            match client.schedule_query(get_request).await {
                Ok(response) => {
                    let response = response.into_inner();

                    let file_scan_exec_conf =
                        FileScanExecConf::decode(response.file_scan_config.clone().into_buf())
                            .unwrap();
                    let file_scan_conf = from_proto::parse_protobuf_file_scan_config(
                        &file_scan_exec_conf,
                        &SessionContext::new(),
                    )
                    .unwrap();

                    if file_scan_conf.file_groups.is_empty() {
                        println!("No output for the submitted query, continuing...");
                    } else {
                        println!("Printing results for query id {}", query_num);
                        let context = ctx.state().task_ctx();
                        let mut output_stream = physical_plan::execute_stream(
                            scan_from_parquet(file_scan_conf),
                            context,
                        )
                        .unwrap();
                        while let Some(_rec_batch) = output_stream.try_next().await.unwrap() {}
                    }
                }

                Err(e) => {
                    println!("query scheduling unsuccessful: {:?}", e);
                }
            }
        }

        let elapsed = start.elapsed(); //.as_secs_f64() * 1000.0;
        let ms: f64 = elapsed.as_secs_f64() * 1000.0;
        millis.push(ms);
        println!("Query {query_num} iteration {i} took {ms:.1} ms");
    }

    let avg = millis.iter().sum::<f64>() / millis.len() as f64;
    println!("Query {query_num} avg time: {avg:.2} ms");
    avg
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    debug_println!("{:?}", args);

    let parallel: bool = args[1].parse().unwrap();

    let rt = runtime::Builder::new_multi_thread()
        .thread_stack_size(30 * 1024 * 1024)
        .enable_all()
        .build()
        .unwrap();

    let mut results: Vec<(u32, f64)> = vec![];
    let mut handles = Vec::new();

    if parallel {
        for query_num in 1..23 {
            handles.push(rt.spawn(run_and_time_bench(query_num, 1)));
        }
        let mut i = 0;
        for handle in handles {
            i += 1;
            let x = handle.await.unwrap();
            results.push((i, x));
        }
    } else {
        for query_num in 1..23 {
            let handle = rt.spawn(run_and_time_bench(query_num, 1));
            let x = handle.await.unwrap();
            results.push((query_num, x));
        }
    }

    for res in results {
        println!("{:?}", res.1)
    }
    Ok(())
}
