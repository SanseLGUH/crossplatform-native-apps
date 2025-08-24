use serde::Deserialize;

#[derive(Deserialize)]
pub struct Event<T> {
    pub d: T,
}

#[derive(Deserialize)]
pub struct Hello {
    pub heartbeat_interval: u64,
}

#[derive(Deserialize)]
pub struct Ready {
    pub resume_gateway_url: String,
}

pub type HelloEvent = Event<Hello>;
pub type ReadyEvent = Event<Ready>;