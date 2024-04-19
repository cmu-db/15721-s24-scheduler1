use crate::executor_interface::executor_service_server::{ExecutorService, ExecutorServiceServer};
use crate::executor_interface::{ExecuteQueryArgs, ExecuteQueryRet};
use crate::scheduler_interface::scheduler_service_client::SchedulerServiceClient;
use datafusion::datasource::physical_plan::FileScanConfig;
use datafusion_proto::bytes::physical_plan_from_bytes;
use datafusion_proto::protobuf::FileScanExecConf;

use crate::scheduler_interface::GetQueryRet;
use tokio::runtime::Handle;
use tonic::{transport::Server, Code, Request, Response, Status};

use crate::integration::{local_file_config, scan_from_parquet, spill_records_to_disk};
use core::time;
use datafusion::prelude::*;
use prost::Message;
use std::env;
use std::thread::{self, sleep};
use std::time::Duration;

#[derive(Debug, Default)]
pub struct MyExecutor {}
use datafusion::physical_plan;

#[tonic::async_trait]
impl ExecutorService for MyExecutor {
    async fn execute_query(
        &self,
        request: Request<ExecuteQueryArgs>,
    ) -> Result<Response<ExecuteQueryRet>, Status> {
        let request_content = request.into_inner();

        let reply = ExecuteQueryRet {};
        Ok(Response::new(reply))
    }
}

async fn integration_process(
    get_query_response: GetQueryRet,
    ctx: &SessionContext,
) -> FileScanConfig {
    let query_id = get_query_response.query_id;
    let fragment_id = get_query_response.fragment_id;
    let process_plan = physical_plan_from_bytes(&get_query_response.physical_plan, &ctx).unwrap();
    let output_schema = process_plan.schema();
    let intermediate_output = format!("query {query_id} fragment {fragment_id}.parquet");

    let context = ctx.state().task_ctx();
    let mut output_stream = physical_plan::execute_stream(process_plan, context).unwrap();

    spill_records_to_disk(
        &intermediate_output,
        output_stream,
        output_schema.clone(),
        fragment_id == 0,
    )
    .await
    .unwrap();
    local_file_config(output_schema, intermediate_output.as_str())
}

async fn initialize(port: i32) {
    let ctx = SessionContext::new();
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

    // let request = tonic::Request::new(crate::scheduler_interface::RegisterExecutorArgs { port });
    // let response = client.register_executor(request).await;
    // println!("Registered with the scheduler at http://[::1]:{scheduler_service_port}");

    loop {
        let get_request = tonic::Request::new(crate::scheduler_interface::GetQueryArgs {});
        match client.get_query(get_request).await {
            Ok(response) => {
                let response = response.into_inner();
                // request should be a enum later where one variant is break
                let interm_file = integration_process(response.clone(), &ctx).await;
                let interm_proto = FileScanExecConf::try_from(&interm_file).unwrap();

                let finished_request =
                    tonic::Request::new(crate::scheduler_interface::QueryExecutionDoneArgs {
                        fragment_id: response.fragment_id,
                        status: 0,
                        file_scan_config: interm_proto.encode_to_vec(),
                    });

                match client.query_execution_done(finished_request).await {
                    Err(e) => println!("Finished reply unsuccessful: {:?}", e),
                    Ok(_finished_response) => (),
                }
            }

            Err(e) => {
                println!("Query get unsuccessful: {:?}", e);
                sleep(time::Duration::from_millis(100));
            }
        };
    }

    // let addr = format!("[::1]:{port}").parse().unwrap();
    // println!("Executor server listening on {addr}");
    // let executor = MyExecutor::default();

    // Server::builder()
    //     .add_service(ExecutorServiceServer::new(executor))
    //     .serve(addr)
    //     .await;
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);

    let num_workers: i32 = args[1].parse().unwrap();

    let mut handles = Vec::new();
    let base_port = 5555;

    let handle = Handle::current;

    for i in 0..num_workers {
        handles.push(tokio::spawn(async move {
            initialize(base_port + i).await;
        }));
    }

    for handle in handles {
        let _ = handle.await;
    }

    Ok(())
}
