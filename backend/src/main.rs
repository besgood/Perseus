use axum::{Router, routing::get};
use std::net::SocketAddr;

async fn hello() -> &'static str {
    "Hello, Perseus backend is running!"
}

#[tokio::main]
async fn main() {
    let app = Router::new().route("/", get(hello));
    let addr = SocketAddr::from(([0, 0, 0, 0], 8000));
    println!("Listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
