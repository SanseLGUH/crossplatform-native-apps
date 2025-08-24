pub mod websocket;

use tokio::runtime;
use crossbeam_channel::{Sender, Receiver, bounded};

use crate::{
	client::{websocket::{WebClient, types::{AtomicState, WebSocketState, conerr_to_errocu} , error::WebResult}}, 
	settings::WebsocketBackend
};

enum SyncCommands {
	Send(String),
	Disconnect,
}

pub struct SyncClient {
	sender: Sender<SyncCommands>
}

impl SyncClient {
	pub fn new(web_state: AtomicState, token: &str) -> Self {
		let (tx, rx): (Sender<SyncCommands>, Receiver<SyncCommands>) = bounded(3);

		let atomic_state_clone = web_state.clone();
		let token = token.to_string();

	    std::thread::spawn( move || {
	        let rt = runtime::Builder::new_multi_thread()
	            .worker_threads(2)
	            .enable_all()
	            .build()
	            .unwrap();

	        rt.block_on(async {

	        	web_state.store( WebSocketState::Connecting );

	        	match WebClient::connect(&token, atomic_state_clone, "wss://gateway.discord.gg/?v=9&encoding=json").await {
	        		Ok(mut async_client) => {
			        	
	        			web_state.store( WebSocketState::Connected );

			        	loop {
				        	match rx.recv() {
				        		Ok(message) => {
				        			match message {
				        				SyncCommands::Send(payload) => {
				        					async_client.write.send_request(payload, 100000);
				        				}
				        				SyncCommands::Disconnect => {
				        					async_client.disconnect().await;

				        					break;
				        				}
				        			}

				        		}
				        		Err(e) => { println!("{e}"); }
				        	}	        		
			        	}
	        		}
	        		Err(e) => {
	        			web_state.store( WebSocketState::ConnectionError( conerr_to_errocu(e) ) );
	        		}
	        	}
	        });
	    });

		Self { 
			sender: tx 
		}
	}

	pub fn send_request(&self, payload: String) {
		self.sender.send( SyncCommands::Send(payload) );
	}

	pub fn disconnect(&self) {
		self.sender.send( SyncCommands::Disconnect );
	}
}