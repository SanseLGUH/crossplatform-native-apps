use std::sync::Arc;
use tokio::sync::Mutex;

use actix_web::{
    post, get, web::{
        self, Data, Json
    }, 
    HttpResponse, HttpServer, App
};

mod websocket;

use crate::{
    backend::websocket::{
        structures::{ GatewayEvent },
        connect, Client as WebsocketClient
    }
};

#[derive(Default)]
struct WebsocketClientState {
    client: Option<WebsocketClient>
}

#[post("/start")]
async fn start(state: Data<Arc<Mutex<WebsocketClientState>>>) -> HttpResponse {
    match connect("token").await {
    	Ok(client) => HttpResponse::Ok().finish(),
    	Err(e) => HttpResponse::Unauthorized().finish()
    }
}

#[get("/current_state")]
async fn current_state() -> HttpResponse {
    // current state must be ws

    HttpResponse::Ok().finish()
}

#[post("/disconnect")]
async fn disconnect(state: Data<Arc<Mutex<WebsocketClientState>>>) -> HttpResponse {
    let mut locked_state = state.lock().await;

    if let Some(client) = &mut locked_state.client {
        client.disconnect();

        return HttpResponse::Ok().finish();
    }

	HttpResponse::NotFound().finish()
}

#[post("/send_act")]
async fn send_act(payload: Json<GatewayEvent>) -> HttpResponse {
    HttpResponse::Ok().json(payload.into_inner())
}

pub async fn run_server() -> std::io::Result<()> {
	let web_client_state = Data::new(Arc::new(Mutex::new(WebsocketClientState::default())));

    HttpServer::new( move || {
        App::new()
        	.app_data(web_client_state.clone())
            .service(start)
            .service(disconnect)
            .service(current_state)
    })
    .bind(("127.0.0.1", 4462))?
    .run()
    .await
}
