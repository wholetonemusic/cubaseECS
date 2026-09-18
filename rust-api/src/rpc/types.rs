use serde::{Deserialize, Serialize};
use serde_json::Value;

/// JSON-RPC 2.0 request. `params` と `id` は仕様通りゆるく受ける。
#[derive(Debug, Clone, Deserialize)]
pub struct JsonRpcRequest {
    pub jsonrpc: String,
    pub method: String,
    #[serde(default)]
    pub params: Value,
    #[serde(default)]
    pub id: Value,
}

#[derive(Debug, Clone, Serialize)]
pub struct JsonRpcResponse {
    pub jsonrpc: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<JsonRpcError>,
    pub id: Value,
}

#[derive(Debug, Clone, Serialize)]
pub struct JsonRpcError {
    pub code: i32,
    pub message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<Value>,
}

impl JsonRpcResponse {
    pub fn ok(id: Value, result: Value) -> Self {
        Self {
            jsonrpc: "2.0".to_string(),
            result: Some(result),
            error: None,
            id,
        }
    }

    pub fn err(id: Value, code: i32, message: impl Into<String>, data: Option<Value>) -> Self {
        Self {
            jsonrpc: "2.0".to_string(),
            result: None,
            error: Some(JsonRpcError {
                code,
                message: message.into(),
                data,
            }),
            id,
        }
    }
}

// ---- method params (docs 1.2準拠) ----

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct MixerSetParams {
    pub track: String,
    pub param: String,
    pub value: f64,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct PluginSetParamParams {
    pub track: String,
    pub slot: u32,
    pub plugin: String,
    pub param: String,
    pub value: f64,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CommandExecParams {
    pub id: String,
}
