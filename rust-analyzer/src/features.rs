//! 共有メモリ上の特徴量構造体 (spec §4.3) と AI向けJSON変換 (spec §4.4)。
//!
//! 仕様との差分 (cubaseECS親和性):
//! - `magic / version / seq` を先頭に付加。`seq` は lock-free seqlock用
//!   (odd=書込中 / even=確定)。VST3はRTスレッドのため Mutex不使用 (spec §7)。
//! - 残りは0埋め予約とし、全体で1KB固定 (spec §4.2)。

use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

/// OS共有メモリ名。`Global\` 接頭辞は付けない
/// (サービス/管理者セッション共用が必要になったら付与する)。
pub const SHM_OS_ID: &str = "CubaseECS_AudioFeatures_v1";
/// メモリサイズ。1KB固定 (spec §4.2)。
pub const SHM_SIZE: usize = 1024;
/// マジック "ECSF" (CubaseECS Features)。撕裂・未初期化検出用。
pub const FEATURES_MAGIC: u32 = 0x4543_5346;
pub const FEATURES_VERSION: u16 = 1;
/// この値を超えたフレームは `stale:true` として扱う。
pub const STALE_THRESHOLD_MS: u64 = 500;

/// 共有メモリ上のバイナリ構造体。`#[repr(C)]` + 1KBパディング。
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct AudioFeatures {
    pub magic: u32,
    pub version: u16,
    pub _reserved: u16,
    /// seqlockカウンタ。Writerが +1(odd)→書込→+1(even)。
    pub seq: u32,

    pub rms: f32,
    pub peak: f32,
    pub crest_factor: f32,

    pub band_low: f32,
    pub band_lowmid: f32,
    pub band_mid: f32,
    pub band_highmid: f32,
    pub band_high: f32,

    pub transient: f32,
    pub stereo_width: f32,

    pub timestamp_ms: u64,

    // 先行フィールド合計 64B (u64アライメント含む) + pad = 1024B。
    pub _pad: [u8; 960],
}

// フィールド合計 60B + pad 964B = 1024B をコンパイル時に保証する。
const _: () = assert!(std::mem::size_of::<AudioFeatures>() == SHM_SIZE);

impl AudioFeatures {
    pub fn empty() -> Self {
        Self {
            magic: FEATURES_MAGIC,
            version: FEATURES_VERSION,
            _reserved: 0,
            seq: 0,
            rms: f32::NEG_INFINITY,
            peak: f32::NEG_INFINITY,
            crest_factor: 0.0,
            band_low: f32::NEG_INFINITY,
            band_lowmid: f32::NEG_INFINITY,
            band_mid: f32::NEG_INFINITY,
            band_highmid: f32::NEG_INFINITY,
            band_high: f32::NEG_INFINITY,
            transient: 0.0,
            stereo_width: 1.0,
            timestamp_ms: 0,
            _pad: [0; 960],
        }
    }

    pub fn is_valid(&self) -> bool {
        self.magic == FEATURES_MAGIC && self.version == FEATURES_VERSION
    }

    /// spec §4.4 のJSON形 (+ `stale` 付加)。`now_ms` は読取側の現在時刻。
    /// 非有限値 (-inf等) は -120.0 に丸める (JSONは非有限数を表現不可)。
    pub fn to_json_value(&self, now_ms: u64) -> Value {
        let stale = now_ms.saturating_sub(self.timestamp_ms) > STALE_THRESHOLD_MS;
        json!({
            "rms": fin(self.rms),
            "peak": fin(self.peak),
            "crest": fin(self.crest_factor),
            "bands": {
                "low": fin(self.band_low),
                "lowmid": fin(self.band_lowmid),
                "mid": fin(self.band_mid),
                "highmid": fin(self.band_highmid),
                "high": fin(self.band_high),
            },
            "transient": fin(self.transient),
            "stereo_width": fin(self.stereo_width),
            "timestamp": self.timestamp_ms,
            "stale": stale,
        })
    }
}

/// AIエージェント入力の用途・目標 (spec §5.1)。プリセット選択のヒント用。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum Purpose {
    Vocal,
    Drum,
    Bass,
    Other,
}

/// JSON化のための有限化。非有限値は無音フロアに丸める。
fn fin(x: f32) -> f32 {
    if x.is_finite() {
        x
    } else {
        -120.0
    }
}

/// 現在時刻 (UNIX epoch ms)。`timestamp_ms` 用。
pub fn now_ms() -> u64 {
    use std::time::{SystemTime, UNIX_EPOCH};
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_millis() as u64)
        .unwrap_or(0)
}

/// spec §5.2 のAI出力例を cubaseECS の既存RPCへ正規化する際の雛形。
/// dB/Hzの生値は `plugin.set_param` の 0..1 プロセス値へ変換して送る
/// (変換は Cubase Remote側。ここでは形のみ示す)。
pub fn example_ai_output() -> Value {
    json!({
        "eq": [
            { "freq": 80, "type": "highpass", "slope": 24 },
            { "freq": 300, "gain": -3, "q": 1.2 },
            { "freq": 5000, "gain": 2, "q": 1.0 }
        ],
        "compressor": { "ratio": 3.0, "attack": 20, "release": 120, "threshold": -16 }
    })
}
