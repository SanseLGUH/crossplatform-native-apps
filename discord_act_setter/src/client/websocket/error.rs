use thiserror::Error;
use serde_json::Error as JsonError;

pub type WebResult<T> = std::result::Result<T, ConnectionError>; 

#[derive(Error, Debug)]
pub enum ConnectionError {
    #[error("Tungstenite error: {0}")]
    Tungstenite( #[from] tokio_tungstenite::tungstenite::Error ),
    
    #[error("Serde_Json error: {0}")]
    Json( #[from] JsonError ),

    #[error("Invalid Auth-token: re-check your token!")]
    InvalidAuthorization,

    #[error("Websocket encountered very unexpected error")]
    Unexpected,

    #[error("Encountered unexpected connection error {0}")]
    ConFailure( String )
}