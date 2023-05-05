use axum::{response::Html, response::IntoResponse, routing::get, Router,
  http::{StatusCode, header::{self}}};

const UTF8_PAGE: &str = include_str!("../templates/utf8.html");
const JSON_PAGE: &str = include_str!("../templates/json.json");

pub fn routes() -> Router {
    Router::new().route("/encoding/utf8", get(utf8))
    .route("/json", get(json))
}

async fn utf8() -> Html<&'static str> {
    UTF8_PAGE.into()
}

async fn json() -> impl IntoResponse {
(
    StatusCode::OK,
    [(header::CONTENT_TYPE, mime::APPLICATION_JSON.essence_str())],
    JSON_PAGE,
)
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::{
        body::Body,
        http::{header, HeaderValue, Request, StatusCode},
    };
    use tower::ServiceExt;

    #[tokio::test]
    async fn utf8() {
        let app = routes();

        let response = app
            .oneshot(
                Request::builder()
                    .uri("/encoding/utf8")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);
        assert_eq!(
            response.headers().get(header::CONTENT_TYPE),
            Some(&HeaderValue::from_static(mime::TEXT_HTML_UTF_8.as_ref()))
        );

        let body = hyper::body::to_bytes(response.into_body()).await.unwrap();
        assert!(std::str::from_utf8(&body).is_ok())
    }
}
