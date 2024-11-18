//! Asynchronous background job queue.
//!
//! Implemented using a Postgres table and a fast locking query (`FOR UPDATE SKIP LOCKED`).
//! This implementation makes the job queue durable (doesn't lose jobs) and fast.
pub mod clock;
pub mod cron;
pub mod error;
pub mod model;
pub mod worker;

pub use clock::Clock;
pub use cron::Cron;
pub use error::Error;
pub use model::{queue_async, queue_delay, Job, JobHandler, JobModel};
pub use worker::Worker;
