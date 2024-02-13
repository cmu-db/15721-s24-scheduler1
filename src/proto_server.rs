use tonic::{transport::Server, Request, Response, Status};

use scheduler::scheduler_server::{Scheduler, SchedulerServer};

pub mod scheduler {
    tonic::include_proto!("scheduler");
}

#[derive(Debug,Default)]
pub struct MyScheduler {}

#[tonic::async_trait]
impl Scheduler for MyScheduler {
    async fn schedule_query(
        &self,
        request: Request<scheduler::ScheduleQueryArgs>
    ) -> Result<Response<scheduler::ScheduleQueryRet>, Status> {
        unimplemented!()
    }
    async fn query_job_status(
        &self,
        request: Request<scheduler::QueryJobStatusArgs>
    ) -> Result<Response<scheduler::QueryJobStatusRet>, Status> {
        unimplemented!()
    }
    async fn query_execution_done(
        &self,
        request: Request<scheduler::QueryExecutionDoneArgs>
    ) -> Result<Response<scheduler::QueryExecutionDoneRet>, Status> {
        unimplemented!()
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:50051".parse()?;
    let scheduler = MyScheduler::default();

    Server::builder()
        .add_service(SchedulerServer::new(scheduler))
        .serve(addr)
        .await?;

    Ok(())
}
