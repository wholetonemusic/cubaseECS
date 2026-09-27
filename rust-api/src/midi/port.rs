use anyhow::Result;

/// MIDIポート抽象化。
/// Docker内(dev)は MockPort、ホスト実機接続時は HostMidiPort(feature host-midi)を使う。
pub trait MidiPort: Send + Sync {
    fn send_sysex(&self, data: &[u8]) -> Result<()>;
    fn name(&self) -> &'static str;
}

#[derive(Debug, Default)]
pub struct MockPort;

impl MidiPort for MockPort {
    fn send_sysex(&self, data: &[u8]) -> Result<()> {
        tracing::debug!(bytes = data.len(), "MockPort: SysEx send (no-op)");
        Ok(())
    }
    fn name(&self) -> &'static str {
        "mock"
    }
}

#[cfg(feature = "host-midi")]
pub mod host {
    use super::*;
    use anyhow::Context;

    /// MIDI_PORT の既定値。loopMIDI のポート名に部分一致させる
    /// ("loopMIDI Port" は "loopMIDI Port 1" にもマッチする)。
    pub fn wanted_port_name() -> String {
        std::env::var("MIDI_PORT").unwrap_or_else(|_| "loopMIDI Port".to_string())
    }

    /// ポート選択: 完全一致を優先し、なければ部分一致、最後に先頭へ
    /// フォールバックする。完全一致優先により `MIDI_PORT="loopMIDI Port 1"`
    /// のように曖昧さを排除できる。
    pub fn select_index(names: &[String], want: &str) -> Option<usize> {
        if names.is_empty() {
            return None;
        }
        names
            .iter()
            .position(|n| n == want)
            .or_else(|| names.iter().position(|n| n.contains(want)))
            .or(Some(0))
    }

    /// 診断用(/healthz): 利用可能なMIDI出力ポート名一覧。
    pub fn list_output_ports() -> serde_json::Value {
        match midir::MidiOutput::new("cubaseECS-probe") {
            Ok(out) => serde_json::Value::Array(
                out.ports()
                    .iter()
                    .map(|p| serde_json::Value::String(out.port_name(p).unwrap_or_default()))
                    .collect(),
            ),
            Err(_) => serde_json::Value::Array(vec![]),
        }
    }

    /// 診断用(/healthz): 利用可能なMIDI入力ポート名一覧。
    pub fn list_input_ports() -> serde_json::Value {
        match midir::MidiInput::new("cubaseECS-probe") {
            Ok(input) => serde_json::Value::Array(
                input
                    .ports()
                    .iter()
                    .map(|p| serde_json::Value::String(input.port_name(p).unwrap_or_default()))
                    .collect(),
            ),
            Err(_) => serde_json::Value::Array(vec![]),
        }
    }

    pub struct HostMidiPort {
        pub port_name: String,
    }

    impl HostMidiPort {
        fn connect_output(&self) -> Result<midir::MidiOutputConnection> {
            let out = midir::MidiOutput::new("cubaseECS").context("MidiOutput::new failed")?;
            let ports = out.ports();
            if ports.is_empty() {
                anyhow::bail!("no MIDI output ports found (is loopMIDI running?)");
            }
            let names: Vec<String> = ports
                .iter()
                .map(|p| out.port_name(p).unwrap_or_default())
                .collect();
            let want = self.port_name.as_str();
            let idx = select_index(&names, want).context("no MIDI output ports")?;
            let port = &ports[idx];
            tracing::info!(
                port = %names[idx],
                available = ?names,
                "HostMidiPort: connecting MIDI out"
            );
            out.connect(port, "cubaseECS-out")
                .map_err(|e| anyhow::anyhow!("MIDI out connect failed: {e}"))
        }
    }

    impl MidiPort for HostMidiPort {
        fn send_sysex(&self, data: &[u8]) -> Result<()> {
            // Phase 2: 低頻度コマンド想定のため送信ごとに接続・切断する。
            let mut conn = self.connect_output()?;
            conn.send(data).context("MIDI SysEx send failed")?;
            Ok(())
        }
        fn name(&self) -> &'static str {
            "host-midi"
        }
    }

    #[cfg(test)]
    mod tests {
        use super::select_index;

        #[test]
        fn prefers_exact_match_over_substring() {
            let names = vec!["loopMIDI Port 1".to_string(), "loopMIDI Port".to_string()];
            assert_eq!(select_index(&names, "loopMIDI Port"), Some(1));
            assert_eq!(select_index(&names, "loopMIDI Port 1"), Some(0));
            // 部分一致フォールバック
            assert_eq!(select_index(&names, "loopMIDI"), Some(0));
            // 不一致は先頭へフォールバック
            assert_eq!(select_index(&names, "nothing"), Some(0));
            assert_eq!(select_index(&[], "loopMIDI Port"), None);
        }
    }
}

/// MIDI_MODE=host かつ feature=host-midi のときのみ実ポート、それ以外はモック。
pub fn port_from_env() -> Box<dyn MidiPort> {
    let mode = std::env::var("MIDI_MODE").unwrap_or_else(|_| "mock".to_string());
    #[cfg(feature = "host-midi")]
    if mode == "host" {
        return Box::new(host::HostMidiPort {
            port_name: host::wanted_port_name(),
        });
    }
    let _ = mode;
    Box::new(MockPort)
}
