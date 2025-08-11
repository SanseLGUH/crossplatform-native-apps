use thiserror::Error;

pub type WebResult<T> = std::result::Result<T, ConnectionError>; 

#[derive(Error, Debug)]
pub enum ConnectionError {
    #[error("Tungstenite error: {0}")]
    tungstenite( #[from] tokio_tungstenite::tungstenite::Error ),
    #[error("InvalidAuthorization")]
    InvalidAuthorization,

    #[error("Encountered unexpected connection error {0}")]
    ConFailure( String )
}