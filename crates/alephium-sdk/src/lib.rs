pub mod api;
pub mod client;
pub mod error;
pub mod types;
pub mod utils;

pub(crate) type ApiResult<T> = Result<T, error::ApiError>;
