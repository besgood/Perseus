use axum::Router;
use tracing_subscriber;
use std::net::SocketAddr;

mod routes;
mod db;
mod scheduler;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();
    dotenv::dotenv().ok();
    let db_pool = db::init_db().await.expect("DB connection failed");
    scheduler::start_scheduler();
    let app = Router::new().nest("/api", routes::create_routes());
    let addr = SocketAddr::from(([0, 0, 0, 0], 8000));
    println!("Backend running at {}", addr);
    axum::Server::bind(&addr).serve(app.into_make_service()).await.unwrap();
}
