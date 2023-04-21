use std::net::SocketAddr;

use axum::{
    extract::{ConnectInfo, TypedHeader},
    headers::UserAgent,
    http::HeaderMap,
    response::IntoResponse,
    routing::get,
    Json, Router,
};
use serde_json::{json, Map, Value};

pub fn routes() -> Router {
    Router::new()
        .route("/headers", get(headers))
        .route("/ip", get(ip))
        .route("/user-agent", get(user_agent))
}

async fn headers(headers: HeaderMap) -> impl IntoResponse {
    let return_headers = Map::from_iter(headers.iter().map(|(name, value)| {
        (
            name.as_str().into(),
            Value::String(value.to_str().unwrap().into()),
        )
    }));
    Json(json!({ "headers": return_headers }))
}

async fn ip(ConnectInfo(addr): ConnectInfo<SocketAddr>) -> Json<Value> {
    Json(json!({ "origin": addr.ip().to_string() }))
}

async fn user_agent(TypedHeader(user_agent): TypedHeader<UserAgent>) -> Json<Value> {
    Json(json!({ "user_agent": user_agent.as_str() }))
}
