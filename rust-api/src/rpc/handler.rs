use axum::http::StatusCode;
use serde_json::{json, Value};

use crate::cubase::dispatcher::Dispatcher;
use crate::rpc::types::{CommandExecParams, JsonRpcResponse, MixerSetParams, PluginSetParamParams};

/// POST /rpc の分岐本体。テスト容易性のため dispatcher を注入する。
pub async fn handle_rpc(req: Value, dispatcher: &Dispatcher) -> Result<Value, (StatusCode, Value)> {
    let method = req.get("method").and_then(Value::as_str).unwrap_or("");
    let params = req.get("params").cloned().unwrap_or(Value::Null);
    let id = req.get("id").cloned().unwrap_or(Value::Null);

    // jsonrpc バージョンチェック
    if req.get("jsonrpc").and_then(Value::as_str) != Some("2.0") {
        let resp =
            JsonRpcResponse::err(id, -32600, "Invalid Request: jsonrpc must be \"2.0\"", None);
        return Ok(serde_json::to_value(resp).unwrap());
    }

    match method {
        "mixer.set" => {
            let p: MixerSetParams =
                serde_json::from_value(params).map_err(|e| invalid_params(&id, &e.to_string()))?;
            if let Err(detail) = validate_mixer(&p) {
                return Err(invalid_params(&id, &detail));
            }
            let result = dispatcher
                .dispatch("mixer.set", serde_json::to_value(&p).unwrap(), id.clone())
                .await;
            Ok(serde_json::to_value(JsonRpcResponse::ok(id, result)).unwrap())
        }
        "plugin.set_param" => {
            let p: PluginSetParamParams =
                serde_json::from_value(params).map_err(|e| invalid_params(&id, &e.to_string()))?;
            if let Err(detail) = validate_plugin(&p) {
                return Err(invalid_params(&id, &detail));
            }
            let result = dispatcher
                .dispatch(
                    "plugin.set_param",
                    serde_json::to_value(&p).unwrap(),
                    id.clone(),
                )
                .await;
            Ok(serde_json::to_value(JsonRpcResponse::ok(id, result)).unwrap())
        }
        "command.exec" => {
            let p: CommandExecParams =
                serde_json::from_value(params).map_err(|e| invalid_params(&id, &e.to_string()))?;
            if let Err(detail) = validate_command(&p) {
                return Err(invalid_params(&id, &detail));
            }
            let result = dispatcher
                .dispatch(
                    "command.exec",
                    serde_json::to_value(&p).unwrap(),
                    id.clone(),
                )
                .await;
            Ok(serde_json::to_value(JsonRpcResponse::ok(id, result)).unwrap())
        }
        "session.status" => {
            let result = dispatcher
                .dispatch("session.status", json!({}), id.clone())
                .await;
            Ok(serde_json::to_value(JsonRpcResponse::ok(id, result)).unwrap())
        }
        "" => {
            let resp = JsonRpcResponse::err(id, -32600, "Invalid Request: missing method", None);
            Ok(serde_json::to_value(resp).unwrap())
        }
        other => {
            let resp =
                JsonRpcResponse::err(id, -32601, format!("Method not found: {}", other), None);
            Ok(serde_json::to_value(resp).unwrap())
        }
    }
}

fn invalid_params(id: &Value, detail: &str) -> (StatusCode, Value) {
    let resp = JsonRpcResponse::err(
        id.clone(),
        -32602,
        "Invalid params",
        Some(json!({ "detail": detail })),
    );
    (StatusCode::OK, serde_json::to_value(resp).unwrap())
}

/// mixer.set の Cubase非依存バリデーション。NG時は detail 文字列を返す。
fn validate_mixer(p: &MixerSetParams) -> Result<(), String> {
    if p.track.trim().is_empty() {
        return Err("track must be non-empty".to_string());
    }
    match p.param.to_lowercase().as_str() {
        "volume" => Ok(()),
        "volume01" => {
            if !(0.0..=1.0).contains(&p.value) {
                return Err(format!("volume01 must be 0..1, got {}", p.value));
            }
            Ok(())
        }
        "mute" | "solo" => Ok(()),
        "pan" => {
            if !(-1.0..=1.0).contains(&p.value) {
                return Err(format!("pan must be -1..1, got {}", p.value));
            }
            Ok(())
        }
        other => Err(format!(
            "unsupported mixer param: {other} (volume/volume01/mute/solo/pan)"
        )),
    }
}

fn validate_plugin(p: &PluginSetParamParams) -> Result<(), String> {
    if p.track.trim().is_empty() {
        return Err("track must be non-empty".to_string());
    }
    if p.param.trim().is_empty() {
        return Err("param must be non-empty".to_string());
    }
    Ok(())
}

/// Cubase Remote側 TRANSPORT_IDS と対応する既知コマンドのみ通す。
fn validate_command(p: &CommandExecParams) -> Result<(), String> {
    const KNOWN: &[&str] = &[
        "Transport_Play",
        "TransportPlay",
        "play",
        "Transport_Stop",
        "TransportStop",
        "stop",
        "Transport_Record",
        "TransportRecord",
        "record",
    ];
    if KNOWN.contains(&p.id.as_str()) {
        Ok(())
    } else {
        Err(format!(
            "unsupported command id: {} (Transport_Play/Transport_Stop/Transport_Record)",
            p.id
        ))
    }
}
