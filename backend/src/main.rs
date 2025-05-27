use axum::{Router, routing::{get, post}, Extension};
use tracing_subscriber;
use std::net::SocketAddr;
mod routes;
mod db;
mod auth;
mod scheduler;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();
    dotenv::dotenv().ok();
    let pool = db::init_db().await.expect("Failed to connect DB");
    scheduler::start_scheduler();
    let app = Router::new()
        .nest("/api", routes::create_routes())
        .layer(Extension(pool));
    let addr = SocketAddr::from(([0, 0, 0, 0], 8000));
    tracing::info!("Backend running on {}", addr);
    axum::Server::bind(&addr).serve(app.into_make_service()).await.unwrap();
}
