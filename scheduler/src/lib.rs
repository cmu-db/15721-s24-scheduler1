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

#[macro_export]
macro_rules! debug_println {
    ($($arg:tt)*) => (if ::std::cfg!(debug_assertions) { ::std::println!($($arg)*); })
}

// mod mock_executor;
// mod proto_server;
