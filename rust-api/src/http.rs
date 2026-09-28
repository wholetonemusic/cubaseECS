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
    // host-midi無効時は ports 付加がないため unused_mut になる。feature依存の正当な可変。
    #[allow(unused_mut)]
    let mut body = json!({
        "status": "ok",
        "service": "cubase-ecs-api",
        "version": env!("CARGO_PKG_VERSION"),
        "midi_mode": std::env::var("MIDI_MODE").unwrap_or_else(|_| "mock".to_string()),
        "midi_port_want": std::env::var("MIDI_PORT").unwrap_or_else(|_| "loopMIDI Port".to_string()),
        "analyzer_available": crate::analyzer::is_available(),
        "analyzer_age_ms": crate::analyzer::age_ms(),
    });
    #[cfg(feature = "host-midi")]
    {
        body["midi_out_ports"] = crate::midi::port::host::list_output_ports();
        body["midi_in_ports"] = crate::midi::port::host::list_input_ports();
    }
    (StatusCode::OK, Json(body))
}

pub fn build_app() -> Router {
    Router::new()
        .route("/rpc", post(rpc_handler))
        .route("/healthz", get(healthz))
        .layer(tower_http::limit::RequestBodyLimitLayer::new(64 * 1024))
        .layer(tower_http::cors::CorsLayer::permissive())
        .layer(tower_http::trace::TraceLayer::new_for_http())
}
