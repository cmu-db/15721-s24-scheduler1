use lib::executor_interface::executor_service_server::{ExecutorService, ExecutorServiceServer};
use lib::executor_interface::{ExecuteQueryArgs, ExecuteQueryRet};
use lib::scheduler_interface::scheduler_service_client::SchedulerServiceClient;

use tokio::runtime::Handle;
use tonic::{transport::Server, Code, Request, Response, Status};

use std::env;
use std::thread;
use std::time::Duration;

#[derive(Debug, Default)]
pub struct MyExecutor {}

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

async fn initialize(port: i32) {
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

    let request = tonic::Request::new(lib::scheduler_interface::RegisterExecutorArgs { port });
    let response = client.register_executor(request).await;
    println!("Registered with the scheduler at http://[::1]:{scheduler_service_port}");

    let addr = format!("[::1]:{port}").parse().unwrap();
    println!("Executor server listening on {addr}");

    let executor = MyExecutor::default();

    Server::builder()
        .add_service(ExecutorServiceServer::new(executor))
        .serve(addr)
        .await;
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();

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

