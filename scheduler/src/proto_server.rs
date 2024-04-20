use datafusion::parquet::file;
use prost::Message;
use tonic::{transport::Server, Code, Request, Response, Status};

use datafusion_proto::bytes::{physical_plan_from_bytes, physical_plan_to_bytes};

use datafusion::execution::context::{self, SessionContext};

use datafusion_proto::physical_plan::{AsExecutionPlan, DefaultPhysicalExtensionCodec, from_proto};
use datafusion_proto::protobuf::{self, FileScanExecConf};

use lib::scheduler::SCHEDULER_INSTANCE;
use lib::scheduler_interface::scheduler_service_server::SchedulerService;
use lib::scheduler_interface::scheduler_service_server::SchedulerServiceServer;
use lib::scheduler_interface::*;

use datafusion::prelude::CsvReadOptions;
use bytes::IntoBuf;

#[derive(Debug, Default)]
pub struct MyScheduler {}

#[tonic::async_trait]
impl SchedulerService for MyScheduler {
    async fn get_query(
        &self,
        request: Request<GetQueryArgs>,
    ) -> Result<Response<GetQueryRet>, Status> {
        // let scheduler = SCHEDULER_INSTANCE.lock().await;
        let plan = lib::queue::get_plan_from_queue().await;

        match plan {
            Some(p) => {
                let reply = GetQueryRet {
                    query_id: i32::try_from(p.query_id).unwrap(),
                    fragment_id: i32::try_from(p.fragment_id).unwrap(),
                    physical_plan: physical_plan_to_bytes(p.root.unwrap()).unwrap().to_vec(),
                    root: p.parent_fragments.is_empty(),
                };
                Ok(Response::new(reply))
            }
            None => {
                let reply = GetQueryRet {
                    query_id: -1,
                    fragment_id: -1,
                    physical_plan: vec![],
                    root: false,
                };
                Ok(Response::new(reply))
            }
        }
    }

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
        // let physical_plan_proto =
        //     protobuf::PhysicalPlanNode::try_from_physical_plan(physical_plan.clone(), &codec);

        // if physical_plan_proto.is_err() {
        //     let status = Status::new(Code::InvalidArgument, "Error converting to proto");
        //     return Err(status);
        // }

        // let scheduler = SCHEDULER_INSTANCE.lock().await;
        let query_id = lib::queue::schedule_query(physical_plan, metadata.unwrap(), true).await;

        let reply = ScheduleQueryRet { query_id };
        Ok(Response::new(reply))
    }

    async fn query_job_status(
        &self,
        request: Request<QueryJobStatusArgs>,
    ) -> Result<Response<QueryJobStatusRet>, Status> {
        let request_content = request.into_inner();
        let query_id = request_content.query_id;

        // let scheduler = SCHEDULER_INSTANCE.lock().await;
        // let query_status = lib::queue::query_job_status(query_id);
        let query_status = 1; // hardcode for now

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
        let file_scan_exec_conf = FileScanExecConf::decode(request_content.file_scan_config.into_buf()).unwrap();
        let file_scan_conf = from_proto::parse_protobuf_file_scan_config(&file_scan_exec_conf, &SessionContext::new()).unwrap();

        if query_status.is_err() {
            let status = Status::new(Code::InvalidArgument, "Query status not specified");
            return Err(status);
        }
        // let scheduler = SCHEDULER_INSTANCE.lock().await;

        lib::queue::finish_fragment(fragment_id.try_into().unwrap(), lib::queue::QueryResult::ParquetExec(file_scan_conf)).await;

        let reply = QueryExecutionDoneRet {};
        Ok(Response::new(reply))
    }

    async fn register_executor(
        &self,
        request: Request<RegisterExecutorArgs>,
    ) -> Result<Response<RegisterExecutorRet>, Status> {
        let request_content = request.into_inner();
        let port = request_content.port;

        let mut scheduler = SCHEDULER_INSTANCE.lock().await;
        scheduler.register_executor(port);

        let reply = RegisterExecutorRet {};
        Ok(Response::new(reply))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:50051".parse()?;
    let scheduler = MyScheduler::default();

    println!("Scheduler server listening on {addr}");

    Server::builder()
        .add_service(SchedulerServiceServer::new(scheduler))
        .serve(addr)
        .await?;

    Ok(())
}
