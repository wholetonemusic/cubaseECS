use axum::http::StatusCode;
use serde_json::{Value, json};

use crate::cubase::dispatcher::Dispatcher;
use crate::rpc::types::{CommandExecParams, JsonRpcResponse, MixerSetParams, PluginSetParamParams};

/// POST /rpc の分岐本体。テスト容易性のため dispatcher を注入する。
pub async fn handle_rpc(
    req: Value,
    dispatcher: &Dispatcher,
) -> Result<Value, (StatusCode, Value)> {
    let method = req.get("method").and_then(Value::as_str).unwrap_or("");
    let params = req.get("params").cloned().unwrap_or(Value::Null);
    let id = req.get("id").cloned().unwrap_or(Value::Null);

    // jsonrpc バージョンチェック
    if req.get("jsonrpc").and_then(Value::as_str) != Some("2.0") {
        let resp = JsonRpcResponse::err(id, -32600, "Invalid Request: jsonrpc must be \"2.0\"", None);
        return Ok(serde_json::to_value(resp).unwrap());
    }

    match method {
        "mixer.set" => {
            let p: MixerSetParams = serde_json::from_value(params).map_err(|e| {
                let resp = JsonRpcResponse::err(id.clone(), -32602, "Invalid params", Some(json!({ "detail": e.to_string() })));
                (StatusCode::OK, serde_json::to_value(resp).unwrap())
            })?;
            let result = dispatcher.dispatch("mixer.set", serde_json::to_value(&p).unwrap()).await;
            Ok(serde_json::to_value(JsonRpcResponse::ok(id, result)).unwrap())
        }
        "plugin.set_param" => {
            let p: PluginSetParamParams = serde_json::from_value(params).map_err(|e| {
                let resp = JsonRpcResponse::err(id.clone(), -32602, "Invalid params", Some(json!({ "detail": e.to_string() })));
                (StatusCode::OK, serde_json::to_value(resp).unwrap())
            })?;
            let result =
                dispatcher.dispatch("plugin.set_param", serde_json::to_value(&p).unwrap()).await;
            Ok(serde_json::to_value(JsonRpcResponse::ok(id, result)).unwrap())
        }
        "command.exec" => {
            let p: CommandExecParams = serde_json::from_value(params).map_err(|e| {
                let resp = JsonRpcResponse::err(id.clone(), -32602, "Invalid params", Some(json!({ "detail": e.to_string() })));
                (StatusCode::OK, serde_json::to_value(resp).unwrap())
            })?;
            let result =
                dispatcher.dispatch("command.exec", serde_json::to_value(&p).unwrap()).await;
            Ok(serde_json::to_value(JsonRpcResponse::ok(id, result)).unwrap())
        }
        "session.status" => {
            let result = dispatcher.dispatch("session.status", json!({})).await;
            Ok(serde_json::to_value(JsonRpcResponse::ok(id, result)).unwrap())
        }
        "" => {
            let resp = JsonRpcResponse::err(id, -32600, "Invalid Request: missing method", None);
            Ok(serde_json::to_value(resp).unwrap())
        }
        other => {
            let resp = JsonRpcResponse::err(
                id,
                -32601,
                format!("Method not found: {}", other),
                None,
            );
            Ok(serde_json::to_value(resp).unwrap())
        }
    }
}
