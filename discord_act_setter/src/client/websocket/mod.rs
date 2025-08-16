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

use tokio::{ 
    sync::Mutex, 
    task::{ self, JoinHandle }, 
    net::TcpStream 
};

use tokio_tungstenite::connect_async;

pub struct WebClient {
    pub read: ReadClient,
    pub write: WriteClient
}

impl WebClient {
    pub async fn connect(token: &str, web_state: AtomicState) -> WebResult<Self> {
        let (stream, _) = connect_async("wss://gateway.discord.gg/?v=9&encoding=json").await?;
        let ( mut write, mut read ) = stream.split();

        Err( ConnectionError::InvalidAuthorization )
    }

    pub fn disconnect(&mut self) -> WebResult<()> {
        self.write.disconnect();
        self.read.disconnect();

        Ok(())
    }

    async fn reconnect(&mut self) -> WebResult<()> {
        *self = Self::connect("something", self.read.state.clone()).await?;

        Ok(())
    }
}