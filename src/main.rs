use axum::{Router, routing::get};
use tracing::{Level, info};

async fn root() -> &'static str {
    "Hello, World!"
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt().with_max_level(Level::INFO).init();

    let app = Router::new().route("/", get(root));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080")
        .await
        .expect("Failed");

    info!("Server running on http://0.0.0.0:8080");
    axum::serve(listener, app).await.expect("Failed");
}
