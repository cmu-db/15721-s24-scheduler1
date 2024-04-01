mod parser;
mod queue;

pub mod scheduler;
pub mod scheduler_interface {
    tonic::include_proto!("scheduler_interface");
}
