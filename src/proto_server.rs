use tonic::{transport::Server, Code, Request, Response, Status};

use datafusion_proto::bytes::physical_plan_from_bytes;

use datafusion::execution::context::SessionContext;

use datafusion_proto::physical_plan::{AsExecutionPlan, DefaultPhysicalExtensionCodec};
use datafusion_proto::protobuf;

use lib::scheduler::SCHEDULER_INSTANCE;
use lib::scheduler_interface::scheduler_service_server::SchedulerService;
use lib::scheduler_interface::scheduler_service_server::SchedulerServiceServer;
use lib::scheduler_interface::*;

#[derive(Debug, Default)]
pub struct MyScheduler {}

#[tonic::async_trait]
impl SchedulerService for MyScheduler {
    async fn schedule_query(
        &self,
        request: Request<ScheduleQueryArgs>,
    ) -> Result<Response<ScheduleQueryRet>, Status> {
        let request_content = request.into_inner();
        let physical_plan_bytes = request_content.physical_plan;

        // Deserialize physical plan
        let ctx = SessionContext::new();
        let physical_plan = physical_plan_from_bytes(&physical_plan_bytes, &ctx).unwrap();

        let metadata = request_content.metadata;

        if metadata.is_none() {
            let status = Status::new(Code::InvalidArgument, "Metadata not specified");
            return Err(status);
        }

        let codec = DefaultPhysicalExtensionCodec {};
        let physical_plan_proto =
            protobuf::PhysicalPlanNode::try_from_physical_plan(physical_plan.clone(), &codec);

        if physical_plan_proto.is_err() {
            let status = Status::new(Code::InvalidArgument, "Error converting to proto");
            return Err(status);
        }

        let query_id = SCHEDULER_INSTANCE.schedule_query(physical_plan, metadata.unwrap());

        let reply = ScheduleQueryRet { query_id };
        Ok(Response::new(reply))
    }

    async fn query_job_status(
        &self,
        request: Request<QueryJobStatusArgs>,
    ) -> Result<Response<QueryJobStatusRet>, Status> {
        let request_content = request.into_inner();
        let query_id = request_content.query_id;

        let query_status = SCHEDULER_INSTANCE.query_job_status(query_id);

        let reply = QueryJobStatusRet {
            query_status: query_status.into(),
        };
        Ok(Response::new(reply))
    }

    async fn query_execution_done(
        &self,
        request: Request<QueryExecutionDoneArgs>,
    ) -> Result<Response<QueryExecutionDoneRet>, Status> {
        let request_content = request.into_inner();
        let fragment_id = request_content.fragment_id;
        let query_status = QueryStatus::try_from(request_content.status);

        if query_status.is_err() {
            let status = Status::new(Code::InvalidArgument, "Query status not specified");
            return Err(status);
        }
        SCHEDULER_INSTANCE.query_execution_done(fragment_id, query_status.unwrap());

        let reply = QueryExecutionDoneRet {};
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
