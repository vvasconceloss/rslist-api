use thiserror::Error;

#[derive(Error, Debug)]
pub enum ServerError {
    #[error("Failed to bind TCP listener")]
    TcpListenerError,

    #[error("Failed to serve Axum server")]
    AxumServerError,

    #[error("Failed to connect to the database")]
    ConnectionError,
}
