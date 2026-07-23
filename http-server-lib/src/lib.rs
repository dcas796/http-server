pub mod server;
pub mod error;
mod session;
mod http;
mod headers;
mod api;
mod types;

pub type HttpResult<T> = Result<T, error::Error>;
