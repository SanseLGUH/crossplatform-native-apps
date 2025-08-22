use smart_default::SmartDefault;
use eframe::egui::Color32;

use crate::client::websocket::types::WebSocketState;

#[derive(SmartDefault)]
pub struct Layout {
	#[default = "Displays the current program status. 
Please enter your Discord token."]
	pub label: String,

	#[default(_code = "Color32::from_gray(10)")]
	pub color: Color32,
}

impl Layout {
    pub fn update(&mut self, state: &WebSocketState) {
        match state {
            WebSocketState::Connected => {
                self.label = "WebSocket is connected.".to_string();
            }
            WebSocketState::Connecting => {
                self.label = "WebSocket is connecting...".to_string();
            }
            WebSocketState::Disconnected => {
                self.label = "WebSocket is disconnected.".to_string();
            }
            WebSocketState::WaitingForData => {
                self.label = "Waiting for data from WebSocket...".to_string();
            }
            WebSocketState::ConnectionError(e) => {
                self.label = format!("Connection failure: {:?}", e);
            }
        }
    }
}
