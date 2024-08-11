use axum::extract::FromRef;
use axum::Router;
use service::stream::StreamService;
use service::token::TokenService;
use std::sync::Arc;

mod error;
mod stream;
mod token;

#[derive(FromRef, Clone)]
pub struct AppState {
    pub token_service: Arc<dyn TokenService + Send + Sync>,
    pub stream_service: Arc<dyn StreamService + Send + Sync>,
}

pub fn router(state: AppState) -> Router {
    Router::new()
        .route("/streams", axum::routing::get(stream::get_all))
        .route("/streams/:id", axum::routing::get(stream::get_one))
        .route("/tokens", axum::routing::get(token::get_all))
        .with_state(state)
}
