use axum::{
    extract::{rejection::JsonRejection, Json},
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
    Router,
};
use serde_json::{json, Value};

use crate::cubase::dispatcher::Dispatcher;
use crate::rpc::{handler::handle_rpc, types::JsonRpcResponse};

/// JSON抽出失敗をJSON-RPC Parse error (-32700) にマップする。
async fn rpc_handler(body: Result<Json<Value>, JsonRejection>) -> impl IntoResponse {
    let req = match body {
        Ok(Json(req)) => req,
        Err(rejection) => {
            let status = rejection.status();
            let resp = JsonRpcResponse::err(
                Value::Null,
                -32700,
                "Parse error",
                Some(json!({ "detail": rejection.body_text() })),
            );
            return (status, Json(serde_json::to_value(resp).unwrap())).into_response();
        }
    };

    let dispatcher = Dispatcher::from_env();
    match handle_rpc(req, &dispatcher).await {
        Ok(resp) => (StatusCode::OK, Json(resp)).into_response(),
        Err((status, resp_body)) => (status, Json(resp_body)).into_response(),
    }
}

async fn healthz() -> impl IntoResponse {
    (
        StatusCode::OK,
        Json(json!({
            "status": "ok",
            "service": "cubase-ecs-api",
            "version": env!("CARGO_PKG_VERSION"),
            "midi_mode": std::env::var("MIDI_MODE").unwrap_or_else(|_| "mock".to_string()),
        })),
    )
}

pub fn build_app() -> Router {
    Router::new()
        .route("/rpc", post(rpc_handler))
        .route("/healthz", get(healthz))
        .layer(tower_http::limit::RequestBodyLimitLayer::new(64 * 1024))
        .layer(tower_http::cors::CorsLayer::permissive())
        .layer(tower_http::trace::TraceLayer::new_for_http())
}
