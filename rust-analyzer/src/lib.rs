//! AI Audio Analyzer — 解析専用VST3プラグイン (spec §3)。
//!
//! - Plugin ID: `com.yoshiki.AudioAnalyzer` / Display: `AI Audio Analyzer`。
//! - オーディオは素通し (加工なし)。解析結果のみ共有メモリへ書込む。
//! - VST3バインディングは `vst3` クレート直結 (nih-plugはcrates.io未提供のため)。
//! - RT適合: `process` 内で確保・ロック (他スレッド競合なしのMutexのみ)・
//!   I/Oなし。FFTは10Hz満了時のみ、事前計画済み。
//!
//! 運用制約 (cubaseECS親和性): 選択トラック単独運用。
//! 複数インスタンスは同一マッピングへ上書きする (Phase 1仕様)。

#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use std::ffi::{c_char, c_void};
use std::sync::Mutex;
use std::{ptr, slice};

use shared_memory::Shmem;
use vst3::{uid, Class, ComWrapper, Steinberg::Vst::*, Steinberg::*};

pub mod agg;
pub mod dsp;
pub mod features;
pub mod shm;

use agg::{Aggregator, Frame};
use features::{now_ms, AudioFeatures, FEATURES_MAGIC, FEATURES_VERSION};

fn copy_cstring(src: &str, dst: &mut [c_char]) {
    let bytes = src.as_bytes();
    let n = bytes.len().min(dst.len().saturating_sub(1));
    for (i, b) in bytes.iter().take(n).enumerate() {
        dst[i] = *b as c_char;
    }
    if let Some(last) = dst.get_mut(n) {
        *last = 0;
    }
}

fn copy_wstring(src: &str, dst: &mut [TChar]) {
    let mut len = 0;
    for (s, d) in src.encode_utf16().zip(dst.iter_mut()) {
        *d = s as TChar;
        len += 1;
    }
    if len < dst.len() {
        dst[len] = 0;
    } else if let Some(last) = dst.last_mut() {
        *last = 0;
    }
}

pub const PLUGIN_NAME: &str = "AI Audio Analyzer";
pub const PLUGIN_VENDOR: &str = "WholeTone";
pub const PLUGIN_SUBCATEGORIES: &str = "Fx|Analyzer";

/// `Shmem` は生ポインタ保持のためSend/Syncでない。
/// 単一Writer (本プラグイン) + seqlock手順でのみ触るため安全。
/// SAFETY: 書込は `shm::write_features` のseqlock手順に限定し、
/// `ProcessorInner` のMutex配下でのみアクセスする。
struct ShmMapping(Shmem);
unsafe impl Send for ShmMapping {}
unsafe impl Sync for ShmMapping {}

struct ProcessorInner {
    agg: Aggregator,
    shm: Option<ShmMapping>,
}

struct AnalyzerProcessor {
    inner: Mutex<ProcessorInner>,
}

impl Class for AnalyzerProcessor {
    type Interfaces = (IComponent, IAudioProcessor, IProcessContextRequirements);
}

impl AnalyzerProcessor {
    const CID: TUID = uid(0xC7A3E5B1, 0x9D2F4A6C, 0xB8D4E6F0, 0x1A2B3C4D);

    fn new() -> AnalyzerProcessor {
        AnalyzerProcessor {
            inner: Mutex::new(ProcessorInner {
                agg: Aggregator::new(48000.0),
                shm: None,
            }),
        }
    }

    fn on_frame(shm: &Shmem, frame: &Frame) {
        let f = AudioFeatures {
            magic: FEATURES_MAGIC,
            version: FEATURES_VERSION,
            _reserved: 0,
            seq: 0, // write_featuresが設定
            rms: frame.rms_db,
            peak: frame.peak_db,
            crest_factor: frame.crest,
            band_low: frame.bands.low,
            band_lowmid: frame.bands.lowmid,
            band_mid: frame.bands.mid,
            band_highmid: frame.bands.highmid,
            band_high: frame.bands.high,
            transient: frame.transient,
            stereo_width: frame.stereo_width,
            timestamp_ms: now_ms(),
            _pad: [0; 960],
        };
        unsafe {
            shm::write_features(shm, &f);
        }
    }
}

impl IPluginBaseTrait for AnalyzerProcessor {
    unsafe fn initialize(&self, _context: *mut FUnknown) -> tresult {
        // 共有メモリが開けなくても音は止めない (解析のみ欠落)。
        match shm::create_or_open_writer() {
            Ok(mapping) => {
                if let Ok(mut inner) = self.inner.lock() {
                    inner.shm = Some(ShmMapping(mapping));
                }
            }
            Err(_) => {
                if let Ok(mut inner) = self.inner.lock() {
                    inner.shm = None;
                }
            }
        }
        kResultOk
    }

    unsafe fn terminate(&self) -> tresult {
        if let Ok(mut inner) = self.inner.lock() {
            inner.shm = None;
        }
        kResultOk
    }
}

impl IComponentTrait for AnalyzerProcessor {
    unsafe fn getControllerClassId(&self, class_id: *mut TUID) -> tresult {
        *class_id = AnalyzerController::CID;
        kResultOk
    }

    unsafe fn setIoMode(&self, _mode: IoMode) -> tresult {
        kResultOk
    }

    unsafe fn getBusCount(&self, mediaType: MediaType, dir: BusDirection) -> i32 {
        match mediaType as BusDirections {
            MediaTypes_::kAudio => match dir as BusDirections {
                BusDirections_::kInput => 1,
                BusDirections_::kOutput => 1,
                _ => 0,
            },
            MediaTypes_::kEvent => 0,
            _ => 0,
        }
    }

    #[allow(clippy::unnecessary_cast)]
    unsafe fn getBusInfo(
        &self,
        mediaType: MediaType,
        dir: BusDirection,
        index: i32,
        bus: *mut BusInfo,
    ) -> tresult {
        if index != 0 {
            return kInvalidArgument;
        }
        match mediaType as MediaTypes {
            MediaTypes_::kAudio => match dir as BusDirections {
                BusDirections_::kInput | BusDirections_::kOutput => {
                    let bus = &mut *bus;
                    bus.mediaType = MediaTypes_::kAudio as MediaType;
                    bus.direction = dir;
                    bus.channelCount = 2;
                    copy_wstring(
                        if dir as BusDirections == BusDirections_::kInput {
                            "Input"
                        } else {
                            "Output"
                        },
                        &mut bus.name,
                    );
                    bus.busType = BusTypes_::kMain as BusType;
                    bus.flags = BusInfo_::BusFlags_::kDefaultActive as u32;
                    kResultOk
                }
                _ => kInvalidArgument,
            },
            MediaTypes_::kEvent => kInvalidArgument,
            _ => kInvalidArgument,
        }
    }

    unsafe fn getRoutingInfo(
        &self,
        _in_info: *mut RoutingInfo,
        _out_info: *mut RoutingInfo,
    ) -> tresult {
        kNotImplemented
    }

    unsafe fn activateBus(
        &self,
        _media_type: MediaType,
        _dir: BusDirection,
        _index: i32,
        _state: TBool,
    ) -> tresult {
        kResultOk
    }

    unsafe fn setActive(&self, _state: TBool) -> tresult {
        kResultOk
    }

    unsafe fn setState(&self, _state: *mut IBStream) -> tresult {
        kResultOk
    }

    unsafe fn getState(&self, _state: *mut IBStream) -> tresult {
        kResultOk
    }
}

fn is_mono_or_stereo(arr: SpeakerArrangement) -> bool {
    arr == SpeakerArr::kMono || arr == SpeakerArr::kStereo
}

impl IAudioProcessorTrait for AnalyzerProcessor {
    unsafe fn setBusArrangements(
        &self,
        inputs: *mut SpeakerArrangement,
        num_ins: i32,
        outputs: *mut SpeakerArrangement,
        num_outs: i32,
    ) -> tresult {
        if num_ins != 1 || num_outs != 1 {
            return kResultFalse;
        }
        // Mono / Stereo を受付け、入出力一致のみ通す。
        if *inputs == *outputs && is_mono_or_stereo(*inputs) {
            kResultTrue
        } else {
            kResultFalse
        }
    }

    unsafe fn getBusArrangement(
        &self,
        dir: BusDirection,
        index: i32,
        arr: *mut SpeakerArrangement,
    ) -> tresult {
        match dir as BusDirections {
            BusDirections_::kInput | BusDirections_::kOutput => {
                if index == 0 {
                    *arr = SpeakerArr::kStereo;
                    kResultOk
                } else {
                    kInvalidArgument
                }
            }
            _ => kInvalidArgument,
        }
    }

    unsafe fn canProcessSampleSize(&self, symbolic_sample_size: i32) -> tresult {
        match symbolic_sample_size as SymbolicSampleSizes {
            SymbolicSampleSizes_::kSample32 => kResultOk,
            SymbolicSampleSizes_::kSample64 => kNotImplemented,
            _ => kInvalidArgument,
        }
    }

    unsafe fn getLatencySamples(&self) -> u32 {
        0
    }

    unsafe fn setupProcessing(&self, setup: *mut ProcessSetup) -> tresult {
        let sample_rate = (*setup).sampleRate as f32;
        if sample_rate > 0.0 {
            if let Ok(mut inner) = self.inner.lock() {
                if (sample_rate - inner.agg.sample_rate()).abs() > f32::EPSILON {
                    inner.agg = Aggregator::new(sample_rate);
                }
            }
        }
        kResultOk
    }

    unsafe fn setProcessing(&self, _state: TBool) -> tresult {
        kResultOk
    }

    unsafe fn process(&self, data: *mut ProcessData) -> tresult {
        let process_data = &*data;
        let num_samples = process_data.numSamples as usize;

        if process_data.numInputs < 1 || process_data.numOutputs < 1 || num_samples == 0 {
            return kResultOk;
        }

        let input_buses =
            slice::from_raw_parts(process_data.inputs, process_data.numInputs as usize);
        let output_buses =
            slice::from_raw_parts(process_data.outputs, process_data.numOutputs as usize);
        let num_channels = input_buses[0]
            .numChannels
            .min(output_buses[0].numChannels)
            .min(2);
        if num_channels < 1 {
            return kResultOk;
        }

        let input_channels = slice::from_raw_parts(
            input_buses[0].__field0.channelBuffers32,
            input_buses[0].numChannels as usize,
        );
        let output_channels = slice::from_raw_parts_mut(
            output_buses[0].__field0.channelBuffers32,
            output_buses[0].numChannels as usize,
        );

        // 素通しコピー + 解析投入。
        // in-place (入出力同一バッファ) でも安全な順序でコピーする。
        let ch = num_channels as usize;
        // まず解析用に読む (コピー前に参照を確定)。
        let in_l = slice::from_raw_parts(input_channels[0], num_samples);
        // 右chはMono時 None。
        let frame: Option<Frame> = {
            let guard = self.inner.lock();
            match guard {
                Ok(mut inner) => {
                    // 素通し。
                    for c in 0..ch {
                        let src = slice::from_raw_parts(input_channels[c], num_samples);
                        let dst = slice::from_raw_parts_mut(output_channels[c], num_samples);
                        dst.copy_from_slice(src);
                    }
                    if ch == 2 {
                        let in_r = slice::from_raw_parts(input_channels[1], num_samples);
                        inner.agg.push(in_l, Some(in_r))
                    } else {
                        inner.agg.push(in_l, None)
                    }
                }
                Err(_) => None,
            }
        };
        if let Some(frame) = frame {
            if let Ok(inner) = self.inner.lock() {
                if let Some(mapping) = inner.shm.as_ref() {
                    Self::on_frame(&mapping.0, &frame);
                }
            }
        }
        kResultOk
    }

    unsafe fn getTailSamples(&self) -> u32 {
        0
    }
}

impl IProcessContextRequirementsTrait for AnalyzerProcessor {
    unsafe fn getProcessContextRequirements(&self) -> u32 {
        0
    }
}

/// パラメータなしコントローラ (Analyzerのため編集UIなし)。
struct AnalyzerController {}

impl Class for AnalyzerController {
    type Interfaces = (IEditController,);
}

impl AnalyzerController {
    const CID: TUID = uid(0xD8B4F6C2, 0xAE3F5B7D, 0xC9E5F7A1, 0x2B3C4D5E);

    fn new() -> AnalyzerController {
        AnalyzerController {}
    }
}

impl IPluginBaseTrait for AnalyzerController {
    unsafe fn initialize(&self, _context: *mut FUnknown) -> tresult {
        kResultOk
    }

    unsafe fn terminate(&self) -> tresult {
        kResultOk
    }
}

impl IEditControllerTrait for AnalyzerController {
    unsafe fn setComponentState(&self, _state: *mut IBStream) -> tresult {
        kNotImplemented
    }

    unsafe fn setState(&self, _state: *mut IBStream) -> tresult {
        kResultOk
    }

    unsafe fn getState(&self, _state: *mut IBStream) -> tresult {
        kResultOk
    }

    unsafe fn getParameterCount(&self) -> i32 {
        0
    }

    unsafe fn getParameterInfo(&self, _param_index: i32, _info: *mut ParameterInfo) -> tresult {
        kInvalidArgument
    }

    unsafe fn getParamStringByValue(
        &self,
        _id: u32,
        _value_normalized: f64,
        _string: *mut String128,
    ) -> tresult {
        kInvalidArgument
    }

    unsafe fn getParamValueByString(
        &self,
        _id: u32,
        _string: *mut TChar,
        _value_normalized: *mut f64,
    ) -> tresult {
        kInvalidArgument
    }

    unsafe fn normalizedParamToPlain(&self, _id: u32, _value_normalized: f64) -> f64 {
        0.0
    }

    unsafe fn plainParamToNormalized(&self, _id: u32, _plain_value: f64) -> f64 {
        0.0
    }

    unsafe fn getParamNormalized(&self, _id: u32) -> f64 {
        0.0
    }

    unsafe fn setParamNormalized(&self, _id: u32, _value: f64) -> tresult {
        kInvalidArgument
    }

    unsafe fn setComponentHandler(&self, _handler: *mut IComponentHandler) -> tresult {
        kResultOk
    }

    unsafe fn createView(&self, _name: *const c_char) -> *mut IPlugView {
        ptr::null_mut()
    }
}

struct Factory {}

impl Class for Factory {
    type Interfaces = (IPluginFactory2,);
}

impl IPluginFactoryTrait for Factory {
    unsafe fn getFactoryInfo(&self, info: *mut PFactoryInfo) -> tresult {
        let info = &mut *info;
        copy_cstring(PLUGIN_VENDOR, &mut info.vendor);
        copy_cstring("https://github.com/wholetonemusic/cubaseECS", &mut info.url);
        copy_cstring("cubaseECS project", &mut info.email);
        info.flags = PFactoryInfo_::FactoryFlags_::kUnicode as int32;
        kResultOk
    }

    unsafe fn countClasses(&self) -> i32 {
        2
    }

    unsafe fn getClassInfo(&self, index: i32, info: *mut PClassInfo) -> tresult {
        let (cid, category) = match index {
            0 => (AnalyzerProcessor::CID, "Audio Module Class"),
            1 => (AnalyzerController::CID, "Component Controller Class"),
            _ => return kInvalidArgument,
        };
        let info = &mut *info;
        info.cid = cid;
        info.cardinality = PClassInfo_::ClassCardinality_::kManyInstances as int32;
        copy_cstring(category, &mut info.category);
        copy_cstring(PLUGIN_NAME, &mut info.name);
        kResultOk
    }

    unsafe fn createInstance(
        &self,
        cid: FIDString,
        iid: FIDString,
        obj: *mut *mut c_void,
    ) -> tresult {
        let instance = match *(cid as *const TUID) {
            AnalyzerProcessor::CID => Some(
                ComWrapper::new(AnalyzerProcessor::new())
                    .to_com_ptr::<FUnknown>()
                    .unwrap(),
            ),
            AnalyzerController::CID => Some(
                ComWrapper::new(AnalyzerController::new())
                    .to_com_ptr::<FUnknown>()
                    .unwrap(),
            ),
            _ => None,
        };
        if let Some(instance) = instance {
            let ptr = instance.as_ptr();
            ((*(*ptr).vtbl).queryInterface)(ptr, iid as *mut TUID, obj)
        } else {
            kInvalidArgument
        }
    }
}

impl IPluginFactory2Trait for Factory {
    unsafe fn getClassInfo2(&self, index: i32, info: *mut PClassInfo2) -> tresult {
        let (cid, category, sub) = match index {
            0 => (
                AnalyzerProcessor::CID,
                "Audio Module Class",
                PLUGIN_SUBCATEGORIES,
            ),
            1 => (
                AnalyzerController::CID,
                "Component Controller Class",
                "Analyzer",
            ),
            _ => return kInvalidArgument,
        };
        let info = &mut *info;
        info.cid = cid;
        info.cardinality = PClassInfo_::ClassCardinality_::kManyInstances as int32;
        copy_cstring(category, &mut info.category);
        copy_cstring(PLUGIN_NAME, &mut info.name);
        info.classFlags = 0;
        copy_cstring(sub, &mut info.subCategories);
        copy_cstring(PLUGIN_VENDOR, &mut info.vendor);
        copy_cstring("1.0.0", &mut info.version);
        copy_cstring("VST 3.7", &mut info.sdkVersion);
        kResultOk
    }
}

#[cfg(target_os = "windows")]
#[no_mangle]
extern "system" fn InitDll() -> bool {
    true
}

#[cfg(target_os = "windows")]
#[no_mangle]
extern "system" fn ExitDll() -> bool {
    true
}

#[no_mangle]
extern "system" fn GetPluginFactory() -> *mut IPluginFactory {
    ComWrapper::new(Factory {})
        .to_com_ptr::<IPluginFactory>()
        .unwrap()
        .into_raw()
}
