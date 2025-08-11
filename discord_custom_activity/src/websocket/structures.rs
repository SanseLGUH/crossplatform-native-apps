use smart_default::SmartDefault;

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

#[derive(Default, Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConnectionState {
    Disconnecting,
    
    #[default]
    Disconnected,

    Connecting,
    Connected,
    Failed,
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

#[derive(SmartDefault, Serialize, Clone)]
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

pub struct Timestamps {
    start: i64,
}

pub struct Party {
    id: String,
    size: String,
}

pub struct Assets {
    large_image: String,
    large_text: String,
    small_image: String,
    small_text: String 
}

pub struct Secrets {
    join: String,
    spectate: String,
    r#match: String
}