use prost::Message;
use tonic::{transport::Server, Code, Request, Response, Status};

use datafusion_proto::bytes::{physical_plan_from_bytes, physical_plan_to_bytes};

use datafusion::execution::context::SessionContext;

use datafusion_proto::physical_plan::from_proto;
use datafusion_proto::protobuf::FileScanExecConf;

use chronos::scheduler::SCHEDULER_INSTANCE;
use chronos::scheduler_interface::scheduler_server::Scheduler;
use chronos::scheduler_interface::scheduler_server::SchedulerServer;
use chronos::scheduler_interface::*;

use bytes::IntoBuf;
use tokio::sync::mpsc;

use std::time::UNIX_EPOCH;

#[derive(Debug, Default)]
pub struct MyScheduler {}

#[tonic::async_trait]
impl Scheduler for MyScheduler {
    async fn get_query(
        &self,
        _request: Request<GetQueryArgs>,
    ) -> Result<Response<GetQueryRet>, Status> {
        let plan = chronos::scheduler::SCHEDULER_INSTANCE
            .get_plan_from_queue()
            .await;

        match plan {
            Some(p) => {
                let physical_plan = physical_plan_to_bytes(p.root.unwrap());

                match physical_plan {
                    Ok(p_bytes) => {
                        let reply = GetQueryRet {
                            query_id: i32::try_from(p.query_id).unwrap(),
                            fragment_id: i32::try_from(p.fragment_id).unwrap(),
                            physical_plan: p_bytes.to_vec(),
                            root: p.parent_fragments.is_empty(),
                        };
                        Ok(Response::new(reply))
                    }
                    Err(e) => {
                        println!("{:?}", e);
                        // chronos::queue::kill_query(p.query_id); TODO
                        let reply = GetQueryRet {
                            query_id: -1,
                            fragment_id: -1,
                            physical_plan: vec![],
                            root: true, // setting this to true frees the CLI on the tokio channel, will do for now
                        };
                        Ok(Response::new(reply))
                    }
                }
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

        let sched_info = chronos::scheduler::SCHEDULER_INSTANCE
            .schedule_query(physical_plan, metadata.unwrap(), false)
            .await;

        let (tx, mut rx) = mpsc::channel::<Vec<u8>>(1);

        {
            SCHEDULER_INSTANCE
                .job_status
                .write()
                .await
                .insert(sched_info.query_id, tx);
        }

        let reply = ScheduleQueryRet {
            query_id: sched_info.query_id,
            file_scan_config: rx.recv().await.unwrap_or_default(),
            enqueue_time: sched_info.enqueue_time.duration_since(UNIX_EPOCH).unwrap().as_millis().try_into().unwrap()
        };
        Ok(Response::new(reply))
    }

    async fn query_job_status(
        &self,
        request: Request<QueryJobStatusArgs>,
    ) -> Result<Response<QueryJobStatusRet>, Status> {
        let request_content = request.into_inner();
        let _query_id = request_content.query_id;
        let query_status = 1; // hardcode for now

        let reply = QueryJobStatusRet { query_status };
        Ok(Response::new(reply))
    }

    async fn query_execution_done(
        &self,
        request: Request<QueryExecutionDoneArgs>,
    ) -> Result<Response<QueryExecutionDoneRet>, Status> {
        let request_content = request.into_inner();
        let fragment_id = request_content.fragment_id;
        let query_status = QueryStatus::try_from(request_content.status);
        let file_scan_exec_conf =
            FileScanExecConf::decode(request_content.file_scan_config.clone().into_buf()).unwrap();
        let file_scan_conf = from_proto::parse_protobuf_file_scan_config(
            &file_scan_exec_conf,
            &SessionContext::new(),
        )
        .unwrap();

        if query_status.is_err() {
            let status = Status::new(Code::InvalidArgument, "Query status not specified");
            return Err(status);
        }
        // let scheduler = SCHEDULER_INSTANCE.lock().await;
        chronos::scheduler::SCHEDULER_INSTANCE
            .finish_fragment(
                fragment_id.try_into().unwrap(),
                chronos::scheduler::QueryResult::ParquetExec(file_scan_conf),
            )
            .await;

        if request_content.root {
            // let mut scheduler = SCHEDULER_INSTANCE.lock().await;
            if let Some(tx) = SCHEDULER_INSTANCE
                .job_status
                .write()
                .await
                .remove(&request_content.query_id)
            {
                tx.send(request_content.file_scan_config).await.unwrap();
            }
        }

        let reply = QueryExecutionDoneRet {};
        Ok(Response::new(reply))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:50051".parse()?;
    let scheduler = MyScheduler::default();

    println!("Scheduler server listening on {addr}");

    Server::builder()
        .add_service(SchedulerServer::new(scheduler))
        .serve(addr)
        .await?;

    Ok(())
}
