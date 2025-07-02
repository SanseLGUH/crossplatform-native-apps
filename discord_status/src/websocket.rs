use tokio_tungstenite::{ WebSocketStream, MaybeTlsStream, connect_async, tungstenite::Message };
use futures::{SinkExt, StreamExt};
use tokio::{ sync::Mutex, task, net::TcpStream };
use std::sync::Arc;

use serde_json::json;

async fn send_idetify(stream: &mut futures::stream::SplitSink<WebSocketStream<tokio_tungstenite::MaybeTlsStream<tokio::net::TcpStream>>, tokio_tungstenite::tungstenite::Message>, token: &str) {
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
    tungstenite_error( tokio_tungstenite::tungstenite::Error ),
    InvalidAuthorization
}

impl From<tokio_tungstenite::tungstenite::Error> for ConnectionError {
    fn from(err: tokio_tungstenite::tungstenite::Error) -> Self {
        ConnectionError::tungstenite_error( err )
    }
}

// my second mem leak in rust somehow 
pub async fn connect(token: &str) -> Result<Websocket_CONNECTED, ConnectionError> {
    let (stream, _) = connect_async("wss://gateway.discord.gg/?v=9&encoding=json").await?;
    let ( mut write, mut read ) = stream.split();

    read.next().await;
    
    // ready event or auth invalid 

    send_idetify(&mut write, token).await;
    
    match read.next().await {
        Some( Ok( tokio_tungstenite::tungstenite::protocol::Message::Text(resp)) ) => {
            println!("{:?}", resp);
        }
        _ => {
            return Err( ConnectionError::InvalidAuthorization ); 
        }
    }

    task::spawn( async move { loop {  read.next().await; } });

    let arc_mutex_writer = Arc::new( Mutex::new( write ) );
    
    let conn = Websocket_CONNECTED {
        mutex_write: arc_mutex_writer
    };

    conn.send_heartbeat();

    Ok( conn )
}

pub struct Websocket_CONNECTED {
    pub mutex_write: Arc< Mutex< futures::stream::SplitSink<WebSocketStream<tokio_tungstenite::MaybeTlsStream<tokio::net::TcpStream>>, tokio_tungstenite::tungstenite::Message> > > 
}

impl Websocket_CONNECTED {
    fn send_heartbeat(&self) {
        let mutex_write = self.mutex_write.clone();

        task::spawn(async move {
            let payload = json!({
                "op": 1,
                "d": 251
            });

            loop {
                mutex_write.lock().await.send(payload.to_string().into()).await;
                tokio::time::sleep( std::time::Duration::from_millis( 41000 ) );
            }
        });
    }

    pub fn send_request(&mut self, payload: String, interval: u64) {
        let mutex_write = self.mutex_write.clone();
        task::spawn( async move { 
            loop {
                mutex_write.lock().await.send( payload.clone().into() ).await;
                tokio::time::sleep( std::time::Duration::from_millis( interval ) );
            }   
        });
    }

    fn recconect(&mut self) {
        todo!()
    }

    fn disconnect(mut self) {
        let mutex_write = self.mutex_write.clone();
        task::spawn(async move { mutex_write.lock().await.close(); });
    } 
}
