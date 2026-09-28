//! DSPコア (spec §3.3)。VST3 / テスト双方から使う純粋計算部。
//!
//! - レベル系: RMS/Peak (dBFS)、Crest = Peak - RMS。
//! - 帯域別: rustfft + Hann窓。5バンド固定 (20-120/120-500/500-2.5k/2.5-6k/6-20k)。
//! - トランジェント: `|RMS_short - RMS_long|` (線形値域。spec §3.3.3)。
//! - ステレオ幅: 左右相関 -1..1 (Mono/無音時は 1.0)。

use num_complex::Complex;
use rustfft::FftPlanner;

/// 無音フロア (dBFS)。-infを避ける。
pub const FLOOR_DB: f32 = -120.0;

/// 帯域境界 [low, lowmid, mid, highmid, high] の上限Hz。
pub const BAND_EDGES_HZ: [f32; 6] = [20.0, 120.0, 500.0, 2500.0, 6000.0, 20000.0];

pub fn lin_to_db(x: f32) -> f32 {
    if x <= 0.0 {
        FLOOR_DB
    } else {
        (20.0 * x.log10()).max(FLOOR_DB)
    }
}

pub fn rms_of(samples: &[f32]) -> f32 {
    if samples.is_empty() {
        return 0.0;
    }
    let sum: f32 = samples.iter().map(|s| s * s).sum();
    (sum / samples.len() as f32).sqrt()
}

pub fn rms_db(samples: &[f32]) -> f32 {
    lin_to_db(rms_of(samples))
}

pub fn peak_of(samples: &[f32]) -> f32 {
    samples.iter().fold(0.0f32, |m, s| m.max(s.abs()))
}

pub fn peak_db(samples: &[f32]) -> f32 {
    lin_to_db(peak_of(samples))
}

pub fn crest_factor(rms_db: f32, peak_db: f32) -> f32 {
    (peak_db - rms_db).max(0.0)
}

/// 左右相関。+1=同一 / 0=無相関(広い) / -1=逆相。
/// 空・無エネルギー時は 1.0 (中央定位扱い)。
pub fn stereo_correlation(left: &[f32], right: &[f32]) -> f32 {
    let n = left.len().min(right.len());
    if n == 0 {
        return 1.0;
    }
    let (mut lr, mut l2, mut r2) = (0.0f32, 0.0f32, 0.0f32);
    for i in 0..n {
        lr += left[i] * right[i];
        l2 += left[i] * left[i];
        r2 += right[i] * right[i];
    }
    let denom = (l2 * r2).sqrt();
    if denom <= f32::EPSILON {
        1.0
    } else {
        (lr / denom).clamp(-1.0, 1.0)
    }
}

/// 短時間RMS (線形) と長時間RMS (線形) の差の絶対値。
pub fn transient_index(rms_short_lin: f32, rms_long_lin: f32) -> f32 {
    (rms_short_lin - rms_long_lin).abs()
}

#[derive(Debug, Clone, Copy, Default)]
pub struct BandEnergiesDb {
    pub low: f32,
    pub lowmid: f32,
    pub mid: f32,
    pub highmid: f32,
    pub high: f32,
}

/// FFT帯域分析器。`FftPlanner` と窓・バッファを事前確保し、
/// RT中の確保を避ける。`sample_rate` 変更時のみ再計画する。
pub struct FftBands {
    fft_len: usize,
    sample_rate: f32,
    hann: Vec<f32>,
    buf: Vec<Complex<f32>>,
    fft: std::sync::Arc<dyn rustfft::Fft<f32>>,
}

impl FftBands {
    pub fn new(fft_len: usize, sample_rate: f32) -> Self {
        let mut planner = FftPlanner::<f32>::new();
        let fft = planner.plan_fft_forward(fft_len);
        let hann: Vec<f32> = (0..fft_len)
            .map(|i| 0.5 * (1.0 - (2.0 * std::f32::consts::PI * i as f32 / fft_len as f32).cos()))
            .collect();
        Self {
            fft_len,
            sample_rate,
            hann,
            buf: vec![Complex::new(0.0, 0.0); fft_len],
            fft,
        }
    }

    /// `mono.len() == fft_len` のこと。各バンドのRMSをdBFSで返す。
    pub fn analyze(&mut self, mono: &[f32]) -> BandEnergiesDb {
        debug_assert_eq!(mono.len(), self.fft_len);
        let n = self.fft_len;
        for (i, b) in self.buf.iter_mut().enumerate() {
            let s = if i < mono.len() { mono[i] } else { 0.0 };
            *b = Complex::new(s * self.hann[i], 0.0);
        }
        self.fft.process(&mut self.buf);

        // |X[k]|^2 のバンド別平均 → RMS → dB。DC/Nyquistは除外しない
        // (窓適用後の漏れ込みを帯域エネルギーに含める方針)。
        let nyquist = self.sample_rate * 0.5;
        let mut acc = [0.0f32; 5];
        let mut cnt = [0usize; 5];
        let half = n / 2;
        for k in 1..half {
            let freq = k as f32 * self.sample_rate / n as f32;
            if freq < BAND_EDGES_HZ[0] || freq > nyquist {
                continue;
            }
            let band = if freq < BAND_EDGES_HZ[1] {
                0
            } else if freq < BAND_EDGES_HZ[2] {
                1
            } else if freq < BAND_EDGES_HZ[3] {
                2
            } else if freq < BAND_EDGES_HZ[4] {
                3
            } else if freq <= BAND_EDGES_HZ[5] {
                4
            } else {
                continue;
            };
            let mag2 = self.buf[k].norm_sqr();
            acc[band] += mag2;
            cnt[band] += 1;
        }
        let db = |i: usize| {
            if cnt[i] == 0 {
                FLOOR_DB
            } else {
                // FFTスケーリングを戻す: mag/N → 時間領域RMS相当。
                lin_to_db((acc[i] / cnt[i] as f32).sqrt() / n as f32)
            }
        };
        BandEnergiesDb {
            low: db(0),
            lowmid: db(1),
            mid: db(2),
            highmid: db(3),
            high: db(4),
        }
    }
}
