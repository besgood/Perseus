use axum::{Router, routing::get};
use std::net::SocketAddr;
use tracing_subscriber;

async fn health() -> &'static str {
    "Perseus backend is running!"
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let app = Router::new().route("/health", get(health));

    let addr = SocketAddr::from(([0, 0, 0, 0], 8000));
    println!("Listening on {}", addr);

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
