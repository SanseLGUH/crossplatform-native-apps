use crossbeam::atomic::AtomicCell;
use serde::Serialize;
use std::{ sync::Arc, sync::Mutex };
use tokio::task::JoinHandle;

use crate::websocket::WebSocket_Connected;

#[derive(Default)]
pub struct WebsocketBackend {
    pub task: Option<JoinHandle<()>>, 
    pub websocket: Option< WebSocket_Connected > ,
    pub connection_state: Arc<AtomicCell<ConnectionState>>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConnectionState {
    Disconnecting,
    Disconnected,
    Connecting,
    Connected,
    Failed,
}

impl Default for ConnectionState {
    fn default() -> Self {
        ConnectionState::Disconnected
    }
}

#[derive(Serialize, Clone)]
pub struct GatewayEvent {
    pub op: u8, 
    pub d: GatewayEventData,
}

#[derive(Serialize, Clone)]
pub struct GatewayEventData {
    pub since: u64,
    pub activities: Vec<Settings>,
    pub status: String,
    pub afk: bool,
}

impl GatewayEvent {
    pub fn from_settings(settings: Settings) -> Self {
        let data = GatewayEventData {
            since: 91879200,
            activities: vec![settings],
            status: "online".to_string(),
            afk: false,
        };

        GatewayEvent {
            op: 3,
            d: data,
        }
    }

    pub fn without_activities() -> Self {
        let data = GatewayEventData {
            since: 4234093,
            activities: Vec::new(),
            status: "online".to_string(),
            afk: false
        };

        GatewayEvent {
            op: 3,
            d: data
        }
    }
}

#[derive(Serialize, Clone)]
pub struct Settings {
    pub details: String,
    pub state: String,
    pub name: String,
    pub r#type: i64,
    pub url: String,
}

impl Default for Settings {
    fn default() -> Self {
        Settings {
            details: "Rust / tokio-tungstenite / eframe".to_string(),
            state: "ver 1.0".to_string(),
            name: "Custom discord activity".to_string(),
            r#type: 0,
            url: "https://github.com/SanseLGUH".to_string()
        }
    }
}

