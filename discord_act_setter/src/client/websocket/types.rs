use futures::stream::{SplitStream, SplitSink};
use tokio_tungstenite::{ 
    WebSocketStream, 
    MaybeTlsStream, 
    connect_async, 
    tungstenite::Message 
};

use std::sync::Arc;
use tokio::sync::Mutex;

use crossbeam::atomic::AtomicCell;

pub type WebSocket = WebSocketStream<MaybeTlsStream<tokio::net::TcpStream>>;
pub type WebSocketSender = SplitSink<WebSocket, Message>;
pub type SharedSender = Arc<Mutex<WebSocketSender>>;
pub type WebsocketReader = SplitStream<WebSocket>;

use thiserror::Error;
use crate::client::websocket::error::ConnectionError;

#[derive( PartialEq, Clone, Copy, Default, Debug)]
pub enum WebSocketState {
    Connected,
    Connecting,

    #[default]
    Disconnected,

    WaitingForData,

    ConnectionError( ConnectionErrorOccured )
}


#[derive( PartialEq, Clone, Copy, Debug)]
pub enum ConnectionErrorOccured {
    InvalidAuthorization,
    Unexpected
}

pub fn conerr_to_errocu(e: ConnectionError) -> ConnectionErrorOccured {
    match e {
        ConnectionError::InvalidAuthorization => ConnectionErrorOccured::InvalidAuthorization,
        _ => ConnectionErrorOccured::Unexpected
    }
}

pub type AtomicState = Arc<AtomicCell<WebSocketState>>;