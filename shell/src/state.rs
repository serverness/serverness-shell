use std::sync::{Arc, Mutex};

use serverness::{Client, ClientConfig};
use tokio::runtime::Runtime;

pub struct State {
    pub client: Client,
    pub runtime: Runtime,
}

impl State {
    pub fn new(address: String, secret: String) -> Self {
        let client_config = ClientConfig::default()
            .with_auth(address, secret)
            .with_insecure(true);

        let client = serverness::Client::new_authenticated_config(&client_config).unwrap();

        let runtime = tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()
            .unwrap();

        Self { client, runtime }
    }
}

pub fn create_state(address: String, secret: String) -> Arc<Mutex<State>> {
    let state = State::new(address, secret);

    Arc::new(Mutex::new(state))
}
