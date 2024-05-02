//! A scheduler for databases.
//!
//! Provides inter-query and intra-query parallelism by splitting up queries
//! into fragments.

mod parser;
mod queue;
/// Util code.
pub mod utils;

/// Provides APIs to schedule a DataFusion [ExecutionPlan](datafusion::physical_plan::ExecutionPlan) for execution.
pub mod scheduler;

/// gRPC interface to interact with the scheduler APIs. Intended for use by the query optimizer and execution engines.
pub mod scheduler_interface {
    tonic::include_proto!("scheduler_interface");
}

#[macro_export]
macro_rules! debug_println {
    ($($arg:tt)*) => (if ::std::cfg!(debug_assertions) { ::std::println!($($arg)*); })
}
