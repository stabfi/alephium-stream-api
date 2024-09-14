use crate::error::ApiError;
use crate::AppState;
use axum::extract::State;
use axum::Json;
use model::token::Token;

#[tracing::instrument(name = "route::token::get_all", skip(state))]
pub async fn get_all(State(state): State<AppState>) -> Result<Json<Vec<Token>>, ApiError> {
    let tokens = state.token_service.get_all().await?;

    Ok(Json(tokens))
}
