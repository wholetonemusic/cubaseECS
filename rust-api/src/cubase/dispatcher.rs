use std::sync::OnceLock;
use std::time::Instant;

use serde_json::{json, Value};

use crate::midi::{port::port_from_env, sysex};

static STARTED_AT: OnceLock<Instant> = OnceLock::new();

fn uptime_secs() -> u64 {
    STARTED_AT.get_or_init(Instant::now).elapsed().as_secs()
}

/// SysEx送信・応答待ちの担当。mock時はエンコード＋エコー応答、
/// host時は送信後に Cubase からのSysEx応答を id 相関で待つ(Phase 2)。
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

    /// method + params + id をSysEx化して送信し、JSON-RPC result相当を返す。
    /// mock時は即エコー応答、host時は Cubase からの応答を id 相関で待つ。
    pub async fn dispatch(&self, method: &str, params: Value, id: Value) -> Value {
        let envelope = json!({"jsonrpc": "2.0", "id": id, "method": method, "params": params});
        match sysex::encode(&envelope) {
            Ok(bytes) => {
                if self.midi_mode != "host" {
                    return self.mock_result(method, &bytes);
                }
                self.dispatch_host(method, &bytes, &id).await
            }
            Err(e) => json!({"ok": false, "error": e.to_string()}),
        }
    }

    fn mock_result(&self, method: &str, bytes: &[u8]) -> Value {
        let port = port_from_env();
        let mut result = json!({
            "ok": true,
            "mode": port.name(),
            "method": method,
            "sysex_bytes": bytes.len(),
            "note": "mock mode: set MIDI_MODE=host with --features host-midi for real Cubase",
        });
        if method == "session.status" {
            result["service"] = json!("cubase-ecs-api");
            result["version"] = json!(env!("CARGO_PKG_VERSION"));
            result["uptime_secs"] = json!(uptime_secs());
        }
        result
    }

    async fn dispatch_host(&self, _method: &str, bytes: &[u8], id: &Value) -> Value {
        #[cfg(feature = "host-midi")]
        {
            let port = port_from_env();
            if let Err(e) = port.send_sysex(bytes) {
                return json!({"ok": false, "mode": port.name(), "error": e.to_string()});
            }
            match crate::midi::responder::wait_for_response(id).await {
                Some(result) => result,
                None => json!({
                    "ok": false,
                    "mode": "host-midi",
                    "error": format!(
                        "timeout waiting for Cubase response ({} ms)",
                        crate::midi::responder::response_timeout_ms()
                    ),
                }),
            }
        }
        #[cfg(not(feature = "host-midi"))]
        {
            let _ = (bytes, id);
            json!({"ok": false, "mode": "mock",
                "error": "MIDI_MODE=host requires --features host-midi"})
        }
    }
}
