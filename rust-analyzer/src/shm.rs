//! 共有メモリ transport (spec §4)。
//!
//! - OS: Windows の名前付き共有メモリ (`CreateFileMapping` 系)。
//!   `shared_memory` クレートで抽象化 (spec §4.1の第一候補)。
//!   ※ `memmap2` はWindowsで名前付きページファイルマッピングを作れないため
//!   不採用。ファイルバッキング案よりAI側のパス結合が不要で堅牢。
//! - 排他制御は Mutex ではなく seqlock (spec §7の lock-free 推奨を採用)。
//!   Writer単一 (VST3) / Reader複数 (rust-api)。RTスレッドでロックしない。
//! - AI側は読み取り専用。

use shared_memory::{Shmem, ShmemConf, ShmemError};

use crate::features::{AudioFeatures, SHM_OS_ID, SHM_SIZE};

/// VST3側 (Writer)。既存マッピングがあれば開く (多重起動・再起動対応)。
pub fn create_or_open_writer() -> Result<Shmem, ShmemError> {
    match ShmemConf::new().size(SHM_SIZE).os_id(SHM_OS_ID).create() {
        Ok(shmem) => {
            // 新規作成時は0初期化して未初期化読取を防ぐ。
            unsafe {
                std::ptr::write_bytes(shmem.as_ptr(), 0, shmem.len());
            }
            Ok(shmem)
        }
        Err(ShmemError::MappingIdExists) => ShmemConf::new().size(SHM_SIZE).os_id(SHM_OS_ID).open(),
        Err(e) => Err(e),
    }
}

/// AI / rust-api側 (Reader)。存在しなければ Err。
pub fn open_reader() -> Result<Shmem, ShmemError> {
    ShmemConf::new().size(SHM_SIZE).os_id(SHM_OS_ID).open()
}

pub fn mapping_exists() -> bool {
    open_reader().is_ok()
}

/// seqlock書込。`&mut` 不要 (生ポインタ経由のvolatile書込)。
/// RTスレッドから呼ぶ: 確保・ロック・I/Oなし。
///
/// # Safety
/// - `shmem` は `SHM_SIZE` 以上の有効なマッピングであること。
/// - Writerは単一であること (VST3インスタンスは選択トラック単独運用)。
pub unsafe fn write_features(shmem: &Shmem, f: &AudioFeatures) {
    debug_assert!(shmem.len() >= SHM_SIZE);
    let base = shmem.as_ptr() as *mut AudioFeatures;
    // seqの現在値を読む (初回0 → 1から開始)。
    let cur = std::ptr::read_volatile(std::ptr::addr_of!((*base).seq));
    let next = cur.wrapping_add(1) | 1; // odd = 書込中
    std::ptr::write_volatile(std::ptr::addr_of_mut!((*base).seq), next);
    std::ptr::write_volatile(base, AudioFeatures { seq: next, ..*f });
    std::ptr::write_volatile(
        std::ptr::addr_of_mut!((*base).seq),
        next.wrapping_add(1), // even = 確定
    );
}

/// seqlock読取。撕裂・未初期化時は `None` (最大3回リトライ)。
///
/// # Safety
/// - `shmem` は `SHM_SIZE` 以上の有効なマッピングであること。
pub unsafe fn read_features(shmem: &Shmem) -> Option<AudioFeatures> {
    if shmem.len() < SHM_SIZE {
        return None;
    }
    let base = shmem.as_ptr() as *const AudioFeatures;
    for _ in 0..3 {
        let s1 = std::ptr::read_volatile(std::ptr::addr_of!((*base).seq));
        if s1 & 1 == 1 {
            continue; // 書込中
        }
        let copy = std::ptr::read_volatile(base);
        let s2 = std::ptr::read_volatile(std::ptr::addr_of!((*base).seq));
        if s1 == s2 && copy.is_valid() {
            return Some(copy);
        }
    }
    None
}
