#[derive(Debug, thiserror::Error)]
#[error("an application error has occurred")]
pub struct AppError;

pub mod common;
pub mod error;
pub mod feature;
pub mod init;
