use std::net::TcpStream;
use std::io::{self, Read};

use reqwest::blocking::Client;

use crate::settings::WebsocketBackend;

fn current_state( state: WebsocketBackend ) -> std::io::Result<()> {
	let mut stream = TcpStream::connect("127.0.0.1:4467")?;

	loop {
		stream.read(&mut [0; 128])?;

		match stream.read(&mut [0; 128]) {
			Ok(_) => {
			}
			Err(e) => panic!("encountered IO error: {e}"),
		}
	}

	Ok(())
}

fn disconnect() {
}

fn connect() {
}

fn send_act() {
}