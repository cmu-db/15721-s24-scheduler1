use datafusion::datasource::physical_plan::FileScanConfig;
use datafusion_proto::bytes::physical_plan_from_bytes;
use datafusion_proto::protobuf::FileScanExecConf;
use lib::executor_interface::executor_service_server::ExecutorService;
use lib::executor_interface::{ExecuteQueryArgs, ExecuteQueryRet};
use lib::scheduler_interface::scheduler_service_client::SchedulerServiceClient;

use lib::scheduler_interface::{GetQueryArgs, GetQueryRet, QueryExecutionDoneArgs};
use tokio::runtime::Handle;
use tonic::{Code, Request, Response, Status};

use core::time;
use datafusion::prelude::*;
use lib::integration::{local_file_config, spill_records_to_disk};
use prost::Message;
use std::env;
use std::thread::sleep;
use tokio::fs;

use lib::debug_println;

#[derive(Debug, Default)]
pub struct MyExecutor {}
use datafusion::physical_plan;

#[tonic::async_trait]
impl ExecutorService for MyExecutor {
    async fn execute_query(
        &self,
        request: Request<ExecuteQueryArgs>,
    ) -> Result<Response<ExecuteQueryRet>, Status> {
        let _request_content = request.into_inner();

        let reply = ExecuteQueryRet {};
        Ok(Response::new(reply))
    }
}

async fn process_fragment(get_query_response: GetQueryRet, ctx: &SessionContext) -> FileScanConfig {
    let wd = env::current_dir().unwrap();
    let wd_str = wd.to_str().unwrap();

    let query_id = get_query_response.query_id;
    let fragment_id = get_query_response.fragment_id;
    let process_plan = physical_plan_from_bytes(&get_query_response.physical_plan, ctx).unwrap();

    let output_schema = process_plan.schema();
    let context = ctx.state().task_ctx();
    let output_stream = physical_plan::execute_stream(process_plan.clone(), context).unwrap();

    let intermediate_output = format!(
        "/{wd_str}/scheduler/src/example_data/query_{query_id}_fragment_{fragment_id}.parquet"
    );

    spill_records_to_disk(
        &intermediate_output,
        output_stream,
        output_schema.clone(),
        get_query_response.root,
    )
    .await
    .unwrap();

    local_file_config(output_schema, intermediate_output.as_str())
}

async fn initialize(port: i32, delete_intermediate: bool) {
    let scheduler_service_port = env::var("SCHEDULER_PORT").unwrap_or_else(|_error| {
        panic!("Scheduler port environment variable not set");
    });
    let uri = format!("http://[::1]:{scheduler_service_port}");
    let mut client = SchedulerServiceClient::connect(uri.clone())
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

                let interm_file = process_fragment(response.clone(), &ctx).await;
                let interm_proto = FileScanExecConf::try_from(&interm_file).unwrap();

                let finished_request = tonic::Request::new(QueryExecutionDoneArgs {
                    fragment_id: response.fragment_id,
                    status: 0,
                    file_scan_config: interm_proto.encode_to_vec(),
                    root: response.root,
                    query_id: response.query_id,
                });

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

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    debug_println!("{:?}", args);

    let num_workers: i32 = args[1].parse().unwrap();

    let delete_intermediate: bool = args[2].parse().unwrap_or_default();

    let mut handles = Vec::new();
    let base_port = 5555;

    let _handle = Handle::current;

    for i in 0..num_workers {
        handles.push(tokio::spawn(async move {
            initialize(base_port + i, delete_intermediate).await;
        }));
    }

    for handle in handles {
        let _ = handle.await;
    }

    Ok(())
}

#[cfg(test)]
mod tests {

    #[tokio::test]
    #[ignore] // add end to end later
    async fn end2end () -> Result<(), Box<dyn std::error::Error>> {
        std::process::Command::new("cargo run")
        .env("SCHEDULER_PORT", "50051")
        .arg("--bin")
        .arg("scheduler-api-server")
        .output()?;

        std::thread::sleep(std::time::Duration::from_millis(2000));

        Ok(())
    }
}
