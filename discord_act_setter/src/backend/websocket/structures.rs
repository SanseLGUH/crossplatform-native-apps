use smart_default::SmartDefault;

use crossbeam::atomic::AtomicCell;
use serde::{Serialize, Deserialize};
use std::{ sync::Arc, sync::Mutex };
use tokio::task::JoinHandle;

use crate::settings::Settings;

#[derive(Deserialize, Serialize, Clone)]
pub struct GatewayEvent {
    pub op: u8, 
    pub d: GatewayEventData,
}

#[derive(Deserialize, Serialize, Clone)]
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