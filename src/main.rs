mod routes;
mod server;

use std::env;
use std::net::SocketAddr;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    // TODO: Better arg parsing and error handling.
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

    tracing::info!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(server::app().into_make_service_with_connect_info::<SocketAddr>())
        .await
        .unwrap();
}
