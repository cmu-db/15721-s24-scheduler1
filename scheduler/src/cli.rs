use anyhow::{Context, Result};
use clap::Parser;

use datafusion::datasource::physical_plan::FileScanConfig;
use datafusion_proto::protobuf::FileScanExecConf;
use lib::executor_interface::executor_service_server::{ExecutorService, ExecutorServiceServer};
use lib::executor_interface::{ExecuteQueryArgs, ExecuteQueryRet};
use lib::scheduler_interface::scheduler_service_client::SchedulerServiceClient;

use lib::scheduler_interface::{GetQueryArgs, GetQueryRet, QueryExecutionDoneArgs, QueryInfo, ScheduleQueryArgs, ScheduleQueryRet};
use tokio::runtime::Handle;
use tonic::{transport::Server, Code, Request, Response, Status};

use core::time;
use datafusion::prelude::*;
use lib::integration::{local_file_config, scan_from_parquet, spill_records_to_disk};
use prost::Message;
use std::env;
use std::thread::{self, sleep};
use std::time::Duration;
use datafusion_proto::bytes::{physical_plan_from_bytes, physical_plan_to_bytes};
use datafusion_proto::physical_plan::{AsExecutionPlan, DefaultPhysicalExtensionCodec, from_proto};

/// Search for a pattern in a file and display the lines that contain it.

#[tokio::main]
async fn main() -> Result<()> {
    // let args = Cli::parse();

    let ctx = SessionContext::new();

    let wd = std::env::current_dir().unwrap();
    let wd_str = wd.to_str().unwrap();

    ctx.register_csv(
        "orders",
        &format!("{}/scheduler/src/example_data/orders.csv", wd_str),
        CsvReadOptions::new(),
    )
    .await.expect("Table Registration Error");
    ctx.register_csv(
        "prices",
        &format!("{}/scheduler/src/example_data/prices.csv", wd_str),
        CsvReadOptions::new(),
    )
    .await.expect("Table Registration Error");

    let scheduler_service_port = env::var("SCHEDULER_PORT").unwrap_or_else(|error| {
        panic!("Scheduler port environment variable not set");
    });
    let uri = format!("http://[::1]:{scheduler_service_port}");
    println!("Attempting to connect to the scheduler at {uri}");
    let mut client = SchedulerServiceClient::connect(uri)
        .await
        .unwrap_or_else(|error| {
            panic!("Unable to connect to the scheduler instance: {:?}", error);
        });
    println!("connected");

    let codec = DefaultPhysicalExtensionCodec {};
    loop {
        let mut sql = String::new();
        println!("Enter a query: ");
        std::io::stdin()
            .read_line(&mut sql)
            .expect("Failed to read line");
    
        sql = sql.trim().to_string();
        if &sql == "quit" {
            break;
        }

        let logical_plan = ctx.state().create_logical_plan(sql.as_str()).await.expect("query string to logical plan failed");
        let physical_plan = ctx.state().create_physical_plan(&logical_plan).await.expect("logical to physical plan failed");

        let get_request = tonic::Request::new(ScheduleQueryArgs {
            physical_plan: physical_plan_to_bytes(physical_plan.clone()).expect("Physical Plan Serialization Error").to_vec(),
            metadata: Some(QueryInfo::default()),
        });

        match client.schedule_query(get_request).await {
            Ok(response) => {
                let response = response.into_inner();
                // request should be a enum later where one variant is break
                println!("{}", response.query_id);
            }

            Err(e) => {
                println!("Query get unsuccessful: {:?}", e);
                sleep(time::Duration::from_millis(500));
            }
        }
    
    }


    Ok(())
}