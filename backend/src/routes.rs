use axum::{Router, Json, routing::{get, post}};
use serde_json::json;
use tracing::info;

pub fn create_routes() -> Router {
    Router::new()
        .route("/status", get(status))
        .route("/scan/start", post(start_scan))
        .route("/admin/audit", get(get_audit_logs))
}

pub async fn status() -> Json<serde_json::Value> {
    Json(json!({ "status": "running" }))
}

pub async fn start_scan() -> Json<serde_json::Value> {
    info!("Starting mock scan...");
    Json(json!({ "message": "Scan started", "job_id": "mock-job-123" }))
}

pub async fn get_audit_logs() -> Json<serde_json::Value> {
    Json(json!([
        { "action": "scan_start", "user": "admin", "timestamp": "2025-05-25" }
    ]))
}
