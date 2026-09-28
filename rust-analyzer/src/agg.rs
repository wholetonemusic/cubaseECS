//! 100ms (10Hz) 集計器 (spec §3.4)。
//!
//! Cubaseの可変バッファ (128〜512 samples) を内部で蓄積し、
//! `sample_rate * 0.1` サンプル毎に1フレームを出力する。
//! ステレオ入力は Mid ((L+R)/2) でレベル・FFTを計算し、
//! L/Rでステレオ幅 (相関) を計算する。Mono時は幅=1.0。

use std::collections::VecDeque;

use crate::dsp::{self, BandEnergiesDb, FftBands};

/// 集計1フレーム (共有メモリ書込直前の値)。
#[derive(Debug, Clone, Copy)]
pub struct Frame {
    pub rms_db: f32,
    pub peak_db: f32,
    pub crest: f32,
    pub bands: BandEnergiesDb,
    pub transient: f32,
    pub stereo_width: f32,
}

pub struct Aggregator {
    sample_rate: f32,
    window_needed: usize,
    short_needed: usize,
    mid: Vec<f32>,
    left: Vec<f32>,
    right: Vec<f32>,
    short_mid: VecDeque<f32>,
    peak_lin: f32,
    fft: FftBands,
}

impl Aggregator {
    pub fn new(sample_rate: f32) -> Self {
        let window_needed = ((sample_rate * 0.1).round() as usize).max(16);
        let short_needed = ((sample_rate * 0.01).round() as usize).max(4);
        let fft = FftBands::new(window_needed, sample_rate);
        Self {
            sample_rate,
            window_needed,
            short_needed,
            mid: Vec::with_capacity(window_needed),
            left: Vec::with_capacity(window_needed),
            right: Vec::with_capacity(window_needed),
            short_mid: VecDeque::with_capacity(short_needed * 2),
            peak_lin: 0.0,
            fft,
        }
    }

    pub fn sample_rate(&self) -> f32 {
        self.sample_rate
    }

    /// オーディオブロックを投入する。窓が満了したら `Some(Frame)`。
    /// `right: None` でMono扱い (内部でLを複製)。
    /// RT適合: 満了時以外の確保なし (Vec容量は事前確保、満了時のFFTは
    /// 10Hzのみ・事前計画済みのため許容)。
    pub fn push(&mut self, left: &[f32], right: Option<&[f32]>) -> Option<Frame> {
        for (i, &l) in left.iter().enumerate() {
            let r = right.and_then(|ch| ch.get(i)).copied().unwrap_or(l);
            let m = 0.5 * (l + r);
            self.peak_lin = self.peak_lin.max(m.abs());
            self.mid.push(m);
            self.left.push(l);
            self.right.push(r);
            self.short_mid.push_back(m);
            while self.short_mid.len() > self.short_needed {
                self.short_mid.pop_front();
            }
        }
        if self.mid.len() < self.window_needed {
            return None;
        }
        Some(self.finish_frame())
    }

    fn finish_frame(&mut self) -> Frame {
        // 先頭 window_needed 点のみ消費し、剰余は次窓へ繰越。
        // FFT長は常に一定のためRT中の再計画は発生しない。
        let n = self.window_needed;
        let (mid, left, right) = (&self.mid[..n], &self.left[..n], &self.right[..n]);
        let rms_lin = dsp::rms_of(mid);
        let rms_db = dsp::lin_to_db(rms_lin);
        let peak_db = dsp::lin_to_db(self.peak_lin);
        let crest = dsp::crest_factor(rms_db, peak_db);
        let (short_a, short_b) = self.short_mid.as_slices();
        // 線形RMSは二乗平均のため2スライスの合算で等価。
        let short_lin = {
            let m = (short_a.len() + short_b.len()) as f32;
            if m == 0.0 {
                0.0
            } else {
                let sum: f32 = short_a.iter().chain(short_b.iter()).map(|s| s * s).sum();
                (sum / m).sqrt()
            }
        };
        let transient = dsp::transient_index(short_lin, rms_lin);
        let stereo_width = dsp::stereo_correlation(left, right);
        let bands = self.fft.analyze(mid);

        self.mid.drain(..n);
        self.left.drain(..n);
        self.right.drain(..n);
        self.peak_lin = self.mid.iter().fold(0.0f32, |m, s| m.max(s.abs()));
        // short_mid は意図的に残す (次窓の立上り検出用)。

        Frame {
            rms_db,
            peak_db,
            crest,
            bands,
            transient,
            stereo_width,
        }
    }
}
