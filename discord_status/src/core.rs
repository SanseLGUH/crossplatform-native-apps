use tokio_tungstenite::{ WebSocketStream, MaybeTlsStream, connect_async, tungstenite::Message };
use futures::{SinkExt, StreamExt};
use tokio::{ sync::Mutex, task, net::TcpStream };
use std::sync::Arc;

use serde_json::json;

async fn send_idetify(stream: &mut WebSocketStream<MaybeTlsStream<TcpStream>>, token: &str) {
    let payload = json!({
        "op": 2,
        "d": {
            "token": token,
            "properties": {
                "os": "Discord Custom Activity",
                "device": "DCA"
            }
        }
    });

    stream.send(payload.to_string().into()).await;
}

fn send_heartbeat(mutex_stream: Arc<Mutex< WebSocketStream<MaybeTlsStream<TcpStream>> >>) {
    let mutex_stream = mutex_stream.clone();

    task::spawn(async move {
        let payload = json!({
            "op": 1,
            "d": 251
        });

        loop {
            mutex_stream.lock().await.send(payload.to_string().into()).await;
            tokio::time::sleep( std::time::Duration::from_millis( 41000 ) );
        }
    });
}

pub enum ConnectionError {
    tungstenite_error( tokio_tungstenite::tungstenite::Error ),
    InvalidAuthorization
}

impl From<tokio_tungstenite::tungstenite::Error> for ConnectionError {
    fn from(err: tokio_tungstenite::tungstenite::Error) -> Self {
        ConnectionError::tungstenite_error( err )
    }
}

pub async fn connect(token: &str) -> Result<Websocket_CONNECTED, ConnectionError> {
    let (mut stream, _) = connect_async("wss://gateway.discord.gg/?v=9&encoding=json").await?;

    stream.next().await;

    send_idetify(&mut stream, token).await;

    match stream.next().await { // need to work here
        Some( Ok( tokio_tungstenite::tungstenite::protocol::Message::Close(_) ) ) => return Err( ConnectionError::InvalidAuthorization ),
        Some( Ok( tokio_tungstenite::tungstenite::protocol::Message::Text(utf8) ) ) => {
            println!("{:?}", utf8);
        },
        _ => { println!("something went wrong ") }
    }
    
    let mutex_stream = Arc::new(Mutex::new( stream ));
    
    send_heartbeat(mutex_stream.clone());

    Ok( Websocket_CONNECTED {
        mutex_stream: mutex_stream.clone()
    } )
}

pub struct Websocket_CONNECTED {
    pub mutex_stream: Arc<Mutex< WebSocketStream<MaybeTlsStream<TcpStream>> >> 
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
