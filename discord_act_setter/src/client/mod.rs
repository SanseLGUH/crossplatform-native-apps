pub mod websocket;

use crossbeam_channel::{Sender, Receiver, bounded};

use crate::{
	client::{websocket::{WebClient, types::AtomicState , error::WebResult}}, 
	settings::WebsocketBackend
};

enum SyncCommands {
	Send(String),
	Disconnect
}

struct SyncClient {
	sender: Sender<SyncCommands>
}

impl SyncClient {
	fn new(web_state: AtomicState) -> WebResult<Self> {
		let (tx, rx): (Sender<SyncCommands>, Receiver<SyncCommands>) = bounded(100);

		Ok( Self { sender: tx } )	
	}

	fn send_request(&self) -> WebResult<()> {
		Ok(())
	}

	fn disconnect(&self) -> WebResult<()> {
		Ok(())
	}

	fn get_state(&self) -> WebResult<()> {
		Ok(())
	}
}