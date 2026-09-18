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

    pub struct HostMidiPort {
        pub port_name: String,
    }

    impl MidiPort for HostMidiPort {
        fn send_sysex(&self, _data: &[u8]) -> Result<()> {
            // TODO Phase2: midir で仮想/実ポートへ送信
            anyhow::bail!("host-midi send not implemented yet (Phase 2)")
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
            port_name: std::env::var("MIDI_PORT").unwrap_or_else(|_| "CubaseECS".to_string()),
        });
    }
    let _ = mode;
    Box::new(MockPort)
}
