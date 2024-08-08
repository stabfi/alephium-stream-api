use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::Json;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct ApiErrorPayload {
    pub code: &'static str,
    pub message: String,
}

#[derive(Debug)]
pub struct ApiError {
    pub status_code: StatusCode,
    pub payload: ApiErrorPayload,
}

impl From<service::error::ServiceError> for ApiError {
    fn from(value: service::error::ServiceError) -> Self {
        match value {
            service::error::ServiceError::DatabaseError(_) => ApiError {
                status_code: StatusCode::INTERNAL_SERVER_ERROR,
                payload: ApiErrorPayload {
                    code: "database_error",
                    message: "An error occurred while processing the request".to_string(),
                },
            },
            error @ service::error::ServiceError::NotFound => ApiError {
                status_code: StatusCode::NOT_FOUND,
                payload: ApiErrorPayload {
                    code: "not_found",
                    message: format!("{error}"),
                },
            },
        }
    }
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        (self.status_code, Json(self.payload)).into_response()
    }
}
