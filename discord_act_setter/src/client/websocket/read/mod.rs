use crate::client::websocket::types::{WebsocketReader, AtomicState};

use tokio::task::JoinHandle;

use std::sync::Arc;

pub struct Client {
	thread: JoinHandle<()>,
	reader: WebsocketReader,
	pub state: AtomicState
}

impl Client {
	fn new( reader: WebsocketReader, state: AtomicState ) -> Self {
		todo!()
	}

	fn read(&mut self) {
	}

	pub fn disconnect(&mut self) {
	}
}