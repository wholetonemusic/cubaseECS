//! DSPコアの単体テスト (Cubase不要)。

use cubase_analyzer::agg::Aggregator;
use cubase_analyzer::dsp::{
    crest_factor, lin_to_db, peak_db, rms_db, stereo_correlation, transient_index, FftBands,
    FLOOR_DB,
};
use cubase_analyzer::features::{AudioFeatures, SHM_SIZE};

fn sine(freq: f32, sr: f32, n: usize, amp: f32) -> Vec<f32> {
    (0..n)
        .map(|i| amp * (2.0 * std::f32::consts::PI * freq * i as f32 / sr).sin())
        .collect()
}

#[test]
fn silence_hits_floor() {
    let z = vec![0.0f32; 1024];
    assert_eq!(rms_db(&z), FLOOR_DB);
    assert_eq!(peak_db(&z), FLOOR_DB);
}

#[test]
fn sine_levels_match_theory() {
    // 振幅1.0正弦波: RMS=-3.01dB, Peak=0dB, Crest=3.01dB。
    let s = sine(1000.0, 48000.0, 4800, 1.0);
    let rms = rms_db(&s);
    let peak = peak_db(&s);
    assert!((rms + 3.01).abs() < 0.05, "rms={rms}");
    assert!(peak.abs() < 0.01, "peak={peak}");
    let crest = crest_factor(rms, peak);
    assert!((crest - 3.01).abs() < 0.06, "crest={crest}");
}

#[test]
fn low_sine_dominates_low_band() {
    let sr = 48000.0;
    let s = sine(100.0, sr, 4800, 0.5);
    let mut fft = FftBands::new(4800, sr);
    let b = fft.analyze(&s);
    assert!(b.low > b.mid + 10.0, "low={} mid={}", b.low, b.mid);
    assert!(b.low > b.high + 10.0, "low={} high={}", b.low, b.high);
}

#[test]
fn high_sine_dominates_high_band() {
    let sr = 48000.0;
    let s = sine(10000.0, sr, 4800, 0.5);
    let mut fft = FftBands::new(4800, sr);
    let b = fft.analyze(&s);
    assert!(b.high > b.low + 10.0, "high={} low={}", b.high, b.low);
    assert!(
        b.high > b.lowmid + 10.0,
        "high={} lowmid={}",
        b.high,
        b.lowmid
    );
}

#[test]
fn stereo_correlation_semantics() {
    let l = sine(440.0, 48000.0, 1024, 0.5);
    let mut r = l.clone();
    assert!((stereo_correlation(&l, &r) - 1.0).abs() < 1e-5);
    for s in r.iter_mut() {
        *s = -*s;
    }
    assert!((stereo_correlation(&l, &r) + 1.0).abs() < 1e-5);
    assert_eq!(stereo_correlation(&[], &[]), 1.0);
    assert_eq!(stereo_correlation(&[0.0; 8], &[0.0; 8]), 1.0);
}

#[test]
fn transient_index_is_abs_diff() {
    assert_eq!(transient_index(0.5, 0.5), 0.0);
    assert!((transient_index(0.7, 0.3) - 0.4).abs() < 1e-6);
}

#[test]
fn lin_to_db_floor() {
    assert_eq!(lin_to_db(0.0), FLOOR_DB);
    assert_eq!(lin_to_db(-1.0), FLOOR_DB);
    assert!((lin_to_db(1.0)).abs() < 1e-6);
}

#[test]
fn aggregator_emits_10hz_frames() {
    // 48kHz / 128sampleブロック → 4800点窓は38ブロック目で満了。
    let sr = 48000.0;
    let mut agg = Aggregator::new(sr);
    let block = sine(440.0, sr, 128, 0.5);
    let mut frames = 0;
    for i in 0..80 {
        if let Some(f) = agg.push(&block, None) {
            frames += 1;
            // Monoは幅1.0。
            assert!((f.stereo_width - 1.0).abs() < 1e-5);
            assert!((f.rms_db + 9.03).abs() < 0.3, "rms={}", f.rms_db);
            let _ = i;
        }
    }
    // 80*128=10240点 → 2窓分 (4800*2=9600) + 残640。
    assert_eq!(frames, 2);
}

#[test]
fn aggregator_stereo_width_detects_wide() {
    let sr = 48000.0;
    let mut agg = Aggregator::new(sr);
    let l = sine(440.0, sr, 128, 0.5);
    let r: Vec<f32> = l.iter().map(|s| -s).collect();
    let mut last = None;
    for _ in 0..40 {
        if let Some(f) = agg.push(&l, Some(&r)) {
            last = Some(f);
        }
    }
    let f = last.expect("frame");
    // 逆相 → 相関-1 (Mid無音でも幅は検出)。
    assert!((f.stereo_width + 1.0).abs() < 1e-4, "w={}", f.stereo_width);
}

#[test]
fn features_struct_is_1k_and_json_shaped() {
    assert_eq!(std::mem::size_of::<AudioFeatures>(), SHM_SIZE);
    let mut f = AudioFeatures::empty();
    assert!(f.is_valid());
    f.rms = -18.5;
    f.timestamp_ms = 1000;
    let v = f.to_json_value(1100);
    assert_eq!(v["rms"], -18.5);
    // empty() の帯域は -inf → JSONでは -120.0 に丸められる。
    assert_eq!(v["bands"]["low"], -120.0);
    assert_eq!(v["stale"], false);
    // 500ms超でstale。
    let v2 = f.to_json_value(1601);
    assert_eq!(v2["stale"], true);
}
