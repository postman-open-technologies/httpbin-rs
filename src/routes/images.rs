use axum::{
    body::StreamBody,
    http::{header::{self}, StatusCode, header::{ACCEPT, HeaderMap}},
    response::{Html, IntoResponse},
    routing::get,
    {Router}
};

use tokio_util::io::ReaderStream;

const SVG_LOGO: &str = include_str!("../templates/images/svg_logo.svg");

pub fn routes() -> Router {
    Router::new().route("/image/svg", get(svg))
    .route("/image/jpeg", get(jpeg))
    .route("/image/png", get(png))
    .route("/image/webp", get(webp))
    .route("/image", get(image))
    .route("/favicon.ico", get(favicon))
}

async fn svg() -> Html<&'static str> {
    SVG_LOGO.into()
}

async fn favicon() -> impl IntoResponse {
    // `File` implements `AsyncRead`
    let file = match tokio::fs::File::open("static/favicon.ico").await {
        Ok(file) => file,
        Err(err) => return Err((StatusCode::NOT_FOUND, format!("File not found: {}", err))),
    };
    // convert the `AsyncRead` into a `Stream`
    let stream = ReaderStream::new(file);
    // convert the `Stream` into an `axum::body::HttpBody`
    let body = StreamBody::new(stream);

    Ok((StatusCode::OK, [(header::CONTENT_TYPE, "image/vnd.microsoft.icon")], body).into_response())
}

async fn jpeg() -> impl IntoResponse {
    let file = match tokio::fs::File::open("src/templates/images/jackal.jpg").await {
        Ok(file) => file,
        Err(err) => return Err((StatusCode::NOT_FOUND, format!("File not found: {}", err))),
    };
    let stream = ReaderStream::new(file);
    let body = StreamBody::new(stream);
    Ok((StatusCode::OK, [(header::CONTENT_TYPE, "image/jpeg")], body).into_response())
}

async fn png() -> impl IntoResponse {
    let file = match tokio::fs::File::open("src/templates/images/pig_icon.png").await {
        Ok(file) => file,
        Err(err) => return Err((StatusCode::NOT_FOUND, format!("File not found: {}", err))),
    };
    let stream = ReaderStream::new(file);
    let body = StreamBody::new(stream);
    Ok((StatusCode::OK, [(header::CONTENT_TYPE, "image/png")], body).into_response())
}

async fn webp() -> impl IntoResponse {
    let file = match tokio::fs::File::open("src/templates/images/wolf_1.webp").await {
        Ok(file) => file,
        Err(err) => return Err((StatusCode::NOT_FOUND, format!("File not found: {}", err))),
    };
    let stream = ReaderStream::new(file);
    let body = StreamBody::new(stream);
    Ok((StatusCode::OK, [(header::CONTENT_TYPE, "image/webp")], body).into_response())
}

async fn image(headers: HeaderMap) -> impl IntoResponse {
    match headers.get(ACCEPT).map(|x| x.as_bytes()) {
        Some(b"image/svg+xml") => svg().await.into_response(),
        Some(b"image/jpeg") => jpeg().await.into_response(),
        Some(b"image/webp") => webp().await.into_response(),
        Some(b"image/*") => png().await.into_response(),
        _ => png().await.into_response(), // Python implementation returns status 406 for all other
                                          // types (except if no Accept header is present)
    }.into_response()
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
    async fn svg() {
        let app = routes();

        let response = app
            .oneshot(
                Request::builder()
                    .uri("/image/svg")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);
        assert_eq!(
            response.headers().get(header::CONTENT_TYPE),
            Some(&HeaderValue::from_static(mime::IMAGE_SVG.as_ref()))
        );

        let body = hyper::body::to_bytes(response.into_body()).await.unwrap();
        assert!(std::str::from_utf8(&body).is_ok())
    }
}
