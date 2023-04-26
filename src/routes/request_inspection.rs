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

#[cfg(test)]
mod tests {
    use super::*;
    use axum::{
        body::Body,
        extract::connect_info::MockConnectInfo,
        http::{header, HeaderValue, Request, StatusCode},
    };
    use http_body_util::BodyExt;
    use std::net::SocketAddr;
    use tower::ServiceExt;

    #[tokio::test]
    async fn headers() {
        let app = routes();

        let response = app
            .oneshot(
                Request::builder()
                    .uri("/headers")
                    .header("foo", "value-foo")
                    .header("bar", "value-bar")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);
        assert_eq!(
            response.headers().get(header::CONTENT_TYPE),
            Some(&HeaderValue::from_static(mime::APPLICATION_JSON.as_ref()))
        );

        let body = response.collect().await.unwrap().to_bytes();
        let response_json = serde_json::from_slice::<Value>(&body.to_vec()).unwrap();
        let headers = Value::as_object(&response_json["headers"]).unwrap();
        assert_eq!(headers["foo"], "value-foo");
        assert_eq!(headers["bar"], "value-bar");
    }

    #[tokio::test]
    async fn ip() {
        let app = routes().layer(MockConnectInfo(SocketAddr::from(([10, 10, 32, 1], 59351))));

        let response = app
            .oneshot(Request::builder().uri("/ip").body(Body::empty()).unwrap())
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);
        assert_eq!(
            response.headers().get(header::CONTENT_TYPE),
            Some(&HeaderValue::from_static(mime::APPLICATION_JSON.as_ref()))
        );

        let body = response.collect().await.unwrap().to_bytes();
        let response_json = serde_json::from_slice::<Value>(&body.to_vec()).unwrap();
        assert_eq!(&response_json["origin"], "10.10.32.1");
    }

    #[tokio::test]
    async fn user_agent() {
        let app = routes();

        let response = app
            .oneshot(
                Request::builder()
                    .uri("/user-agent")
                    .header(header::USER_AGENT, "foo-bar")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);
        assert_eq!(
            response.headers().get(header::CONTENT_TYPE),
            Some(&HeaderValue::from_static(mime::APPLICATION_JSON.as_ref()))
        );

        let body = response.collect().await.unwrap().to_bytes();
        let response_json = serde_json::from_slice::<Value>(&body.to_vec()).unwrap();
        assert_eq!(&response_json["user_agent"], "foo-bar");
    }
}
