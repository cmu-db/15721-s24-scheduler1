//! A scheduler for databases.
//!
//! Provides inter-query and intra-query parallelism by splitting up queries
//! into fragments.

pub mod integration;
mod parser;
mod queue;

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
