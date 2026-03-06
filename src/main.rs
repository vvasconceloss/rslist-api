mod connection;
mod errors;

use axum::{Extension, Router, routing::get};
use tracing::{Level, info};

use crate::{connection::connect, errors::ServerError};

async fn root() -> &'static str {
    "Hello, World!"
}

#[tokio::main]
async fn main() -> Result<(), ServerError> {
    dotenvy::dotenv().ok();
    tracing_subscriber::fmt().with_max_level(Level::INFO).init();

    let database_pool = match connect().await {
        Ok(pool) => pool,
        Err(_) => return Err(ServerError::ConnectionError),
    };

    info!("Connection to the database successfully established");

    let app = Router::new().route("/", get(root).layer(Extension(database_pool)));

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
