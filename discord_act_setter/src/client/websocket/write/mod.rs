use crate::client::websocket::types::{WebSocketSender, SharedSender};
use crate::client::websocket::error::WebResult;

use tokio::task::JoinHandle;

use serde_json::json;

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

    // stream.send(payload.to_string().into()).await;
}

#[derive(Default, Debug)]
pub struct WebsocketThreads {
    heartbeat: Option<JoinHandle<()> >,
    request: Option<JoinHandle<()> >,
}

#[derive(Debug)]
pub struct Client {
    pub token: String,
    pub mutex_write: SharedSender, 
    pub threads: WebsocketThreads
}

impl Client {
    fn new( write: SharedSender, token: &str ) -> Self {
        todo!()
    }

    fn send_heartbeat(&mut self) {
        // let mutex_write = self.mutex_write.clone();

        // self.threads.heartbeat = Some( task::spawn(async move {
        //     let payload = json!({
        //         "op": 1,
        //         "d": 251
        //     });

        //     loop {
        //         mutex_write.lock().await.send(payload.to_string().into()).await;
        //         tokio::time::sleep( std::time::Duration::from_millis( 41000 ) ).await;
        //     }
        // }) );
    }

    pub fn send_request(&mut self, payload: String, interval: u64) {
        // let mutex_write = self.mutex_write.clone();
        
        // self.threads.request = Some( task::spawn( async move { 
        //     loop {
        //         println!("{:?}", payload.clone());
        //         mutex_write.lock().await.send( payload.clone().into() ).await;

        //         tokio::time::sleep( std::time::Duration::from_millis( interval ) ).await;
        //     }   
        // }) );
    }

    pub fn disconnect(&mut self) {
        // let mutex_write = self.mutex_write.clone();
     
        // task::spawn(async move {
        //     mutex_write.lock().await.send( serde_json::to_string( &GatewayEvent::without_activities() ).unwrap().into() ).await; 
        //     mutex_write.lock().await.close().await;
        // });

        // if let Some(heartbeat) = &self.threads.heartbeat {
        //     // reader.abort();
        //     heartbeat.abort();
        // }

        // if let Some(request) = &self.threads.request {
        //     request.abort();
        // }
    } 
}
