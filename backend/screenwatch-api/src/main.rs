use axum::{routing::get, Router, Json};
use serde::Serialize;
use tokio::net::TcpListener;
use tracing_subscriber;

#[derive(Serialize)]
struct HealthResponse {
    status: &'static str,
}

#[tokio::main]
async fn main() {
    // Enable logging / tracing
    tracing_subscriber::fmt::init();

    // Define your API routes
    let app = Router::new()
        .route("/health", get(health_check));

    // Listen on 127.0.0.1:3000
    let listener = TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();

    tracing::info!("🚀 Server running at http://127.0.0.1:3000/health");

    // NEW AXUM 0.8 SERVER API — THIS IS THE KEY
    axum::serve(listener, app)
        .await
        .unwrap();
}

async fn health_check() -> Json<HealthResponse> {
    Json(HealthResponse { status: "ok" })
}
