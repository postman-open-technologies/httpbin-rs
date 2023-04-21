use axum::{response::Html, routing::get, Router};

const UTF8_PAGE: &'static str = include_str!("../templates/utf8.html");

pub fn routes() -> Router {
    Router::new().route("/encoding/utf8", get(utf8))
}

async fn utf8() -> Html<&'static str> {
    UTF8_PAGE.into()
}
