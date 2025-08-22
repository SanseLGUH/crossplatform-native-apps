use crate::client::websocket::{types::{WebsocketReader, AtomicState}, error::{WebResult, ConnectionError}};

use tokio_tungstenite::tungstenite::protocol::Message;

use futures::StreamExt;
use tokio::{sync::Mutex, task::JoinHandle};

use std::sync::Arc;

use tokio::task;

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
	thread: Option< JoinHandle<()> >,
	reader: WebsocketReader,
	pub state: AtomicState
}

impl Client {
	pub async fn new( mut reader: WebsocketReader, state: AtomicState ) -> WebResult<Self> {
		let mut data = WebsocketData::default();

		// hello event
		match reader.next().await {
			Some( Ok(resp) ) => { println!("{:?}", resp); },
			Some( Err(e) ) => { println!("{:?}", e); },
			None => { println!("something went wrong"); }
		}

		// check for auth error
		match reader.next().await {
			Some( Ok( Message::Close(Some(resp))) ) => return Err( ConnectionError::InvalidAuthorization),
			Some( Err(e) ) => return Err( ConnectionError::ConFailure( e.to_string() ) ),
			_ => { println!("Something went wrong") }
		}

		let mut client = Client {
			websocket_data: Arc::new( Mutex::new( data ) ),
			thread: None,
			reader: reader,
			state: state
		};

		client.read();

		Ok( client )
	}

	fn read(&mut self) {
		let atomic_state = self.state.clone();

		task::spawn(async move {

		});
	}

	pub fn disconnect(&mut self) {
		if let Some(read_task) = &self.thread {
			read_task.abort();
		}
	}
}