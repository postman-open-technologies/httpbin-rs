mod routes;

use crate::routes::{request_inspection, response_formats, status_codes};
use axum::{
    http::{header, HeaderValue, Method, Request, StatusCode},
    middleware::{from_fn, Next},
    response::{Html, IntoResponse, Response},
    routing::get,
    Router,
};
use minijinja::render;
use std::env;
use std::net::SocketAddr;

const INDEX_TEMPLATE: &'static str = include_str!("./templates/index.html");
const OPENAPI_SPECIFICATION: &'static str = include_str!("./templates/openapi.yaml");
const API_DOCS_LOCATION: &'static str = "https://redocly.github.io/redoc/?url=https://raw.githubusercontent.com/postman-open-technologies/httpbin-rs/main/src/templates/openapi.yaml&poo=1&nocors";
const NOT_FOUND_PAGE: &'static str = include_str!("./templates/not_found.html");

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let args: Vec<String> = env::args().collect();
    let default_port: u16 = 8080;
    let port = match args.get(1) {
        Some(port_arg) => match port_arg.parse::<u16>() {
            Ok(port) => port,
            Err(_) => default_port,
        },
        None => default_port,
    };

    let addr = SocketAddr::from(([127, 0, 0, 1], port));

    let app = Router::new()
        .route("/", get(root))
        .route("/api-docs", get(api_docs))
        .route("/openapi.yaml", get(openapi))
        .merge(request_inspection::routes())
        .merge(response_formats::routes())
        .merge(status_codes::routes())
        .layer(from_fn(inject_headers))
        .fallback(not_found);

    tracing::debug!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service_with_connect_info::<SocketAddr>())
        .await
        .unwrap();
}

async fn inject_headers<B>(request: Request<B>, next: Next<B>) -> Response {
    let method = request.method().clone();
    let mut response = next.run(request).await;

    let headers = response.headers_mut();
    if let Ok(server) = HeaderValue::from_str(concat!("httpbin-rs/", env!("CARGO_PKG_VERSION"))) {
        headers.insert("Server", server);
    }

    headers.insert(
        "Access-Control-Allow-Origin",
        headers
            .get("Origin")
            .unwrap_or(&HeaderValue::from_static("*"))
            .clone(),
    );
    headers.insert(
        "Access-Control-Allow-Credentials",
        HeaderValue::from_static("true"),
    );

    if method == Method::OPTIONS {
        headers.insert(
            "Access-Control-Allow-Methods",
            HeaderValue::from_static("GET, POST, PUT, DELETE, PATCH, OPTIONS"),
        );
        headers.insert("Access-Control-Max-Age", HeaderValue::from_static("3600"));
        if let Some(cors_req_headers) = headers.get("Access-Control-Request-Headers") {
            headers.insert("Access-Control-Allow-Headers", cors_req_headers.clone());
        }
    }

    response
}

async fn root() -> Html<String> {
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
