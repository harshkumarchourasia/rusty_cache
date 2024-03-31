use axum;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::{
    extract::Path, extract::State, routing::delete, routing::get, routing::post, Json, Router,
};
use serde::Deserialize;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use tokio;
use tower::ServiceBuilder;
use tower_http::trace::TraceLayer;
use tracing;

pub enum Error {
    KeyNotFound(String),
}

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        match self {
            Error::KeyNotFound(s) => {
                (StatusCode::NO_CONTENT, format!("Key not present {}", s)).into_response()
            }
        }
    }
}

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

    pub async fn get(self, key: &str) -> Result<String, Error> {
        let map = self.kb_map.lock().unwrap();
        match map.get(key) {
            Some(v) => Ok(v.to_string()),
            None => Err(Error::KeyNotFound(key.to_string())),
        }
    }

    pub async fn unset(self, key: &str) -> Result<String, Error> {
        let mut map = self.kb_map.lock().unwrap();
        match map.remove(key) {
            Some(_) => Ok("Key unset".to_string()),
            None => Err(Error::KeyNotFound("Key was not present.".to_string())),
        }
    }
}

pub async fn set_key(State(data): State<Data>, Json(set_key): Json<SetKeyStruct>) -> String {
    tracing::info!("HANDLER => set_key");
    match data.set(set_key.key, set_key.value).await {
        Some(v) => format!("Updating the key value from {v}!"),
        None => "New Key was set!".to_string(),
    }
}

pub async fn get_key(State(data): State<Data>, Path(key): Path<String>) -> Result<String, Error> {
    tracing::info!("HANDLER => get_key");
    data.get(&key).await
}

pub async fn unset_key(State(data): State<Data>, Path(key): Path<String>) -> Result<String, Error> {
    tracing::info!("HANDLER => unset_key");
    data.unset(&key).await
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt().with_target(false).init();

    let data: Data = Data::new().await;

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();

    let v1_router: Router<()> = Router::new()
        .route("/set", post(set_key))
        .route("/get/:key", get(get_key))
        .route("/unset/:key", delete(unset_key))
        .layer(ServiceBuilder::new().layer(TraceLayer::new_for_http()))
        .with_state(data);

    let router: Router<()> = Router::new().nest("/v1", v1_router);

    axum::serve(listener, router).await.unwrap();
}
