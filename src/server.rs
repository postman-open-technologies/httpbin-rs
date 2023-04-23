use crate::routes::{request_inspection, response_formats, root, status_codes};
use axum::{
    http::{header, HeaderValue, Method, Request, StatusCode},
    middleware::{from_fn, Next},
    response::Response,
    Router,
};
use std::env;

pub fn app() -> Router {
    Router::new()
        .merge(root::routes())
        .merge(request_inspection::routes())
        .merge(response_formats::routes())
        .merge(status_codes::routes())
        .layer(from_fn(inject_server_header))
        .layer(from_fn(inject_cors_headers))
}

async fn inject_server_header<B>(request: Request<B>, next: Next<B>) -> Response {
    let mut response = next.run(request).await;

    let headers = response.headers_mut();
    if let Ok(server) = HeaderValue::from_str(concat!("httpbin-rs/", env!("CARGO_PKG_VERSION"))) {
        headers.insert(header::SERVER, server);
    }

    response
}

async fn inject_cors_headers<B>(request: Request<B>, next: Next<B>) -> Response {
    let method = request.method().clone();
    let request_headers = request.headers().clone();
    let mut response = next.run(request).await;

    let headers = response.headers_mut();
    headers.insert(
        header::ACCESS_CONTROL_ALLOW_ORIGIN,
        request_headers
            .get(header::ORIGIN)
            .unwrap_or(&HeaderValue::from_static("*"))
            .clone(),
    );
    headers.insert(
        header::ACCESS_CONTROL_ALLOW_CREDENTIALS,
        HeaderValue::from_static("true"),
    );

    if method == Method::OPTIONS {
        headers.insert(
            header::ACCESS_CONTROL_ALLOW_METHODS,
            HeaderValue::from_static("GET, POST, PUT, DELETE, PATCH, OPTIONS"),
        );
        headers.insert(
            header::ACCESS_CONTROL_MAX_AGE,
            HeaderValue::from_static("3600"),
        );
        if let Some(cors_req_headers) = request_headers.get(header::ACCESS_CONTROL_REQUEST_HEADERS)
        {
            headers.insert(
                header::ACCESS_CONTROL_ALLOW_HEADERS,
                cors_req_headers.clone(),
            );
        }
        *response.status_mut() = StatusCode::NO_CONTENT;
    }

    response
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::{
        body::Body,
        http::{header, HeaderValue, Request},
    };
    use tower::ServiceExt;

    #[tokio::test]
    async fn inserts_server_header() {
        let app = app();

        let response = app
            .oneshot(Request::builder().uri("/").body(Body::empty()).unwrap())
            .await
            .unwrap();

        let headers = response.headers();
        assert!(headers.get(header::SERVER).is_some());
    }

    #[tokio::test]
    async fn inserts_default_cors_headers() {
        let app = app();

        let response = app
            .oneshot(Request::builder().uri("/").body(Body::empty()).unwrap())
            .await
            .unwrap();

        let headers = response.headers();
        assert_eq!(
            headers.get(header::ACCESS_CONTROL_ALLOW_ORIGIN),
            Some(&HeaderValue::from_static("*"))
        );
        assert_eq!(
            headers.get(header::ACCESS_CONTROL_ALLOW_CREDENTIALS),
            Some(&HeaderValue::from_static("true"))
        );
    }

    #[tokio::test]
    async fn inserts_cors_origin_header_with_origin_from_request() {
        let app = app();

        let response = app
            .oneshot(
                Request::builder()
                    .uri("/")
                    .header(header::ORIGIN, "example.com")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        let headers = response.headers();
        assert_eq!(
            headers.get(header::ACCESS_CONTROL_ALLOW_ORIGIN),
            Some(&HeaderValue::from_static("example.com"))
        );
    }

    #[tokio::test]
    async fn inserts_cors_headers_on_preflight_requests() {
        let app = app();

        let response = app
            .oneshot(
                Request::builder()
                    .method(Method::OPTIONS)
                    .header(
                        header::ACCESS_CONTROL_REQUEST_HEADERS,
                        "Content-Type, Authorization",
                    )
                    .uri("/")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::NO_CONTENT);
        let headers = response.headers();
        assert_eq!(
            headers.get(header::ACCESS_CONTROL_ALLOW_HEADERS),
            Some(&HeaderValue::from_static("Content-Type, Authorization"))
        );
        assert!(headers.get(header::ACCESS_CONTROL_ALLOW_METHODS).is_some());
        assert!(headers.get(header::ACCESS_CONTROL_MAX_AGE).is_some());
    }
}
