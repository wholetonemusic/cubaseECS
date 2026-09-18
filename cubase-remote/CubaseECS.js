// CubaseECS.js — Cubase MIDI Remote driver for cubaseECS (Phase 2)
// Target: Cubase Artist 15 / MIDI Remote API v1.x. ES5 only.
// Rust API -> SysEx F0 7D <ASCII JSON> F7 -> this script -> Cubase.
//
// Value semantics (Cubase実API合わせ):
// - mixer volume は Cubase 内部では 0..1 正規化値。本スクリプトは dB を受けて
//   近似変換する (param "volume": dB, "volume01": 0..1 直値)。
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

// ---- hidden surface actuators (transport) ----
var surface = deviceDriver.mSurface
var transportPlay = surface.makeCustomValueVariable('ecsTransportPlay')
var transportStop = surface.makeCustomValueVariable('ecsTransportStop')
var transportRecord = surface.makeCustomValueVariable('ecsTransportRecord')

var page = deviceDriver.mMapping.makePage('ECS')
page.makeValueBinding(transportPlay, page.mHostAccess.mTransport.mValue.mStart)
page.makeValueBinding(transportStop, page.mHostAccess.mTransport.mValue.mStop)
page.makeValueBinding(transportRecord, page.mHostAccess.mTransport.mValue.mRecord)

// ---- DirectAccess (mixer by track name / insert params) ----
var daMixConsole = page.mHostAccess.makeDirectAccess(page.mHostAccess.mMixConsole)
var selChannel = page.mHostAccess.mTrackSelection.mMixerChannel
var insertViewer = selChannel.mInsertAndStripEffects.makeInsertEffectViewer('ecsInsertViewer')
var daInsertViewer = page.mHostAccess.makeDirectAccess(insertViewer)

var currentMapping = null
var selectedTrackTitle = ''
var lastPluginIdentity = ''

page.mOnActivate = function (activeDevice, activeMapping) {
  currentMapping = activeMapping
  daMixConsole.activate(activeMapping)
  daInsertViewer.activate(activeMapping)
}

page.mOnDeactivate = function (activeDevice, activeMapping) {
  currentMapping = null
  try { daMixConsole.deactivate(activeMapping) } catch (e) {}
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

function encodeJsonToSysEx(obj) {
  var str = JSON.stringify(obj)
  var out = [0xF0, ECS_SYSEX_ID]
  for (var i = 0; i < str.length; i++) {
    out.push(str.charCodeAt(i) & 0x7F)
  }
  out.push(0xF7)
  return out
}

// dB -> 0..1 process value (振幅則の近似。Cubaseフェーダーテーパとは微差あり)
function dbToProcess(db) {
  var v = Number(db)
  if (isNaN(v)) {
    throw new Error('volume must be a number (dB), got ' + db)
  }
  if (v <= ECS_DB_MIN) return 0
  if (v >= ECS_DB_MAX) return 1
  var proc = Math.pow(10, v / 20) / Math.pow(10, ECS_DB_MAX / 20)
  return Math.max(0, Math.min(1, proc))
}

function findChannelByTitle(da, title) {
  var want = String(title).toLowerCase()
  var baseId = da.getBaseObjectID(currentMapping)
  var n = da.getNumberOfChildObjects(currentMapping, baseId)
  var examples = []
  for (var i = 0; i < n; i++) {
    var childId = da.getChildObjectID(currentMapping, baseId, i)
    var t = ''
    try {
      t = da.getObjectTitle(currentMapping, childId)
    } catch (e) {
      continue
    }
    if (examples.length < 8) examples.push(t)
    if (String(t).toLowerCase() === want) {
      return { id: childId, title: t }
    }
  }
  throw new Error('Track not found: ' + title +
    ' (' + n + ' channels, e.g. ' + examples.join(' / ') + ')')
}

function findParamTag(da, objectId, wanted) {
  var want = String(wanted).toLowerCase()
  var count = da.getNumberOfParameters(currentMapping, objectId)
  for (var i = 0; i < count; i++) {
    var tag = da.getParameterTagByIndex(currentMapping, objectId, i)
    var name = ''
    try {
      name = da.getParameterTitle(currentMapping, objectId, tag, 128)
    } catch (e) {
      continue
    }
    if (String(name).toLowerCase().indexOf(want) !== -1) {
      return { tag: tag, name: name }
    }
  }
  throw new Error('Parameter not found: ' + wanted)
}

function requireActiveMapping() {
  if (currentMapping === null) {
    throw new Error('Mapping not active yet (page ECS not activated)')
  }
}

// ---- method implementations ----

function mixerSet(params) {
  requireActiveMapping()
  var found = findChannelByTitle(daMixConsole, params.track)
  var name = String(params.param).toLowerCase()
  var tag
  if (name === 'volume') {
    var proc = dbToProcess(params.value)
    tag = findParamTag(daMixConsole, found.id, 'volume')
    daMixConsole.setParameterProcessValue(currentMapping, found.id, tag.tag, proc)
    return { ok: true, track: found.title, param: 'volume', db: Number(params.value), process: proc }
  }
  if (name === 'volume01') {
    var direct = Number(params.value)
    if (isNaN(direct) || direct < 0 || direct > 1) {
      throw new Error('volume01 must be 0..1, got ' + params.value)
    }
    tag = findParamTag(daMixConsole, found.id, 'volume')
    daMixConsole.setParameterProcessValue(currentMapping, found.id, tag.tag, direct)
    return { ok: true, track: found.title, param: 'volume01', process: direct }
  }
  if (name === 'mute' || name === 'solo') {
    var on = (params.value === true || params.value === 1 || params.value === '1') ? 1 : 0
    tag = findParamTag(daMixConsole, found.id, name)
    daMixConsole.setParameterProcessValue(currentMapping, found.id, tag.tag, on)
    return { ok: true, track: found.title, param: name, value: on }
  }
  if (name === 'pan') {
    var pan = Number(params.value)
    if (isNaN(pan) || pan < -1 || pan > 1) {
      throw new Error('pan must be -1..1, got ' + params.value)
    }
    tag = findParamTag(daMixConsole, found.id, 'pan')
    daMixConsole.setParameterProcessValue(currentMapping, found.id, tag.tag, (pan + 1) / 2)
    return { ok: true, track: found.title, param: 'pan', value: pan }
  }
  throw new Error('Unsupported mixer param: ' + params.param + ' (volume/volume01/mute/solo/pan)')
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
      return mixerSet(json.params || {})
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
