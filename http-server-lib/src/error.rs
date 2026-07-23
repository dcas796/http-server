use std::io;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("Failed to bind to port: {0}")]
    BindError(#[from] io::Error),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Error)]
pub enum RequestError {
    #[error("Malformed http request")]
    MalformedRequest,
    #[error("UTF-8 error: {0}")]
    Utf8Error(#[from] std::str::Utf8Error),
    #[error("Invalid http request type")]
    InvalidRequestType,
    #[error("Unsupported http version")]
    UnsupportedHttpVersion,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Error)]
pub enum ResponseError {
    #[error("Invalid http response code: {0}")]
    InvalidStatusCode(u16),
}
