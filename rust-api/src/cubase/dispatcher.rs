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
                let result = self.dispatch_host(method, &bytes, &id).await;
                if method == "session.status" {
                    return Self::with_local_status(result);
                }
                result
            }
            Err(e) => json!({"ok": false, "error": e.to_string()}),
        }
    }

    /// session.status 応答にAPI側の稼働情報を付加する(host時)。
    /// Cubase応答がオブジェクトならキーをマージし、そうでなければ
    /// `cubase` キー配下に格納する。エラー応答はそのまま通す。
    fn with_local_status(mut result: Value) -> Value {
        let local = json!({
            "service": "cubase-ecs-api",
            "version": env!("CARGO_PKG_VERSION"),
            "midi_mode": "host-midi",
            "uptime_secs": uptime_secs(),
        });
        match result.as_object_mut() {
            Some(map) => {
                if map.get("ok") == Some(&Value::Bool(false)) && map.get("error").is_some() {
                    map.insert("local".to_string(), local);
                } else {
                    for (k, v) in local.as_object().unwrap() {
                        map.entry(k.clone()).or_insert(v.clone());
                    }
                }
                result
            }
            None => json!({"ok": true, "local": local, "cubase": result}),
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
            result["midi_mode"] = json!(port.name());
            result["uptime_secs"] = json!(uptime_secs());
        }
        result
    }

    async fn dispatch_host(&self, _method: &str, bytes: &[u8], id: &Value) -> Value {
        #[cfg(feature = "host-midi")]
        {
            use crate::midi::responder;
            // self-echo対策: 待機登録→送信→待機の順。送信後に登録すると
            // loopMIDI反響を取りこぼし、応答を誤配送する。
            let rx = responder::register(id);
            let port = port_from_env();
            if let Err(e) = port.send_sysex(bytes) {
                responder::cancel(id);
                return json!({"ok": false, "mode": port.name(), "error": e.to_string()});
            }
            match responder::wait_on_registered(rx, id, responder::response_timeout()).await {
                Some(result) => result,
                None => json!({
                    "ok": false,
                    "mode": "host-midi",
                    "error": format!(
                        "timeout waiting for Cubase response ({} ms)",
                        responder::response_timeout_ms()
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
