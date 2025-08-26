use crate::client::websocket::{
    types::SharedSender,
    structures::GatewayEvent,
};

use tokio::task::{self, JoinHandle};
use futures::SinkExt;
use std::sync::{
    Arc,
    atomic::{AtomicU64, Ordering},
};
use serde_json::json;
use tokio::time::{sleep, Duration};

async fn send_identify(stream: &SharedSender, token: &str) {
    let payload = json!({
        "op": 2,
        "d": {
            "token": token,
            "properties": {
                "os": "Discord Custom Activity",
                "device": "DCA By SanseL"
            }
        }
    });

    let _ = stream.lock().await.send(payload.to_string().into()).await;
}

#[derive(Default, Debug)]
pub struct WebsocketThreads {
    heartbeat: Option<JoinHandle<()>>,
    request: Option<JoinHandle<()>>,
}

#[derive(Debug)]
pub struct Client {
    pub token: String,
    pub shared_write: SharedSender, 
    pub threads: WebsocketThreads,
}

impl Client {
    pub async fn new(write: SharedSender, token: &str) -> Self {
        send_identify(&write, token).await;

        Client {
            token: token.to_string(),
            shared_write: write.clone(),
            threads: WebsocketThreads::default(),
        }
    }

    pub fn send_heartbeat(&mut self, interval: Arc<AtomicU64>) {
        let shared_write = self.shared_write.clone();
        let interval_ms = interval.load(Ordering::Relaxed);

        self.threads.heartbeat = Some(task::spawn(async move {
            let payload = json!({ "op": 1, "d": 251 });

            loop {
                let _ = shared_write.lock().await.send(payload.to_string().into()).await;
                sleep(Duration::from_millis(interval_ms)).await;
            }
        }));
    }

    pub fn send_request(&mut self, payload: String, interval: u64) {
        let shared_write = self.shared_write.clone();

        self.threads.request = Some(task::spawn(async move {
            loop {
                let _ = shared_write.lock().await.send(payload.clone().into()).await;
                sleep(Duration::from_millis(interval)).await;
            }
        }));
    }

    pub async fn disconnect(&mut self) {
        {
            let mut shared_write = self.shared_write.lock().await;
            let _ = shared_write
                .send(
                    serde_json::to_string(&GatewayEvent::without_activities())
                        .unwrap()
                        .into(),
                )
                .await;
            let _ = shared_write.close().await;
        }

        if let Some(heartbeat) = &self.threads.heartbeat {
            heartbeat.abort();
        }

        if let Some(request) = &self.threads.request {
            request.abort();
        }
    }
}
