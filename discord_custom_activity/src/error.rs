use thiserror::Error;

type Result<T> = std::result::Result<T, ConnectionError>; 

pub enum ConnectionError {
    #[error("Tungstenite error: {0}")]
    tungstenite( #[from] tokio_tungstenite::tungstenite::Error ),
    #[error("InvalidAuthorization")]
    InvalidAuthorization
}