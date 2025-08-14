pub mod structures;
mod error;

use crossbeam_channel::unbounded;

use crate::{
    backend::websocket::structures::*, 
    backend::websocket::error::{
        WebResult, ConnectionError
    }
};

use tokio_tungstenite::{ 
    WebSocketStream, 
    MaybeTlsStream, 
    connect_async, 
    tungstenite::Message 
};

use futures::{
    SinkExt, 
    StreamExt
};

use tokio::{ 
    sync::Mutex, 
    task::{ self, JoinHandle }, 
    net::TcpStream 
};

use serde_json::json;

use std::sync::Arc;
use futures::stream::{SplitStream, SplitSink};

pub type WebSocket = WebSocketStream<MaybeTlsStream<tokio::net::TcpStream>>;
pub type WebSocketSender = SplitSink<WebSocket, Message>;
pub type SharedSender = Arc<Mutex<WebSocketSender>>;
pub type WebsocketReader = SplitStream<WebSocket>;

async fn send_identify(stream: &mut WebSocketSender, token: &str) {
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

// my second mem leak in rust somehow 
pub async fn connect(token: &str) -> WebResult<Client> {
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
        reader: Some( task::spawn( async move { loop {  
            if let Some( Ok( tokio_tungstenite::tungstenite::protocol::Message::Text(mess) ) ) = read.next().await {
                println!("{:?}", mess);
            } 
        } }) ),
        ..Default::default()
    };

    let arc_mutex_writer = Arc::new( Mutex::new( write ) );
    
    let mut conn = Client {
        token: token.to_string(),
        mutex_write: arc_mutex_writer,
        threads: websocket_threads
    };

    conn.send_heartbeat();

    Ok( conn )
}

#[derive(Default, Debug)]
pub struct WebsocketThreads {
    heartbeat: Option<JoinHandle<()> >,
    reader: Option<JoinHandle<()> >,
    request: Option<JoinHandle<()> >,
}

#[derive(Debug)]
pub struct Client {
    pub token: String,
    pub mutex_write: SharedSender, 
    pub threads: WebsocketThreads
}

impl Client {
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
                println!("{:?}", payload.clone());
                mutex_write.lock().await.send( payload.clone().into() ).await;

                tokio::time::sleep( std::time::Duration::from_millis( interval ) ).await;
            }   
        }) );
    }

    async fn recconect(&mut self) -> WebResult<()> {
        self.disconnect();

        let (stream, _) = connect_async("wss://gateway.discord.gg/?v=9&encoding=json").await?;
        let ( mut write, mut read ) = stream.split();

        read.next().await;

        // ready event or auth invalid 
        send_identify(&mut write, &self.token).await;

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

        let mut conn = Client {
            token: self.token.to_string(),
            mutex_write: arc_mutex_writer,
            threads: websocket_threads
        };

        conn.send_heartbeat();

        Ok(())
    }

    pub fn disconnect(&mut self) {
        let mutex_write = self.mutex_write.clone();
     
        task::spawn(async move {
            mutex_write.lock().await.send( serde_json::to_string( &GatewayEvent::without_activities() ).unwrap().into() ).await; 
            mutex_write.lock().await.close().await;
        });

        if let ( Some(reader), Some(heartbeat) ) = ( &self.threads.reader, &self.threads.heartbeat ) {
            reader.abort();
            heartbeat.abort();
        }

        if let Some(request) = &self.threads.request {
            request.abort();
        }
    } 
}
