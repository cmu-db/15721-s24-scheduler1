use datafusion_proto::protobuf::FileScanExecConf;
use lib::integration::scan_from_parquet;
use lib::scheduler_interface::scheduler_service_client::SchedulerServiceClient;

use bytes::IntoBuf;
use datafusion::arrow::{array::RecordBatch, util::pretty};
use datafusion::physical_plan;
use datafusion::prelude::*;
use datafusion_proto::bytes::physical_plan_to_bytes;
use datafusion_proto::physical_plan::from_proto;
use futures::stream::TryStreamExt;
use lib::debug_println;
use lib::scheduler_interface::{QueryInfo, ScheduleQueryArgs};
use prost::Message;
use std::fs;
use std::io::{self, Write};

async fn register_bench_table(ctx: &SessionContext, table: &str) -> () {
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

async fn register_all_bench_tables(ctx: &SessionContext) -> () {
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

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let ctx = SessionContext::new();
    let wd = std::env::current_dir().unwrap();
    let wd_str = wd.to_str().unwrap();
    debug_println!(
        "intermediate files saved to {}/scheduler/src/example_data",
        wd_str
    );

    register_all_bench_tables(&ctx).await;

    // register the scheduler api server
    let scheduler_service_port = std::env::var("SCHEDULER_PORT").unwrap_or_else(|_error| {
        panic!("Scheduler port environment variable not set");
    });
    let uri = format!("http://[::1]:{scheduler_service_port}");
    let mut client = SchedulerServiceClient::connect(uri.clone())
        .await
        .unwrap_or_else(|error| {
            panic!("Unable to connect to the scheduler instance: {:?}", error);
        });
    debug_println!("bench connected to scheduler at {}", uri);

    loop {
        let mut query_num = String::new();
        let mut prio = String::new();
        println!("\nEnter a query below: \n");
        std::io::stdin()
            .read_line(&mut query_num)
            .expect("failed to read line");

        print!("Enter a priority: ");
        io::stdout().flush().unwrap();
        std::io::stdin()
            .read_line(&mut prio)
            .expect("failed to read line");

        query_num = query_num.trim().to_string();
        let priority: i32 = prio.trim_end().parse().unwrap_or_default();
        if &query_num == "quit" {
            break;
        }

        let sql = fs::read_to_string(&format!(
            "{}/scheduler/src/tpch-datafusion/queries/{}.sql",
            wd_str, query_num
        ))
        .expect("Should have been able to read the file");

        let logical_plan = match ctx.state().create_logical_plan(sql.as_str()).await {
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
                    FileScanExecConf::decode(response.file_scan_config.clone().into_buf()).unwrap();
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
                    let mut output_stream =
                        physical_plan::execute_stream(scan_from_parquet(file_scan_conf), context)
                            .unwrap();
                    let mut result: Vec<RecordBatch> = vec![];
                    while let Some(rec_batch) = output_stream.try_next().await.unwrap() {
                        result.push(rec_batch);
                    }
                    pretty::print_batches(&result).unwrap();
                }
            }

            Err(e) => {
                println!("query scheduling unsuccessful: {:?}", e);
            }
        }
    }

    Ok(())
}
