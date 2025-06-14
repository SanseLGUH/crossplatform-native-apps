use druid::{Data, Lens, im::Vector};
use serde::{Deserialize, Serialize};

pub const RED_COLOR: &str = "e63232";
pub const BLUE_COLOR: &str = "5cc4ff";
pub const DARK_COLOR: &str = "1D1D1D";
pub const GRAY_COLOR: &str = "3C3C3C";

#[derive(Clone, Data, Lens)]
pub struct AppState {
    pub input: String,
    pub mini_logs: String,
    pub console: Vector<String>,
    pub settings: ToggleSettings,
    pub input_data: InputData,
}

#[derive(Clone, Data, Lens)]
pub struct ToggleSettings {
    pub clean_up: bool,
    pub copy_roles: bool,
    pub copy_channels: bool,
    pub copy_params_roles: bool
}

#[derive(Clone, Data, Lens)]
pub struct InputData {
    pub counter: u8,
    pub token: String,
    pub your_id: String,
    pub wanted_id: String,
}

#[derive(Clone, Deserialize, Serialize, Default, Debug)]
pub struct DiscordRole {
    pub id: String,
    pub name: String,
    pub permissions: String,
    pub position: i32,
    pub color: i32,
    pub hoist: bool,
    pub managed: bool,
    pub mentionable: bool,
    pub flags: i32,
}

#[derive(Clone, Serialize, Default, Debug)]
pub struct DiscordRolePayload {
    pub color: i32,
    pub name: String,
    pub permissions: String,
}

#[derive(Serialize, Debug)]
pub struct Payload {
    pub name: String,
    pub permission_overwrites: Vec<ChannelPermission>,
    pub r#type: i32,
    pub position: i32,
    pub parent_id: Option<String>,
}

#[derive(Deserialize, Serialize,  Default, Debug)]
pub struct DiscordChannels {
    pub name: String,
    pub permission_overwrites: Vec<ChannelPermission>,
    pub position: i32,
    pub parent_id: Option<String>,
    pub r#type: i32,
    pub id: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ChannelPermission {
    pub allow: String,
    pub deny: String,
    pub id: String,
    pub r#type: i32,
}

impl ChannelPermission {
    pub fn default(your_server_id: &str) -> Vec<Self> {
        vec![
            ChannelPermission {
                allow: "0".to_string(),
                deny: "0".to_string(), 
                id: your_server_id.to_string(), 
                r#type: 0,
        }]
    }
}
