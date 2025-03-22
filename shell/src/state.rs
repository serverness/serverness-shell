use std::sync::{Arc, Mutex};

use serverness::{Client, ClientConfig};
use tokio::runtime::Runtime;

pub struct State {
    pub client: Client,
    pub runtime: Runtime,
}

impl State {
    pub fn new() -> Self {
        let client_config = ClientConfig::default()
            .with_auth(
                "http://127.0.0.1:8000",
                "ness-session-ee3015132966647024de61c46821f66e3de48ae5",
            )
            .with_insecure(true);

        let client = serverness::Client::new_authenticated_config(&client_config).unwrap();

        let runtime = tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()
            .unwrap();

        Self { client, runtime }
    }
}

pub fn create_state() -> Arc<Mutex<State>> {
    let state = State::new();

    Arc::new(Mutex::new(state))
}
