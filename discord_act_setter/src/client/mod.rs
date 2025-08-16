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
	Todo
}

pub struct SyncClient {
	sender: Sender<SyncCommands>
}

impl SyncClient {
	pub fn new(web_state: AtomicState) -> WebResult<Self> {
		let (tx, rx): (Sender<SyncCommands>, Receiver<SyncCommands>) = bounded(2);

		let atomic_state_clone = web_state.clone();

	    std::thread::spawn( move || {
	        let rt = runtime::Builder::new_multi_thread()
	            .worker_threads(2)
	            .enable_all()
	            .build()
	            .unwrap();

	        rt.block_on(async {
	        	match WebClient::connect("todo", atomic_state_clone.clone()).await {
	        		Ok(mut async_client) => {
			        	loop {
				        	match rx.recv() {
				        		Ok(message) => {
				        			match message {
				        				SyncCommands::Send(payload) => {
				        					// async_client.write.send_request(payload, 1000);
				        				}
				        				SyncCommands::Disconnect => {
				        					// async_client.disconnect().unwrap();
				        				}
				        				SyncCommands::Todo => {
				        					println!("todo");
				        				}
				        			}

				        		}
				        		Err(e) => { println!("{}", e); }
				        	}	        		
			        	}
	        		}
	        		Err(e) => {
	        			atomic_state_clone.store( WebSocketState::ConnectionError( conerr_to_errocu(e) ) );
	        		}
	        	}
	        });
	    });

	    tx.send( SyncCommands::Todo );

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