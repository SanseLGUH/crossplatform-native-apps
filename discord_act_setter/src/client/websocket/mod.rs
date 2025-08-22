mod read;
mod write;

pub mod structures;
pub mod error;
pub mod types;

use futures::{StreamExt, stream::{SplitStream, SplitSink}};
use crate::{
    client::websocket::{
        structures::*, read::Client as ReadClient, write::Client as WriteClient,
        error::{WebResult, ConnectionError},
    } 
};

use crate::client::websocket::types::AtomicState;

use std::sync::Arc;

use tokio::{ 
    sync::Mutex, 
    task::{ self, JoinHandle }
};

use tokio_tungstenite::connect_async;

pub struct WebClient {
    pub read: ReadClient,
    pub write: WriteClient
}

impl WebClient {
    pub async fn connect(token: &str, web_state: AtomicState, gateway_url: &str) -> WebResult<Self> {
        let (stream, _) = connect_async(gateway_url).await?;
        let ( mut write, mut read ) = stream.split();

        let shared_write = Arc::new( Mutex::new( write ) );

        let write_client = WriteClient::new( shared_write, token ).await;
        let read_client = ReadClient::new( read, web_state ).await?;

        Ok( 
            WebClient { 
                read: read_client, 
                write: write_client 
            } 
        )
    }

    pub fn disconnect(&mut self) -> WebResult<()> {
        self.write.disconnect();
        self.read.disconnect();

        Ok(())
    }

    async fn reconnect(&mut self) -> WebResult<()> {
        let gateway_url = self.read.websocket_data.lock().await.gateway_url.clone();

        *self = Self::connect(&self.write.token, self.read.state.clone(), &gateway_url).await?;

        todo!()
    }
}