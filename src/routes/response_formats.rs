use axum::{response::Html, response::IntoResponse, routing::get, Router,
  http::{StatusCode, header::{self}}};

const UTF8_PAGE: &str = include_str!("../templates/utf8.html");
const XML_PAGE: &str = include_str!("../templates/sample.xml");

pub fn routes() -> Router {
    Router::new().route("/encoding/utf8", get(utf8))
    .route("/xml", get(xml))
}

async fn xml() -> impl IntoResponse {
    (
        StatusCode::OK,
        [(header::CONTENT_TYPE, mime::TEXT_XML.essence_str())],
        XML_PAGE,
    )
}

async fn utf8() -> Html<&'static str> {
    UTF8_PAGE.into()
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

    #[tokio::test]
    async fn xml() {
        let app = routes();

        let response = app
            .oneshot(
                Request::builder()
                    .uri("/xml")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);
        assert_eq!(
            response.headers().get(header::CONTENT_TYPE),
            Some(&HeaderValue::from_static(mime::TEXT_XML.essence_str()))
        );

        let body = hyper::body::to_bytes(response.into_body()).await.unwrap();
        assert!(std::str::from_utf8(&body).is_ok())
    }

}
