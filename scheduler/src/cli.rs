use chronos::integration::scan_from_parquet;
use chronos::scheduler_interface::scheduler_service_client::SchedulerServiceClient;
use datafusion_proto::protobuf::FileScanExecConf;

use bytes::IntoBuf;
use chronos::debug_println;
use chronos::scheduler_interface::{QueryInfo, ScheduleQueryArgs};
use datafusion::arrow::{array::RecordBatch, util::pretty};
use datafusion::physical_plan;
use datafusion::prelude::*;
use datafusion_proto::bytes::physical_plan_to_bytes;
use datafusion_proto::physical_plan::from_proto;
use futures::stream::TryStreamExt;
use prost::Message;
use std::io::{self, Write};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let ctx = SessionContext::new();
    let wd = std::env::current_dir().unwrap();
    let wd_str = wd.to_str().unwrap();
    debug_println!(
        "intermediate files saved to {}/scheduler/src/example_data",
        wd_str
    );

    // let exec_results = Mutex::new(CLIExecutionResult::default());

    // register tables here
    ctx.register_csv(
        "orders",
        &format!("{}/scheduler/src/example_data/orders.csv", wd_str),
        CsvReadOptions::new(),
    )
    .await
    .expect("Table Registration Error");
    ctx.register_csv(
        "prices",
        &format!("{}/scheduler/src/example_data/prices.csv", wd_str),
        CsvReadOptions::new(),
    )
    .await
    .expect("Table Registration Error");

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
    debug_println!("cli connected to scheduler at {}", uri);

    loop {
        let mut sql = String::new();
        let mut prio = String::new();
        println!("\nEnter a query below: \n");
        std::io::stdin()
            .read_line(&mut sql)
            .expect("failed to read line");

        print!("Enter a priority: ");
        io::stdout().flush().unwrap();
        std::io::stdin()
            .read_line(&mut prio)
            .expect("failed to read line");

        sql = sql.trim().to_string();
        let priority: i32 = prio.trim_end().parse().unwrap_or_default();
        if &sql == "quit" {
            break;
        }

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
                priority,
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
                    println!("Printing results for query id {}", response.query_id);
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
