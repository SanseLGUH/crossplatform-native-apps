use tokio_tungstenite::*;
use tokio::task;

struct Websocket_TRY_Connect;

struct Websocket_CONNECTED;

impl Websocket_TRY_Connect {
    pub fn connect() -> Result<Websocket_CONNECTED, ()> {
        todo!()
    }
}

impl Websocket_CONNECTED {
    fn send_request() {
        todo!()
    }

    fn recconect() {
        todo!()
    }
}
