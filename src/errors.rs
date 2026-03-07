use thiserror::Error;

#[derive(Error, Debug)]
pub enum ServerError {
    #[error("Failed to bind TCP listener")]
    TcpListener,

    #[error("Failed to serve Axum server")]
    AxumServer,

    #[error("Failed to connect to the database")]
    Connection,
}
