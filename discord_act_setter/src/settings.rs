use smart_default::SmartDefault;

use crossbeam::atomic::AtomicCell;
use std::sync::Arc;

use tokio::task::JoinHandle;
use futures::stream::{SplitStream, SplitSink};

use serde::{Serialize, Deserialize};

#[derive(Default, Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConnectionState {
    Disconnecting,
    
    #[default]
    Disconnected,

    Connecting,
    Connected,
    Failed,
}

#[derive(Default)]
pub struct WebsocketBackend {
    pub connection_state: Arc<AtomicCell<ConnectionState>>,
}

#[derive(SmartDefault, Deserialize, Serialize, Clone)]
pub struct Settings {
    #[default = "Rust / tokio-tungstenite / eframe"]
    pub details: String,

    pub application_id: Option< String >,

    #[default = "ver 1.0"]
    pub state: String,

    #[default = "Custom discord activity"]
    pub name: String,

    pub r#type: i64,
    
    #[default = "https://github.com/SanseLGUH"]
    pub url: String,
}