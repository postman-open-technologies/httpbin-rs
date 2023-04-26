use axum::{response::Html, routing::get, Router};

const UTF8_PAGE: &str = include_str!("../templates/utf8.html");

pub fn routes() -> Router {
    Router::new().route("/encoding/utf8", get(utf8))
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
    use http_body_util::BodyExt;
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

        let body = response.collect().await.unwrap().to_bytes();
        assert!(std::str::from_utf8(&body).is_ok())
    }
}
