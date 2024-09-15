use crate::types::ApiErrorResponse;
use reqwest::StatusCode;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ApiError {
    #[error(
        "API Error. Status code {status_code:?} (Resource: {resource:?}). Details: {detail:?}"
    )]
    ApiError {
        status_code: StatusCode,
        detail: String,
        resource: Option<String>,
    },

    #[error(transparent)]
    RequestError(#[from] reqwest::Error),

    #[error(transparent)]
    SerdeError(#[from] serde_json::Error),
}

impl ApiError {
    pub fn from_error_response(status_code: StatusCode, response: ApiErrorResponse) -> Self {
        Self::ApiError {
            status_code,
            detail: response.detail,
            resource: response.resource,
        }
    }
}
