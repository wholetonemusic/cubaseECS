// CubaseECS.js — Cubase MIDI Remote driver for cubaseECS (Phase 2)
// Target: Cubase Artist 15 / MIDI Remote API v1.x. ES5 only.
// Rust API -> SysEx F0 7D <ASCII JSON> F7 -> this script -> Cubase.
//
// Value semantics (Cubase実API合わせ):
// - mixer は選択中トラックに束縛した隠しフェーダで駆動する。volume は dB を受けて
//   近似変換する (param "volume": dB, "volume01": 0..1 直値)。
//   plugin同様、対象トラックの事前選択が必須。
// - plugin.set_param の value は 0..1 プロセス値。範囲外の数値はプレーン値
//   (dB等) とみなして convertParameterPlainToProcessValue で変換を試みる。

var midiremote_api = require('midiremote_api_v1')

// loopMIDI ポート名の部分一致。'loopMIDI Port' / 'loopMIDI Port 1' の両対応。
var ECS_PORT_MATCH = 'loopMIDI Port'
var ECS_SYSEX_ID = 0x7D
var ECS_DB_MIN = -60.0
var ECS_DB_MAX = 6.0

var deviceDriver = midiremote_api.makeDeviceDriver('WholeTone', 'CubaseECS', 'cubaseECS project')

var midiInput = deviceDriver.mPorts.makeMidiInput('CubaseECS In')
var midiOutput = deviceDriver.mPorts.makeMidiOutput('CubaseECS Out')

deviceDriver.makeDetectionUnit().detectPortPair(midiInput, midiOutput)
  .expectInputNameContains(ECS_PORT_MATCH)
  .expectOutputNameContains(ECS_PORT_MATCH)

// ---- hidden surface actuators (transport + mixer on selected track) ----
// mixerは選択中トラック(selChannel)に束縛した隠しフェーダで駆動する。
// DirectAccessのトラック名解決は使わない(重複列挙・表記ゆれ・clickVolume誤爆のため)。
var surface = deviceDriver.mSurface
var transportPlay = surface.makeCustomValueVariable('ecsTransportPlay')
var transportStop = surface.makeCustomValueVariable('ecsTransportStop')
var transportRecord = surface.makeCustomValueVariable('ecsTransportRecord')

var page = deviceDriver.mMapping.makePage('ECS')
page.makeValueBinding(transportPlay, page.mHostAccess.mTransport.mValue.mStart)
page.makeValueBinding(transportStop, page.mHostAccess.mTransport.mValue.mStop)
page.makeValueBinding(transportRecord, page.mHostAccess.mTransport.mValue.mRecord)

// ---- selected-track channel (mixer) ----
var selChannel = page.mHostAccess.mTrackSelection.mMixerChannel
var mixFader = surface.makeCustomValueVariable('ecsMixFader')
var mixPan = surface.makeCustomValueVariable('ecsMixPan')
var mixMute = surface.makeCustomValueVariable('ecsMixMute')
var mixSolo = surface.makeCustomValueVariable('ecsMixSolo')
page.makeValueBinding(mixFader, selChannel.mValue.mVolume)
page.makeValueBinding(mixPan, selChannel.mValue.mPan)
page.makeValueBinding(mixMute, selChannel.mValue.mMute)
page.makeValueBinding(mixSolo, selChannel.mValue.mSolo)

// ---- DirectAccess (insert params; plugin用。mixerには使わない) ----
var insertViewer = selChannel.mInsertAndStripEffects.makeInsertEffectViewer('ecsInsertViewer')
var daInsertViewer = page.mHostAccess.makeDirectAccess(insertViewer)

var currentMapping = null
var selectedTrackTitle = ''
var lastPluginIdentity = ''

page.mOnActivate = function (activeDevice, activeMapping) {
  currentMapping = activeMapping
  daInsertViewer.activate(activeMapping)
}

page.mOnDeactivate = function (activeDevice, activeMapping) {
  currentMapping = null
  try { daInsertViewer.deactivate(activeMapping) } catch (e) {}
}

selChannel.mOnTitleChange = function (activeDevice, activeMapping, title) {
  selectedTrackTitle = title
}

insertViewer.mOnChangePluginIdentity = function (activeDevice, activeMapping, pluginName) {
  lastPluginIdentity = pluginName
}

// ---- SysEx helpers ----

function decodeSysExToJson(data) {
  var len = data.length
  if (len < 3 || data[0] !== 0xF0 || data[len - 1] !== 0xF7) {
    throw new Error('Invalid SysEx framing')
  }
  if (data[1] !== ECS_SYSEX_ID) {
    throw new Error('Unexpected manufacturer ID')
  }
  var chars = []
  for (var i = 2; i < len - 1; i++) {
    chars.push(String.fromCharCode(data[i] & 0x7F))
  }
  return JSON.parse(chars.join(''))
}

function asciiSafe(str) {
  // SysExは7bit-ASCII JSONのみ。非ASCIIは\uXXXXエスケープ(情報保持・構文安全)する。
  // 生のまま &0x7F すると制御文字化けでJSON parse不能になり応答ロストする。
  return String(str).replace(/[\u0080-\uFFFF]/g, function (c) {
    return '\\u' + ('0000' + c.charCodeAt(0).toString(16)).slice(-4)
  })
}

function encodeJsonToSysEx(obj) {
  var str = asciiSafe(JSON.stringify(obj))
  var out = [0xF0, ECS_SYSEX_ID]
  for (var i = 0; i < str.length; i++) {
    out.push(str.charCodeAt(i) & 0x7F)
  }
  out.push(0xF7)
  return out
}

// dB -> 0..1 process value (Cubase実測テーパー。2026-09-27校正:
// p=1.00→+6.02dB, 0.75→-0.81dB, 0.50→-8.31dB, 0.25119→-20.3dB。
// 前方特性 dB(p)=6.02+61.87x+64.92x^2+58.22x^3 (x=log10(p)) に二分法で逆変換)
function cubaseDbFromProcess(p) {
  if (p <= 0) return -Infinity
  var x = Math.log(p) / Math.LN10
  return 6.02 + 61.87 * x + 64.92 * x * x + 58.22 * x * x * x
}

function dbToProcess(db) {
  var v = Number(db)
  if (isNaN(v)) {
    throw new Error('volume must be a number (dB), got ' + db)
  }
  if (v <= ECS_DB_MIN) return 0
  if (v >= ECS_DB_MAX) return 1
  var lo = 0
  var hi = 1
  for (var i = 0; i < 50; i++) {
    var mid = (lo + hi) / 2
    if (cubaseDbFromProcess(mid) < v) {
      lo = mid
    } else {
      hi = mid
    }
  }
  return (lo + hi) / 2
}

function findParamTag(da, objectId, wanted) {
  var want = String(wanted).toLowerCase()
  var count = da.getNumberOfParameters(currentMapping, objectId)
  var fallback = null
  for (var i = 0; i < count; i++) {
    var tag = da.getParameterTagByIndex(currentMapping, objectId, i)
    var name = ''
    try {
      name = da.getParameterTitle(currentMapping, objectId, tag, 128)
    } catch (e) {
      continue
    }
    var lname = String(name).toLowerCase()
    // 完全一致を優先。部分一致だけだと clickVolume 等の別物に当たる。
    if (lname === want) {
      return { tag: tag, name: name }
    }
    if (!fallback && lname.indexOf(want) !== -1) {
      fallback = { tag: tag, name: name }
    }
  }
  if (fallback) return fallback
  throw new Error('Parameter not found: ' + wanted)
}

function requireActiveMapping() {
  if (currentMapping === null) {
    throw new Error('Mapping not active yet (page ECS not activated)')
  }
}

// ---- method implementations ----

function mixerSet(params, activeDevice) {
  requireActiveMapping()
  // plugin同様、対象トラック選択が前提。選択チャンネル束縛フェーダで直接駆動する。
  if (String(selectedTrackTitle).toLowerCase() !== String(params.track).toLowerCase()) {
    throw new Error('Select track "' + params.track + '" in Cubase first' +
      ' (selected: "' + selectedTrackTitle + '")')
  }
  var name = String(params.param).toLowerCase()
  var target = null
  var wantValue = 0
  if (name === 'volume') {
    wantValue = dbToProcess(params.value)
    target = mixFader
  } else if (name === 'volume01') {
    wantValue = Number(params.value)
    if (isNaN(wantValue) || wantValue < 0 || wantValue > 1) {
      throw new Error('volume01 must be 0..1, got ' + params.value)
    }
    target = mixFader
  } else if (name === 'mute' || name === 'solo') {
    wantValue = (params.value === true || params.value === 1 || params.value === '1') ? 1 : 0
    target = (name === 'mute') ? mixMute : mixSolo
  } else if (name === 'pan') {
    var pan = Number(params.value)
    if (isNaN(pan) || pan < -1 || pan > 1) {
      throw new Error('pan must be -1..1, got ' + params.value)
    }
    wantValue = (pan + 1) / 2
    target = mixPan
  } else {
    throw new Error('Unsupported mixer param: ' + params.param + ' (volume/volume01/mute/solo/pan)')
  }
  target.setProcessValue(activeDevice, wantValue)
  var result = { ok: true, track: selectedTrackTitle, param: name, via: 'selected-track' }
  if (name === 'volume') {
    result.db = Number(params.value)
    result.process = wantValue
  } else if (name === 'volume01') {
    result.process = wantValue
  } else {
    result.value = wantValue
  }
  return result
}

function pluginSetParam(params) {
  requireActiveMapping()
  if (typeof params.slot !== 'number') {
    throw new Error('plugin.set_param needs numeric slot, got ' + params.slot)
  }
  // Phase 2 制約: 対象トラックを選択状態にしておくこと
  if (String(selectedTrackTitle).toLowerCase() !== String(params.track).toLowerCase()) {
    throw new Error('Select track "' + params.track + '" in Cubase first' +
      ' (Phase 2 limitation; selected: "' + selectedTrackTitle + '")')
  }
  insertViewer.accessSlotAtIndex(params.slot)
  var slotId = insertViewer.getRuntimeID(currentMapping)
  if (slotId < 0) {
    throw new Error('No insert slot ' + params.slot + ' on selected track')
  }
  var pluginId = daInsertViewer.getChildObjectID(currentMapping, slotId, 0)
  if (params.plugin && lastPluginIdentity &&
      String(lastPluginIdentity).toLowerCase().indexOf(String(params.plugin).toLowerCase()) === -1) {
    throw new Error('Plugin mismatch in slot ' + params.slot +
      ': expected "' + params.plugin + '", found "' + lastPluginIdentity + '"')
  }
  var found = findParamTag(daInsertViewer, pluginId, params.param)
  var proc
  if (typeof params.value === 'number' && params.value >= 0 && params.value <= 1) {
    proc = params.value
  } else if (typeof daInsertViewer.convertParameterPlainToProcessValue === 'function') {
    proc = daInsertViewer.convertParameterPlainToProcessValue(
      currentMapping, pluginId, found.tag, params.value)
  } else {
    throw new Error('Value must be 0..1 process value on this Cubase version (got ' + params.value + ')')
  }
  daInsertViewer.setParameterProcessValue(currentMapping, pluginId, found.tag, proc)
  var result = { ok: true, track: params.track, slot: params.slot, param: found.name, process: proc }
  if (params.plugin) result.plugin = params.plugin
  return result
}

var TRANSPORT_IDS = {
  Transport_Play: 'play',
  TransportPlay: 'play',
  play: 'play',
  Transport_Stop: 'stop',
  TransportStop: 'stop',
  stop: 'stop',
  Transport_Record: 'record',
  TransportRecord: 'record',
  record: 'record'
}

function commandExec(params, activeDevice) {
  var action = TRANSPORT_IDS[params.id]
  if (!action) {
    throw new Error('Unsupported command id: ' + params.id +
      ' (Transport_Play/Transport_Stop/Transport_Record)')
  }
  if (action === 'play') transportPlay.setProcessValue(activeDevice, 1)
  else if (action === 'stop') transportStop.setProcessValue(activeDevice, 1)
  else transportRecord.setProcessValue(activeDevice, 1)
  return { ok: true, id: params.id }
}

function sessionStatus() {
  return { ok: true, connected: true, selectedTrack: selectedTrackTitle }
}

function handleRpc(json, activeDevice) {
  switch (json.method) {
    case 'mixer.set':
      return mixerSet(json.params || {}, activeDevice)
    case 'plugin.set_param':
      return pluginSetParam(json.params || {})
    case 'command.exec':
      return commandExec(json.params || {}, activeDevice)
    case 'session.status':
      return sessionStatus()
    default:
      throw new Error('Method not found: ' + json.method)
  }
}

midiInput.mOnSysex = function (activeDevice, sysexMsg) {
  var req = null
  try {
    req = decodeSysExToJson(sysexMsg)
    // loopMIDI反響(自送信応答の跳ね返り: methodなし)は無視する。
    // 応答しないと Rust側のid相関を誤作動させる。
    if (!req || typeof req.method !== 'string') {
      return
    }
    var result = handleRpc(req, activeDevice)
    midiOutput.sendMidi(activeDevice, encodeJsonToSysEx({
      jsonrpc: '2.0',
      id: (req.id === undefined ? null : req.id),
      result: result
    }))
  } catch (e) {
    midiOutput.sendMidi(activeDevice, encodeJsonToSysEx({
      jsonrpc: '2.0',
      id: (req && req.id !== undefined ? req.id : null),
      result: { ok: false, error: String((e && e.message) || e) }
    }))
  }
}
