mod events;

use crate::client::websocket::{
	read::events::*,
	types::{WebsocketReader, AtomicState}, 
	error::{WebResult, ConnectionError}
};
use tokio_tungstenite::tungstenite::protocol::Message;
use tokio::{task, sync::Mutex, task::JoinHandle};
use futures::StreamExt;
use std::sync::Arc;

use serde::Deserialize;
use smart_default::SmartDefault;

#[derive(SmartDefault)]
pub struct WebsocketData {
	#[default = 41000]
	pub heartbeat_interval: u64,

	#[default(_code = r#"String::from("wss://gateway.discord.gg/?v=9&encoding=json")"#)]
	pub gateway_url: String
}

pub struct Client {
	pub websocket_data: Arc<Mutex<WebsocketData>>,
	pub thread: Option<JoinHandle<()>>,
	pub state: AtomicState
}

impl Client {
	pub async fn new(mut reader: WebsocketReader, state: AtomicState) -> WebResult<Self> {
		let mut data = WebsocketData::default();

		match reader.next().await {
			Some(Ok(resp)) => {
				let resp_ser: HelloEvent = serde_json::from_str(&resp.to_string())?;
				data.heartbeat_interval = resp_ser.d.heartbeat_interval;
			},
			_ => return Err(ConnectionError::Unexpected)
		}

		match reader.next().await {
			Some(Ok(Message::Text(resp))) => {
				let resp_ser: ReadyEvent = serde_json::from_str(&resp)?;
				data.gateway_url = resp_ser.d.resume_gateway_url;
			},
			Some(Ok(Message::Close(Some(_resp)))) => return Err(ConnectionError::InvalidAuthorization),
			Some(Err(e)) => return Err(ConnectionError::ConFailure(e.to_string())),
			_ => return Err(ConnectionError::Unexpected)
		}

		let mut client = Client {
			websocket_data: Arc::new(Mutex::new(data)),
			thread: None,
			state
		};		

		client.read(reader);

		Ok(client)
	}

	fn read(&mut self, mut reader: WebsocketReader) {
		let atomic_state = self.state.clone();

		self.thread = Some(task::spawn(async move {
			loop {
				match reader.next().await {
					Some(Ok(_)) => {
						// Placeholder: Handle actual message
					},
					_ => {
						// Optionally handle errors or connection close
						break;
					}
				}
			}
		}));
	}

	pub fn disconnect(&mut self) {
		if let Some(thread) = &self.thread {
			thread.abort();
		}
	}
}
