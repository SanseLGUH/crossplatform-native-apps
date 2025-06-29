use tokio_tungstenite::{ WebSocketStream, MaybeTlsStream, connect_async, tungstenite::Message };
use futures::{SinkExt, StreamExt};
use tokio::{ sync::Mutex, task, net::TcpStream };
use std::sync::Arc;

use serde_json::json;

pub struct Websocket_CONNECTED {
    pub mutex_stream: Arc<Mutex< WebSocketStream<MaybeTlsStream<TcpStream>> >> 
}

fn send_idetify(stream: &mut WebSocketStream<MaybeTlsStream<TcpStream>>, token: &str) {
    let payload = json!({
        "op": 2,
        "d": {
            "token": token,
            "properties": {
                "os": "Croissant Software",
                "device": "Croissant"
            }
        }
    });

    stream.send(payload.to_string().into());
}

pub async fn connect(token: &str) -> Result<Websocket_CONNECTED, ()> {
    let (mut stream, _) = connect_async("wss://gateway.discord.gg/?v=9&encoding=json").await.unwrap();
        
    send_idetify(&mut stream, token);

    // check if ready event seneded

    Ok( Websocket_CONNECTED {
        mutex_stream: Arc::new(Mutex::new( stream ))
    } )
}

impl Websocket_CONNECTED {
    pub fn send_request(&self, payload: String, interval: u64) {
        let mutex_stream = self.mutex_stream.clone();

        task::spawn( async move {
            loop {
                tokio::time::sleep( std::time::Duration::from_millis(interval) ).await;
                mutex_stream.lock().await.send(payload.clone().into());
            }
        });
    }

    fn recconect(&self) {
        todo!()
    }

    fn close_connection(self) {
    } 
}
