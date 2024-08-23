use mongodb::Client;
use service::stream::StreamServiceImpl;
use service::token::TokenServiceImpl;
use std::sync::Arc;
use tokio::net::TcpListener;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;

#[tokio::main]
async fn main() {
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::from_default_env())
        .with(tracing_subscriber::fmt::layer())
        .init();

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
    axum::serve(listener, app).await.unwrap();
}
