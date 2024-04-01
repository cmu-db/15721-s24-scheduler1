// pub mod substrait {
//     include!(concat!(env!("OUT_DIR"), "/substrait.rs"));
//     pub mod extensions {
//         include!(concat!(env!("OUT_DIR"), "/substrait.extensions.rs"));
//     }
// }

mod integration;
mod parser;
mod queue;

pub mod scheduler;
pub mod scheduler_interface {
    tonic::include_proto!("scheduler_interface");
}
