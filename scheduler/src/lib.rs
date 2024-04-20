pub mod integration;
mod parser;
pub mod queue;

pub mod scheduler;
pub mod scheduler_interface {
    tonic::include_proto!("scheduler_interface");
}
pub mod executor_interface {
    tonic::include_proto!("executor_interface");
}

// mod mock_executor;
// mod proto_server;
