use axum::{
    body::{to_bytes, Body},
    http::{Request, StatusCode},
};
use cubase_ecs_api::http::build_app;
use serde_json::{json, Value};
use tower::ServiceExt;

async fn post_raw(content_type: Option<&str>, raw: &str) -> (StatusCode, Value) {
    let app = build_app();
    let mut builder = Request::builder().uri("/rpc").method("POST");
    if let Some(ct) = content_type {
        builder = builder.header("content-type", ct);
    }
    let req = builder.body(Body::from(raw.to_string())).unwrap();
    let resp = app.oneshot(req).await.unwrap();
    let status = resp.status();
    let bytes = to_bytes(resp.into_body(), 64 * 1024).await.unwrap();
    (status, serde_json::from_slice(&bytes).unwrap())
}

async fn post_rpc(payload: Value) -> (StatusCode, Value) {
    post_raw(
        Some("application/json"),
        &serde_json::to_string(&payload).unwrap(),
    )
    .await
}

#[tokio::test]
async fn healthz_reports_service_and_version() {
    let resp = build_app()
        .oneshot(
            Request::builder()
                .uri("/healthz")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(resp.status(), StatusCode::OK);
    let bytes = to_bytes(resp.into_body(), 64 * 1024).await.unwrap();
    let body: Value = serde_json::from_slice(&bytes).unwrap();
    assert_eq!(body["status"], "ok");
    assert_eq!(body["service"], "cubase-ecs-api");
    assert_eq!(body["version"], env!("CARGO_PKG_VERSION"));
}

#[tokio::test]
async fn rpc_over_http_returns_result() {
    let (status, body) = post_rpc(json!({
        "jsonrpc": "2.0",
        "method": "mixer.set",
        "params": {"track": "Vocal", "param": "volume", "value": -6.0},
        "id": 1,
    }))
    .await;
    assert_eq!(status, StatusCode::OK);
    assert_eq!(body["result"]["ok"], true);
    assert_eq!(body["id"], 1);
}

#[tokio::test]
async fn malformed_json_returns_parse_error() {
    let (status, body) = post_raw(Some("application/json"), "{not json").await;
    assert_eq!(status, StatusCode::BAD_REQUEST);
    assert_eq!(body["error"]["code"], -32700);
}

#[tokio::test]
async fn missing_content_type_is_rejected() {
    let (status, body) = post_raw(
        None,
        r#"{"jsonrpc":"2.0","method":"session.status","params":{},"id":4}"#,
    )
    .await;
    assert_eq!(status, StatusCode::UNSUPPORTED_MEDIA_TYPE);
    assert_eq!(body["error"]["code"], -32700);
}

#[tokio::test]
async fn unknown_path_returns_404() {
    let resp = build_app()
        .oneshot(Request::builder().uri("/nope").body(Body::empty()).unwrap())
        .await
        .unwrap();
    assert_eq!(resp.status(), StatusCode::NOT_FOUND);
}
