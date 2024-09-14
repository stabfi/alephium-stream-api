use crate::error::{ApiError, ApiErrorPayload};
use crate::AppState;
use axum::extract::{Path, Query, State};
use axum::http::StatusCode;
use axum::Json;
use model::stream::Stream;
use serde::Deserialize;
use service::stream::StreamRole;

#[derive(Deserialize)]
pub struct GetAllQuery {
    creator: Option<String>,
    recipient: Option<String>,
    skip: Option<u64>,
}

pub async fn get_all(
    State(state): State<AppState>,
    Query(query): Query<GetAllQuery>,
) -> Result<Json<Vec<Stream>>, ApiError> {
    let GetAllQuery {
        creator,
        recipient,
        skip,
    } = query;

    let (address, role) = {
        match (creator, recipient) {
            (Some(creator), None) => (creator, StreamRole::Creator),
            (None, Some(recipient)) => (recipient, StreamRole::Recipient),
            _ => {
                return Err(ApiError {
                    status_code: StatusCode::BAD_REQUEST,
                    payload: ApiErrorPayload {
                        code: "invalid_query",
                        message: "Either creator or recipient must be provided".to_string(),
                    },
                })
            }
        }
    };

    let streams = state
        .stream_service
        .get_all(address.as_str(), role, skip.unwrap_or(0))
        .await?;

    Ok(Json(streams))
}

pub async fn get_one(
    State(state): State<AppState>,
    Path(stream_hash): Path<String>,
) -> Result<Json<Stream>, ApiError> {
    let stream = state.stream_service.get_one(stream_hash).await?;

    Ok(Json(stream))
}
