use tokio_tungstenite::{ WebSocketStream, MaybeTlsStream, connect_async, tungstenite::Message };
use futures::{SinkExt, StreamExt};
use tokio::{ sync::Mutex, task::{ self, JoinHandle }, net::TcpStream };
use std::sync::Arc;

use serde_json::json;

async fn send_identify(stream: &mut futures::stream::SplitSink<WebSocketStream<tokio_tungstenite::MaybeTlsStream<tokio::net::TcpStream>>, tokio_tungstenite::tungstenite::Message>, token: &str) {
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

pub enum ConnectionError {
    tungstenite( tokio_tungstenite::tungstenite::Error ),
    InvalidAuthorization
}

impl From<tokio_tungstenite::tungstenite::Error> for ConnectionError {
    fn from(err: tokio_tungstenite::tungstenite::Error) -> Self {
        ConnectionError::tungstenite( err )
    }
}

// my second mem leak in rust somehow 
pub async fn connect(token: &str) -> Result<WebSocket_Connected, ConnectionError> {
    let (stream, _) = connect_async("wss://gateway.discord.gg/?v=9&encoding=json").await?;
    let ( mut write, mut read ) = stream.split();

    read.next().await;
    
    // ready event or auth invalid 
    send_identify(&mut write, token).await;

    // handle ready
    match read.next().await {
        Some( Ok( tokio_tungstenite::tungstenite::protocol::Message::Text(resp)) ) => {
            println!("{:?}", resp);
        }
        _ => {
            return Err( ConnectionError::InvalidAuthorization ); 
        }
    }

    let websocket_threads = WebsocketThreads {
        reader: Some( task::spawn( async move { loop {  read.next().await; } }) ),
        ..Default::default()
    };

    let arc_mutex_writer = Arc::new( Mutex::new( write ) );
    
    let mut conn = WebSocket_Connected {
        mutex_write: arc_mutex_writer,
        threads: websocket_threads
    };

    conn.send_heartbeat();

    Ok( conn )
}

#[derive(Default)]
pub struct WebsocketThreads {
    heartbeat: Option<JoinHandle<()> >,
    reader: Option<JoinHandle<()> >,
    request: Option<JoinHandle<()> >,
}

pub struct WebSocket_Connected {
    pub mutex_write: Arc< Mutex< futures::stream::SplitSink<WebSocketStream<tokio_tungstenite::MaybeTlsStream<tokio::net::TcpStream>>, tokio_tungstenite::tungstenite::Message> > >, 
    pub threads: WebsocketThreads
}

impl WebSocket_Connected {
    fn send_heartbeat(&mut self) {
        let mutex_write = self.mutex_write.clone();

        self.threads.heartbeat = Some( task::spawn(async move {
            let payload = json!({
                "op": 1,
                "d": 251
            });

            loop {
                mutex_write.lock().await.send(payload.to_string().into()).await;
                tokio::time::sleep( std::time::Duration::from_millis( 41000 ) ).await;
            }
        }) );
    }

    pub fn send_request(&mut self, payload: String, interval: u64) {
        let mutex_write = self.mutex_write.clone();
        self.threads.request = Some( task::spawn( async move { 
            loop {
                mutex_write.lock().await.send( payload.clone().into() ).await;
                tokio::time::sleep( std::time::Duration::from_millis( interval ) ).await;
            }   
        }) );
    }

    fn recconect(&mut self) {
        todo!()
    }

    pub fn disconnect(&mut self) {
        let mutex_write = self.mutex_write.clone();
     
        task::spawn(async move { mutex_write.lock().await.close(); });

        if let ( Some(reader), Some(heartbeat) ) = ( &self.threads.reader, &self.threads.heartbeat ) {
            reader.abort();
            heartbeat.abort();
        }

        if let Some(request) = &self.threads.request {
            request.abort();
        }

    } 
}
