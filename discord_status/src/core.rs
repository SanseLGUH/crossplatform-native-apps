use tokio_tungstenite::{ WebSocketStream, MaybeTlsStream, connect_async, tungstenite::Message };
use futures::{SinkExt, StreamExt};
use tokio::{ sync::Mutex, task, net::TcpStream };
use std::sync::Arc;

use serde_json::json;

struct Websocket_TRY_Connect;

struct Websocket_CONNECTED;

// writer: SplitSink<WebSocketStream<MaybeTlsStream<tokio::net::TcpStream>>, tokio_tungstenite::tungstenite::Message>, 

impl Websocket_TRY_Connect {
    pub async fn connect() -> Result<Websocket_CONNECTED, ()> {
        let (mut stream, _) = connect_async("wss://gateway.discord.gg/?encoding=json&v=9").await.unwrap();
        
        // stream.send(Message::Text("Hello world".into() ));

        todo!();
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
}

impl Websocket_CONNECTED {
    fn send_request(name: &str, state: &str, details: &str, url: &str, r#type: i64, party: String, assets: String, secrets: String) {
// Example activity {
//  "details": "24H RL Stream for Charity",
//  "state": "Rocket League",
//  "name": "Twitch",
//  "type": 1,
//  "url": "https://www.twitch.tv/discord"
// }

        todo!()
    }

    fn recconect() {
        todo!()
    }
}
