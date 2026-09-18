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
            let want = self.port_name.as_str();
            let port = ports
                .iter()
                .find(|p| out.port_name(p).map(|n| n.contains(want)).unwrap_or(false))
                .or_else(|| ports.first())
                .context("no MIDI output ports")?;
            tracing::info!(
                port = %out.port_name(port).unwrap_or_default(),
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
