use axum::routing::{get, post, Router};
use tokio::net;

mod config;
mod handler;

#[tokio::main]
async fn main() {
    // Creating Handler object.
    let request_handler = handler::Handler::new();

    // Parsing config.
    let cfg = config::GlobalConfig::new();

    // Creating applicaton.
    let app = Router::new().route("/", get(request_handler.greet()));

    // Starting to listen on a binded port.
    let listener = net::TcpListener::bind(format!("127.0.0.1:{}", cfg.port))
        .await
        .unwrap();

    println!("starting server on {}...", cfg.port);

    axum::serve(listener, app).await.unwrap();
}
