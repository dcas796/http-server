use std::num::ParseIntError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum AppError {
    #[error("Failed to parse number: {0}")]
    ParseNumberError(#[from] ParseIntError),
    #[error("Failed to start server: {0}")]
    ServerError(#[from] http_server_lib::error::Error),
    #[error("Attempt to clone Server struct, which is not cloneable")]
    ServerCloneError,
}
