use std::collections::HashMap;
use std::sync::{Mutex, OnceLock};
use std::time::Duration;

use serde_json::Value;
use tokio::sync::oneshot;

/// Cubase からの SysEx 応答待ち。`id` でリクエストと相関させる。
/// 受信リスナは host-midi フィーチャでのみ起動し、それ以外は no-op。

type Pending = Mutex<HashMap<String, oneshot::Sender<Value>>>;

static PENDING: OnceLock<Pending> = OnceLock::new();

fn pending() -> &'static Pending {
    PENDING.get_or_init(|| Mutex::new(HashMap::new()))
}

pub fn response_timeout() -> Duration {
    std::env::var("MIDI_RESPONSE_TIMEOUT_MS")
        .ok()
        .and_then(|v| v.parse::<u64>().ok())
        .map(Duration::from_millis)
        .unwrap_or_else(|| Duration::from_millis(2000))
}

pub fn response_timeout_ms() -> u64 {
    response_timeout().as_millis() as u64
}

fn id_key(id: &Value) -> String {
    match id {
        Value::Null => "null".to_string(),
        Value::Number(n) => n.to_string(),
        Value::String(s) => s.clone(),
        _ => serde_json::to_string(id).unwrap_or_else(|_| "null".to_string()),
    }
}

/// 応答を待つ。Cubase 側の `result` 部分だけを取り出して返す。
/// タイムアウト時はエントリを掃除して `None` を返す。
pub async fn wait_for_response(id: &Value) -> Option<Value> {
    wait_for_response_with_timeout(id, response_timeout()).await
}

pub async fn wait_for_response_with_timeout(id: &Value, timeout: Duration) -> Option<Value> {
    let (tx, rx) = oneshot::channel();
    pending().lock().unwrap().insert(id_key(id), tx);
    ensure_listener();
    match tokio::time::timeout(timeout, rx).await {
        Ok(Ok(v)) => Some(v),
        _ => {
            pending().lock().unwrap().remove(&id_key(id));
            None
        }
    }
}

/// 受信した応答 JSON を待機中のリクエストへ配送する。
/// `{"jsonrpc":"2.0","id":N,"result":{...}}` 形式を想定する。
pub fn deliver(msg: Value) {
    let key = msg
        .get("id")
        .map(id_key)
        .unwrap_or_else(|| "null".to_string());
    let result = msg.get("result").cloned().unwrap_or(Value::Null);
    if let Some(tx) = pending().lock().unwrap().remove(&key) {
        let _ = tx.send(result);
    }
}

#[cfg(feature = "host-midi")]
fn ensure_listener() {
    static STARTED: OnceLock<()> = OnceLock::new();
    STARTED.get_or_init(|| {
        std::thread::spawn(|| {
            if let Err(e) = run_input_loop() {
                tracing::warn!(error = %e, "MIDI input listener exited");
            }
        });
    });
}

#[cfg(not(feature = "host-midi"))]
fn ensure_listener() {}

/// MIDI 入力を開き、SysEx を受信するたび `deliver` する(終了しない)。
#[cfg(feature = "host-midi")]
fn run_input_loop() -> anyhow::Result<()> {
    use anyhow::Context;

    let want = super::port::host::wanted_port_name();
    let input = midir::MidiInput::new("cubaseECS").context("MidiInput::new failed")?;
    let ports = input.ports();
    if ports.is_empty() {
        anyhow::bail!("no MIDI input ports found (is loopMIDI running?)");
    }
    let port = ports
        .iter()
        .find(|p| {
            input
                .port_name(p)
                .map(|n| n.contains(want.as_str()))
                .unwrap_or(false)
        })
        .or_else(|| ports.first())
        .context("no MIDI input ports")?
        .clone();
    tracing::info!(
        port = %input.port_name(&port).unwrap_or_default(),
        "MIDI input listener: waiting for Cubase responses"
    );

    let (tx, rx) = std::sync::mpsc::channel::<Vec<u8>>();
    let _conn = input
        .connect(
            &port,
            "cubaseECS-in",
            move |_ts, data: &[u8], _| {
                if data.first() == Some(&0xF0) {
                    let _ = tx.send(data.to_vec());
                }
            },
            (),
        )
        .map_err(|e| anyhow::anyhow!("MIDI in connect failed: {e}"))?;

    while let Ok(bytes) = rx.recv() {
        match super::sysex::decode(&bytes) {
            Ok(json) => deliver(json),
            Err(e) => tracing::warn!(error = %e, "ignoring undecodable MIDI message"),
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[tokio::test]
    async fn delivers_to_waiter() {
        let id = json!(9001);
        let waiter = tokio::spawn(async move {
            wait_for_response_with_timeout(&id, Duration::from_secs(2)).await
        });
        // リスナ起動・登録のための小休止
        tokio::time::sleep(Duration::from_millis(50)).await;
        deliver(json!({"jsonrpc": "2.0", "id": 9001, "result": {"ok": true}}));
        assert_eq!(
            waiter.await.unwrap(),
            Some(json!({"ok": true})),
            "waiter should receive the Cubase result"
        );
    }

    #[tokio::test]
    async fn timeout_returns_none() {
        let id = json!("no-such-response");
        assert_eq!(
            wait_for_response_with_timeout(&id, Duration::from_millis(50)).await,
            None
        );
    }
}
