//! AI Audio Analyzer の読取側 (spec §4–§5)。
//!
//! - VST3 (Writer) が書込むOS共有メモリを seqlock手順で読取り、
//!   spec §4.4 のJSONへ変換する。AI側は読み取り専用 (spec §7)。
//! - バイナリ構造体は `rust-analyzer/src/features.rs` の複写。
//!   VST3側 (cdylib/GPL) への依存を避けるため意図的に複写している。
//!   同期漏れ防止: `SHM_SIZE`/`FEATURES_MAGIC` の一致をテストで表明する
//!   (`analyzer_layout_matches_vst` を参照)。
//! - `MIDI_MODE=mock` (Docker/CI) 時は合成特徴量を返す (B3)。

use serde_json::{json, Value};
use shared_memory::{Shmem, ShmemConf};

pub const SHM_OS_ID: &str = "CubaseECS_AudioFeatures_v1";
pub const SHM_SIZE: usize = 1024;
pub const FEATURES_MAGIC: u32 = 0x4543_5346;
pub const FEATURES_VERSION: u16 = 1;
pub const STALE_THRESHOLD_MS: u64 = 500;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
struct AudioFeatures {
    magic: u32,
    version: u16,
    _reserved: u16,
    seq: u32,
    rms: f32,
    peak: f32,
    crest_factor: f32,
    band_low: f32,
    band_lowmid: f32,
    band_mid: f32,
    band_highmid: f32,
    band_high: f32,
    transient: f32,
    stereo_width: f32,
    timestamp_ms: u64,
    _pad: [u8; 960],
}

const _: () = assert!(std::mem::size_of::<AudioFeatures>() == SHM_SIZE);

fn fin(x: f32) -> f32 {
    if x.is_finite() {
        x
    } else {
        -120.0
    }
}

fn now_ms() -> u64 {
    use std::time::{SystemTime, UNIX_EPOCH};
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_millis() as u64)
        .unwrap_or(0)
}

fn to_json(f: &AudioFeatures, mode: &str) -> Value {
    let now = now_ms();
    let stale = now.saturating_sub(f.timestamp_ms) > STALE_THRESHOLD_MS;
    json!({
        "rms": fin(f.rms),
        "peak": fin(f.peak),
        "crest": fin(f.crest_factor),
        "bands": {
            "low": fin(f.band_low),
            "lowmid": fin(f.band_lowmid),
            "mid": fin(f.band_mid),
            "highmid": fin(f.band_highmid),
            "high": fin(f.band_high),
        },
        "transient": fin(f.transient),
        "stereo_width": fin(f.stereo_width),
        "timestamp": f.timestamp_ms,
        "stale": stale,
        "mode": mode,
    })
}

/// 共有メモリを開く。Windowsでは `allow_raw` により
/// `tools/sim_features.py` のような素の名前付きマッピングも開ける。
/// (VST3正規Writerは本クレート管理のfile-backed mapping)。
fn open_shm() -> Result<Shmem, shared_memory::ShmemError> {
    let conf = ShmemConf::new().size(SHM_SIZE).os_id(SHM_OS_ID);
    #[cfg(windows)]
    let conf = conf.allow_raw(true);
    conf.open()
}

/// 共有メモリが存在するか (VST3稼働の目安)。
pub fn is_available() -> bool {
    open_shm().is_ok()
}

/// 最終更新からの経過ms。shm不在時は `None`。
pub fn age_ms() -> Option<u64> {
    read_live_raw().map(|f| now_ms().saturating_sub(f.timestamp_ms))
}

fn read_live_raw() -> Option<AudioFeatures> {
    let shmem = open_shm().ok()?;
    if shmem.len() < SHM_SIZE {
        return None;
    }
    // seqlock読取 (Writer: VST3単一)。
    unsafe {
        let base = shmem.as_ptr() as *const AudioFeatures;
        for _ in 0..3 {
            let s1 = std::ptr::read_volatile(std::ptr::addr_of!((*base).seq));
            if s1 & 1 == 1 {
                continue;
            }
            let copy = std::ptr::read_volatile(base);
            let s2 = std::ptr::read_volatile(std::ptr::addr_of!((*base).seq));
            if s1 == s2 && copy.magic == FEATURES_MAGIC && copy.version == FEATURES_VERSION {
                return Some(copy);
            }
        }
    }
    None
}

/// 実shmからの読取。撕裂・不在時は `None`。
pub fn read_live() -> Option<Value> {
    read_live_raw().map(|f| to_json(&f, "host-midi"))
}

/// mock/synthetic応答。spec §4.4 の例値を返す。
pub fn mock_features() -> Value {
    let now = now_ms();
    json!({
        "rms": -18.5,
        "peak": -3.2,
        "crest": 15.3,
        "bands": {
            "low": -30.0,
            "lowmid": -22.0,
            "mid": -18.0,
            "highmid": -16.0,
            "high": -15.5,
        },
        "transient": 0.35,
        "stereo_width": 0.1,
        "timestamp": now,
        "stale": false,
        "mode": "mock",
        "note": "mock mode: insert AI Audio Analyzer in Cubase for live values",
    })
}

/// `analyzer.get_features` 本体。mock時は合成値、host時は実測値。
/// hostでshm不在時は `ok:false` オブジェクト (Cubase応答の流儀に合わせる)。
pub fn get_features() -> Value {
    let midi_mode = std::env::var("MIDI_MODE").unwrap_or_else(|_| "mock".to_string());
    if midi_mode != "host" {
        return mock_features();
    }
    match read_live() {
        Some(v) => v,
        None => json!({
            "ok": false,
            "mode": "host-midi",
            "error": "analyzer not running (insert AI Audio Analyzer on the selected track in Cubase)",
        }),
    }
}
