//! A scheduler for databases.
//!
//! Provides inter-query and in and intra-query parallelism by splitting up queries
//! into fragments.
//!
//! In general, a new query fragment is created when an
//! [ExecutionPlan](datafusion::physical_plan::ExecutionPlan) has more than one child, with each child becoming
//! its own fragment. Support is also available for pipelining hash join by splitting up the hash build phase into
//! a separate fragment.

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
