use axum::{
    Json, Router,
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
};
use std::net::SocketAddr;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

use cubase_ecs_api::{cubase::dispatcher::Dispatcher, rpc::handler::handle_rpc};

async fn rpc_handler(Json(req): Json<serde_json::Value>) -> impl IntoResponse {
    let dispatcher = Dispatcher::from_env();
    match handle_rpc(req, &dispatcher).await {
        Ok(resp) => (StatusCode::OK, Json(resp)).into_response(),
        Err((status, body)) => (status, Json(body)).into_response(),
    }
}

async fn healthz() -> impl IntoResponse {
    (
        StatusCode::OK,
        Json(serde_json::json!({ "status": "ok", "service": "cubase-ecs-api" })),
    )
}

#[tokio::main]
async fn main() {
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| "info".into()))
        .with(tracing_subscriber::fmt::layer())
        .init();

    let app = Router::new()
        .route("/rpc", post(rpc_handler))
        .route("/healthz", get(healthz))
        .layer(tower_http::cors::CorsLayer::permissive());

    let port: u16 = std::env::var("PORT")
        .ok()
        .and_then(|v| v.parse().ok())
        .unwrap_or(3001);
    let addr = SocketAddr::from(([0, 0, 0, 0], port));
    tracing::info!("cubase-ecs-api listening on {}", addr);

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
