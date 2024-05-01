use axum::{routing::get, Router};
use std::net::SocketAddr;

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/ping", get(pingpong))
        .route("/health", get(health));

    let addr: SocketAddr = ([0, 0, 0, 0], 80).into();

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn pingpong() -> &'static str {
    "pong"
}

async fn health() -> &'static str {
    "ok"
}
