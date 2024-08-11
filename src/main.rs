use mongodb::Client;
use service::stream::StreamServiceImpl;
use service::token::TokenServiceImpl;
use std::sync::Arc;
use tokio::net::TcpListener;

#[tokio::main]
async fn main() {
    let config = config::load_config().expect("Failed to load config");
    let client = Client::with_uri_str(config.database.uri.as_str())
        .await
        .expect("Failed to connect to MongoDB");
    let database = client.database(config.database.database.as_str());

    let state = route::AppState {
        token_service: Arc::new(TokenServiceImpl {
            collection: database.collection("tokens"),
        }),
        stream_service: Arc::new(StreamServiceImpl {
            collection: database.collection("streams"),
        }),
    };

    let app = route::router(state);
    let listener = TcpListener::bind(format!("{}:{}", config.server.host, config.server.port))
        .await
        .expect("Failed to bind to port");

    axum::serve(listener, app).await.unwrap();
}
