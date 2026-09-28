//! 共有メモリ往復テスト (実OSマッピング使用)。
//!
//! 注意: 開発機でVST3が稼働中の場合、同一マッピングへ並行書込が
//! 発生し得る。タイムスタンプ一致までリトライして自書込を確認する。

use cubase_analyzer::features::{now_ms, AudioFeatures, FEATURES_MAGIC, FEATURES_VERSION};
use cubase_analyzer::shm::{create_or_open_writer, open_reader, read_features, write_features};

#[test]
fn shm_roundtrip_seqlock() {
    let shm = create_or_open_writer().expect("create writer");
    assert!(shm.len() >= 1024);

    let mut f = AudioFeatures::empty();
    f.rms = -18.5;
    f.peak = -3.2;
    f.crest_factor = 15.3;
    f.band_low = -30.0;
    f.band_lowmid = -22.0;
    f.band_mid = -18.0;
    f.band_highmid = -16.0;
    f.band_high = -15.5;
    f.transient = 0.35;
    f.stereo_width = 0.1;
    // 他Writerと区別するための一意なタイムスタンプ。
    f.timestamp_ms = now_ms().max(1) ^ 0x5f5f_0000;

    unsafe {
        write_features(&shm, &f);
    }

    let reader = open_reader().expect("open reader");
    let mut back = None;
    for _ in 0..50 {
        let got = unsafe { read_features(&reader) };
        if let Some(g) = got {
            if g.timestamp_ms == f.timestamp_ms {
                back = Some(g);
                break;
            }
        }
        std::thread::sleep(std::time::Duration::from_millis(10));
    }
    let back = back.expect("read back own frame");
    assert_eq!(back.magic, FEATURES_MAGIC);
    assert_eq!(back.version, FEATURES_VERSION);
    assert_eq!(back.seq & 1, 0, "seq must be even (stable)");
    assert!((back.rms - -18.5).abs() < 1e-6);
    assert!((back.band_high - -15.5).abs() < 1e-6);
    assert!((back.transient - 0.35).abs() < 1e-6);

    // JSON形 (spec §4.4) へ変換できる (f32精度で比較)。
    let v = back.to_json_value(now_ms());
    let peak = v["peak"].as_f64().unwrap();
    assert!((peak - -3.2).abs() < 1e-5, "peak={peak}");
    let mid = v["bands"]["mid"].as_f64().unwrap();
    assert!((mid - -18.0).abs() < 1e-5, "mid={mid}");
}
