use tonic::{transport::Server, Request, Response, Status, Code};

use scheduler_interface::scheduler_service_server::SchedulerService;
use scheduler_interface::scheduler_service_server::SchedulerServiceServer;

use substrait::proto as substrait;

pub mod scheduler_interface {
    tonic::include_proto!("scheduler_interface");
}

use scheduler_interface::*;

#[derive(Debug,Default)]
pub struct MyScheduler {}

mod scheduler;

#[tonic::async_trait]
impl SchedulerService for MyScheduler {
    async fn schedule_query(
        &self,
        request: Request<ScheduleQueryArgs>
    ) -> Result<Response<ScheduleQueryRet>, Status> {
        let request_content = request.into_inner();
        let physical_plan = request_content.physical_plan;
        let metadata = request_content.metadata;

        if metadata.is_none() {
            let status =
                Status::new(Code::InvalidArgument, "Metadata not specified");
            return Err(status);
        }

        if physical_plan.is_none() {
            let status =
                Status::new(Code::InvalidArgument, "Physical plan not specified");
            return Err(status);
        }

        let query_id = scheduler::SCHEDULER_INSTANCE.schedule_query(physical_plan.unwrap(), metadata.unwrap());

        let reply = ScheduleQueryRet {
            query_id
        };
        Ok(Response::new(reply))
    }

    async fn query_job_status(
        &self,
        request: Request<QueryJobStatusArgs>
    ) -> Result<Response<QueryJobStatusRet>, Status> {
        let request_content = request.into_inner();
        let query_id = request_content.query_id;

        let query_status =
            scheduler::SCHEDULER_INSTANCE.query_job_status(query_id);

        let reply = QueryJobStatusRet {
            query_status: query_status.into()
        };
        Ok(Response::new(reply))
    }

    async fn query_execution_done(
        &self,
        request: Request<QueryExecutionDoneArgs>
    ) -> Result<Response<QueryExecutionDoneRet>, Status> {
        let request_content = request.into_inner();
        let fragment_id = request_content.fragment_id;
        let query_status = QueryStatus::try_from(request_content.status);

        if query_status.is_err() {
            let status =
                Status::new(Code::InvalidArgument, "Query status not specified");
            return Err(status);
        }
        scheduler::SCHEDULER_INSTANCE.query_execution_done(
            fragment_id, query_status.unwrap());

        let reply = QueryExecutionDoneRet { };
        Ok(Response::new(reply))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:50051".parse()?;
    let scheduler = MyScheduler::default();

    Server::builder()
        .add_service(SchedulerServiceServer::new(scheduler))
        .serve(addr)
        .await?;

    Ok(())
}
