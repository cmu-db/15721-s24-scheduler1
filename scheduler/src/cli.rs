use anyhow::{Context, Result};
use clap::Parser;

use datafusion::datasource::physical_plan::FileScanConfig;
use datafusion_proto::bytes::physical_plan_from_bytes;
use datafusion_proto::protobuf::FileScanExecConf;
use lib::executor_interface::executor_service_server::{ExecutorService, ExecutorServiceServer};
use lib::executor_interface::{ExecuteQueryArgs, ExecuteQueryRet};
use lib::scheduler_interface::scheduler_service_client::SchedulerServiceClient;

use lib::scheduler_interface::{GetQueryArgs, GetQueryRet, QueryExecutionDoneArgs, QueryInfo, CliScheduleQueryArgs, CliScheduleQueryRet};
use tokio::runtime::Handle;
use tonic::{transport::Server, Code, Request, Response, Status};

use core::time;
use datafusion::prelude::*;
use lib::integration::{local_file_config, scan_from_parquet, spill_records_to_disk};
use prost::Message;
use std::env;
use std::thread::{self, sleep};
use std::time::Duration;

/// Search for a pattern in a file and display the lines that contain it.
#[derive(Parser)]
struct Cli {
    /// The pattern to look for
    pattern: String,
    /// The path to the file to read
    path: std::path::PathBuf,
}

#[tokio::main]
async fn main() -> Result<()> {
    // let args = Cli::parse();

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

    let get_request = tonic::Request::new(CliScheduleQueryArgs {
        sql: "SELECT a.*, b.price, a.quantity * b.price as total 
                         FROM orders a inner join prices b 
                         ON a.item_id = b.item_id 
                         and a.item_id = 6 
                         ORDER by a.order_id".to_string(),
        metadata: Some(QueryInfo::default()),
    });
    match client.cli_schedule_query(get_request).await {
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


    Ok(())
}