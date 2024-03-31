use axum;
use axum::debug_handler;
use axum::{extract::State, routing::delete, routing::get, routing::post, Json, Router};
use serde::Deserialize;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use tokio;

#[derive(Deserialize)]
pub struct SetKeyStruct {
    key: String,
    value: String,
}

#[derive(Debug, Clone)]
pub struct Data {
    pub kb_map: Arc<Mutex<HashMap<String, String>>>,
}

impl Data {
    pub async fn new() -> Data {
        Data {
            kb_map: Arc::new(Mutex::new(HashMap::new())),
        }
    }
}

impl Data {
    pub async fn set(self, key: String, value: String) -> Option<String> {
        let mut map = self.kb_map.lock().unwrap();
        let value: Option<String> = map.insert(key, value);
        return value;
    }

    // pub async fn get(self, key: &String) -> Option<&String> {
    //     let map = self.kb_map.lock().unwrap();
    //     map.get(key)
    // }

    pub async fn unset(self, key: String) {}

    pub async fn isset(self, key: String) {}
}

pub async fn set_key(State(data): State<Data>, Json(set_key): Json<SetKeyStruct>) -> String {
    match data.set(set_key.key, set_key.value).await {
        Some(v) => format!("Updating the key value from {v}!"),
        None => "New Key was set!".to_string(),
    }
}

pub async fn get_key(State(data): State<Data>) {}

pub async fn unset_key(State(data): State<Data>) {}

pub async fn is_set_key(State(data): State<Data>) {}

#[tokio::main]
async fn main() {
    let data: Data = Data::new().await;

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();

    let v1_router: Router<()> = Router::new().route("/set", post(set_key)).with_state(data);

    let router: Router<()> = Router::new().nest("/v1", v1_router);

    axum::serve(listener, router).await.unwrap();
}
