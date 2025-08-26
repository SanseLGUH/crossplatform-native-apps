mod events;

use crate::client::{ SyncCommands, websocket::{
	read::events::*, 
	types::{WebsocketReader, AtomicState, WebSocketState}, 
	error::{WebResult, ConnectionError}
}};
use tokio_tungstenite::tungstenite::protocol::Message;
use tokio::{task, sync::Mutex, task::JoinHandle};

use crossbeam::channel::{Sender, Receiver, bounded};

use futures::StreamExt;
use std::sync::Arc;

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
	pub command_recv: Receiver<SyncCommands>,
	thread: JoinHandle<()>,
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

		let (send, recv) = bounded(2);

		let mut client = Client {
			websocket_data: Arc::new(Mutex::new(data)),
			command_recv: recv,
			thread: Client::read(reader, send, state),
		};

		Ok(client)
	}

	fn read(mut reader: WebsocketReader, command: Sender<SyncCommands>, state: AtomicState) -> JoinHandle<()> {
		task::spawn(async move {
			loop {
				match reader.next().await {
					Some(_resp) => {
						// command sending command to sync client to reconnect
						command.send( SyncCommands::Reconnect );
					},
					_ => {
						println!("nothing responded");
					}
				}

				tokio::time::sleep(std::time::Duration::from_millis(100)).await;
			}
		})
	}

	pub fn disconnect(&mut self) {
		self.thread.abort();
	}
}


// this code is in raw stage