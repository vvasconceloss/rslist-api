mod errors;

use std::env;

use axum::{Router, routing::get};
use tracing::{Level, info};

use crate::errors::ServerError;

async fn root() -> &'static str {
    "Hello, World!"
}

#[tokio::main]
async fn main() -> Result<(), ServerError> {
    dotenvy::dotenv().ok();
    tracing_subscriber::fmt().with_max_level(Level::INFO).init();

    let _database_url = env::var("DATABASE_URL").expect("Failed");
    let app = Router::new().route("/", get(root));

    let listener = match tokio::net::TcpListener::bind("0.0.0.0:8080").await {
        Ok(listener) => listener,
        Err(_) => return Err(ServerError::TcpListenerError),
    };

    info!("Server running on http://0.0.0.0:8080");

    axum::serve(listener, app)
        .await
        .map_err(|_e| ServerError::AxumServerError)?;

    Ok(())
}
