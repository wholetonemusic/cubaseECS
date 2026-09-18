use std::sync::OnceLock;
use std::time::Instant;

use serde_json::{json, Value};

use crate::midi::{port::port_from_env, sysex};

static STARTED_AT: OnceLock<Instant> = OnceLock::new();

fn uptime_secs() -> u64 {
    STARTED_AT.get_or_init(Instant::now).elapsed().as_secs()
}

/// SysEx送信・応答待ちの担当。Phase1はエンコード＋モック応答まで。
pub struct Dispatcher {
    pub midi_mode: String,
}

impl Dispatcher {
    pub fn from_env() -> Self {
        Self {
            midi_mode: std::env::var("MIDI_MODE").unwrap_or_else(|_| "mock".to_string()),
        }
    }

    pub fn mock() -> Self {
        Self {
            midi_mode: "mock".to_string(),
        }
    }

    /// method + params をSysEx化して送信し、JSON-RPC result相当を返す。
    pub async fn dispatch(&self, method: &str, params: Value) -> Value {
        let envelope = json!({"jsonrpc": "2.0", "method": method, "params": params});
        match sysex::encode(&envelope) {
            Ok(bytes) => {
                let port = port_from_env();
                if let Err(e) = port.send_sysex(&bytes) {
                    return json!({"ok": false, "mode": port.name(), "error": e.to_string()});
                }
                // Phase1: 実Cubase応答待ちは未実装のためエコー応答
                let mut result = json!({
                    "ok": true,
                    "mode": port.name(),
                    "method": method,
                    "sysex_bytes": bytes.len(),
                    "note": "Phase1 mock: Cubase応答待ちはPhase2で実装",
                });
                if method == "session.status" {
                    result["service"] = json!("cubase-ecs-api");
                    result["version"] = json!(env!("CARGO_PKG_VERSION"));
                    result["uptime_secs"] = json!(uptime_secs());
                }
                result
            }
            Err(e) => json!({"ok": false, "error": e.to_string()}),
        }
    }
}
