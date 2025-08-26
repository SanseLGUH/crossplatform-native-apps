pub mod websocket;

use tokio::runtime;
use crossbeam_channel::{Sender, Receiver, bounded};

use crate::{
	file_manager::save_token,
	client::websocket::{WebClient, types::{AtomicState, WebSocketState, conerr_to_errocu}},
};

#[derive(Debug)]
pub enum SyncCommands {
	Send(String),
	Disconnect,
	Reconnect
}

pub struct SyncClient {
	sender: Sender<SyncCommands>
}

impl SyncClient {
	pub fn new(web_state: AtomicState, token: &str) -> Self {
		let (tx, rx): (Sender<SyncCommands>, Receiver<SyncCommands>) = bounded(3);

		let state_clone = web_state.clone();
		let token = token.to_string();

		std::thread::spawn(move || {
			let rt = runtime::Builder::new_multi_thread()
				.worker_threads(1)
				.enable_all()
				.build()
				.unwrap();

			rt.block_on(async {
				web_state.store(WebSocketState::Connecting);

				match WebClient::connect(&token, state_clone, "wss://gateway.discord.gg/?v=9&encoding=json").await {
					Ok(mut async_client) => {
						let _ = save_token(&token);

						web_state.store( WebSocketState::Connected );

						loop {
							match async_client.read.command_recv.recv() {
								Ok(msg) => println!("{:?}", msg),
								Err(_) => {	}
							}

							match rx.recv() {
								Ok(message) => {
									match message {
										SyncCommands::Send(payload) => {
											async_client.write.send_request(payload, 100000);
										}
										SyncCommands::Reconnect => {
											if let Err(e) = async_client.reconnect().await {
												break;
											}
										}
										SyncCommands::Disconnect => {
											async_client.disconnect().await;
											break;
										}
									}
								}
								Err(e) => {
									println!("{e}");
								}
							}
						}
					}
					Err(e) => {
						web_state.store(WebSocketState::ConnectionError(conerr_to_errocu(e)));
					}
				}
			});
		});

		Self {
			sender: tx
		}
	}

	pub fn send_request(&self, payload: String) {
		let _ = self.sender.send(SyncCommands::Send(payload));
	}

	pub fn reconnect(&self) {
		let _ = self.sender.send(SyncCommands::Reconnect);
	}

	pub fn disconnect(&self) {
		let _ = self.sender.send(SyncCommands::Disconnect);
	}
}
