use prost::Message;

use tokio::runtime::Handle;
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
            .get_next_query_fragment()
            .await;

        match plan {
            Some(plan) => {
                let plan_bytes = physical_plan_to_bytes(plan.root.unwrap());

                match plan_bytes {
                    Ok(plan_bytes) => {
                        let mut hash_build_data = vec![];
                        for location in plan.hash_probe_locations {
                            let probe_node_path = location.0;
                            let build_fragment_id = location.1;
                            hash_build_data.push(HashBuildData {
                                path_from_parent: probe_node_path,
                                build_fragment_id,
                            })
                        }

                        let reply = GetQueryRet {
                            query_details: Some(QueryDetails {
                                query_id: plan.query_id,
                                fragment_id: plan.fragment_id,
                                physical_plan: plan_bytes.to_vec(),
                                hash_build_data,
                            }),
                            root: plan.parent_fragments.is_empty(),
                            aborted: plan.aborted,
                        };
                        Ok(Response::new(reply))
                    }
                    Err(e) => {
                        println!("{:?}", e);
                        // chronos::queue::kill_query(p.query_id); TODO
                        let reply = GetQueryRet {
                            query_details: None,
                            root: true, // setting this to true frees the CLI on the tokio channel, will do for now
                            aborted: false,
                        };
                        Ok(Response::new(reply))
                    }
                }
            }
            None => {
                let reply = GetQueryRet {
                    query_details: None,
                    root: false,
                    aborted: false,
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
            .schedule_query(physical_plan, metadata.unwrap(), true)
            .await;

        let (tx, mut rx) = mpsc::channel::<Vec<u8>>(1);

        {
            SCHEDULER_INSTANCE
                .query_result_senders
                .write()
                .await
                .insert(sched_info.query_id, tx);
        }

        let reply = ScheduleQueryRet {
            query_id: sched_info.query_id,
            file_scan_config: rx.recv().await.unwrap_or_default(),
            enqueue_time: sched_info
                .enqueue_time
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_millis()
                .try_into()
                .unwrap(),
        };
        Ok(Response::new(reply))
    }

    async fn abort_query(
        &self,
        request: Request<AbortQueryArgs>,
    ) -> Result<Response<AbortQueryRet>, Status> {
        let query_id = request.into_inner().query_id;
        chronos::scheduler::SCHEDULER_INSTANCE
            .abort_query(query_id)
            .await;
        let reply = AbortQueryRet {};
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

        if query_status.is_err() {
            let status = Status::new(Code::InvalidArgument, "Query status not specified");
            return Err(status);
        }

        let file_scan_exec_conf =
            FileScanExecConf::decode(request_content.file_scan_config.clone().into_buf()).unwrap();
        let file_scan_config = from_proto::parse_protobuf_file_scan_config(
            &file_scan_exec_conf,
            &SessionContext::new(),
        )
        .unwrap();
        let query_id = request_content.query_id;
        let is_root_fragment = request_content.root;

        let to_delete = SCHEDULER_INSTANCE
            .query_execution_done(
                query_id,
                fragment_id,
                file_scan_config,
                request_content.file_scan_config,
                is_root_fragment,
            )
            .await;

        let reply = QueryExecutionDoneRet {
            intermediate_files: to_delete,
        };
        Ok(Response::new(reply))
    }
}

async fn server() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:50051".parse()?;
    let scheduler = MyScheduler::default();

    println!("Scheduler server listening on {addr}");

    Server::builder()
        .add_service(SchedulerServer::new(scheduler))
        .serve(addr)
        .await?;

    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut handles = Vec::new();
    let _handle = Handle::current;

    use tokio::runtime;

    let rt = runtime::Builder::new_multi_thread()
        .thread_stack_size(10 * 1024 * 1024)
        .enable_all()
        .build()
        .unwrap();

    for _i in 0..1 {
        handles.push(rt.spawn(async move {
            let _ = server().await;
        }));
    }

    for handle in handles {
        let _ = handle.await;
    }

    Ok(())
}
