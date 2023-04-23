use axum::{
    http::{header, StatusCode},
    response::{Html, IntoResponse},
    routing::get,
    Router,
};

use minijinja::render;

const INDEX_TEMPLATE: &str = include_str!("../templates/index.html");
const OPENAPI_SPECIFICATION: &str = include_str!("../templates/openapi.yaml");
const NOT_FOUND_PAGE: &str = include_str!("../templates/not_found.html");
const API_DOCS_LOCATION: &str = "https://redocly.github.io/redoc/?url=https://raw.githubusercontent.com/postman-open-technologies/httpbin-rs/main/src/templates/openapi.yaml&nocors";

pub fn routes() -> Router {
    Router::new()
        .route("/", get(index))
        .route("/api-docs", get(api_docs))
        .route("/openapi.yaml", get(openapi))
        .fallback(not_found)
}

async fn index() -> Html<String> {
    // It looks like templates were intended here but isn't legitimately used (yet).
    render!(INDEX_TEMPLATE, prefix => "").into()
}

async fn openapi() -> impl IntoResponse {
    (
        [(header::CONTENT_TYPE, "application/yaml")],
        OPENAPI_SPECIFICATION,
    )
}

async fn api_docs() -> impl IntoResponse {
    (
        StatusCode::PERMANENT_REDIRECT,
        [(header::LOCATION, API_DOCS_LOCATION)],
    )
}

async fn not_found() -> impl IntoResponse {
    (StatusCode::NOT_FOUND, Html(NOT_FOUND_PAGE))
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::{
        body::Body,
        http::{HeaderValue, Request},
    };
    use tower::ServiceExt;

    #[tokio::test]
    async fn index() {
        let app = routes();

        let response = app
            .oneshot(Request::builder().uri("/").body(Body::empty()).unwrap())
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);
        assert_eq!(
            response.headers().get(header::CONTENT_TYPE),
            Some(&HeaderValue::from_static(mime::TEXT_HTML_UTF_8.as_ref()))
        );
    }

    #[tokio::test]
    async fn openapi() {
        let app = routes();
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/openapi.yaml")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);
        assert_eq!(
            response.headers().get(header::CONTENT_TYPE),
            Some(&HeaderValue::from_static("application/yaml")) // not a registered mime type (2023)
        );
    }

    #[tokio::test]
    async fn api_docs() {
        let app = routes();
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/api-docs")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::PERMANENT_REDIRECT);
        assert!(response.headers().get(header::LOCATION).is_some());
    }

    #[tokio::test]
    async fn not_found() {
        let app = routes();

        let response = app
            .oneshot(
                Request::builder()
                    .uri("/narwhals")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::NOT_FOUND);
        assert_eq!(
            response.headers().get(header::CONTENT_TYPE),
            Some(&HeaderValue::from_static(mime::TEXT_HTML_UTF_8.as_ref()))
        );
    }
}
