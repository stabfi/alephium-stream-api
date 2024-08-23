mod otel;

use dotenv::dotenv;
use mongodb::Client;
use service::stream::StreamServiceImpl;
use service::token::TokenServiceImpl;
use std::sync::Arc;
use tokio::net::TcpListener;

#[tokio::main]
async fn main() {
    dotenv().ok();

    init_tracing_opentelemetry::tracing_subscriber_ext::init_subscribers()
        .expect("Failed to initialize tracing");

    tracing::info!("Loading config");
    let config = config::load_config().expect("Failed to load config");

    tracing::info!("Connecting to MongoDB");
    let client = Client::with_uri_str(config.database.uri.as_str())
        .await
        .expect("Failed to connect to MongoDB");

    tracing::info!("Connecting to database");
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

    tracing::info!("Listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app)
        .with_graceful_shutdown(otel::shutdown_signal())
        .await
        .unwrap();
}
