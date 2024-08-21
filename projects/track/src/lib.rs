#[derive(Debug, thiserror::Error)]
#[error("an application error has occurred")]
pub struct AppError;

pub mod error;
pub mod init;
pub mod tracker;

pub use tracker::TimeTracker;
