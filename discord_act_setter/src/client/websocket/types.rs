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


#[derive(Default)]
pub enum WebSocketState {
    Connected,

    #[default]
    Connecting,
    WaitingForData,
    ErrorOccurred,
}

pub type AtomicState = Arc<AtomicCell<WebSocketState>>;
