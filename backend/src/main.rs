use axum::{Router, routing::{get, post}, Extension};
use tracing_subscriber;
use std::net::SocketAddr;
use sqlx::PgPool;

mod routes;
mod db;
mod auth;
mod scheduler;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();
    dotenv::dotenv().ok();
    let db_pool = db::init_db().await.expect("DB connection failed");
    scheduler::start_scheduler();
    let app = Router::new()
        .nest("/api", routes::create_routes())
        .layer(Extension(db_pool));
    let addr = SocketAddr::from(([0, 0, 0, 0], 8000));
    println!("Backend running at {}", addr);
    axum::Server::bind(&addr).serve(app.into_make_service()).await.unwrap();
}
