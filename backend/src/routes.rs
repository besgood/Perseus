use axum::{Router, Json, Extension, routing::{get, post}};
use serde_json::json;
use sqlx::PgPool;
use std::sync::Arc;

pub fn create_routes() -> Router {
    Router::new()
        .route("/status", get(status))
        .route("/scan/start", post(start_scan))
        .route("/admin/audit", get(get_audit_logs))
}

async fn status() -> Json<serde_json::Value> {
    Json(json!({ "status": "running" }))
}

async fn start_scan(Extension(_db): Extension<PgPool>) -> Json<serde_json::Value> {
    Json(json!({ "message": "Scan started", "job_id": "job-1234" }))
}

async fn get_audit_logs(Extension(_db): Extension<PgPool>) -> Json<serde_json::Value> {
    Json(json!([ { "action": "scan", "user": "admin", "timestamp": "now" } ]))
}
