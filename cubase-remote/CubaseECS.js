// CubaseECS.js — Cubase MIDI Remote Script 雛形 (Phase 2着手用)
// 配置: Cubase の MIDI Remote Script フォルダにコピーして使う想定
// Rust API -> SysEx(F0 7D <JSON> F7) -> このスクリプト -> Cubase API

function decodeSysExToJson(data) {
  // data: SysExバイト列。F0 7D ... F7 を想定
  if (data.length < 3 || data[0] !== 0xF0 || data[data.length - 1] !== 0xF7) {
    throw new Error('Invalid SysEx framing');
  }
  if (data[1] !== 0x7D) {
    throw new Error('Unexpected manufacturer ID');
  }
  var bytes = data.slice(2, data.length - 1);
  // NOTE: 7bit-ASCII JSONのみ対応。日本語等はRust側で事前に弾かれる仕様
  var str = String.fromCharCode.apply(null, bytes);
  return JSON.parse(str);
}

function encodeJsonToSysEx(obj) {
  var str = JSON.stringify(obj);
  var out = [0xF0, 0x7D];
  for (var i = 0; i < str.length; i++) {
    out.push(str.charCodeAt(i) & 0x7F);
  }
  out.push(0xF7);
  return out;
}

function mixerSet(params) {
  // TODO Phase2: 実機APIに合わせて実装
  // var ch = hostAccess.mMixConsole.getChannelByName(params.track);
  // if (params.param === 'volume') { ch.mVolume.setValue(params.value); }
  return { ok: true, echo: params };
}

function pluginSetParam(params) {
  // TODO Phase2
  return { ok: true, echo: params };
}

function commandExec(params) {
  // TODO Phase2: hostAccess.mApplication.performCommand(params.id);
  return { ok: true, echo: params };
}

function sessionStatus() {
  return { ok: true, connected: true };
}

function handleRpc(json) {
  switch (json.method) {
    case 'mixer.set':
      return mixerSet(json.params);
    case 'plugin.set_param':
      return pluginSetParam(json.params);
    case 'command.exec':
      return commandExec(json.params);
    case 'session.status':
      return sessionStatus();
    default:
      throw new Error('Method not found: ' + json.method);
  }
}

// Cubase MIDI Remote API 上での登録例 (環境に合わせて調整)
// midiInput.setSysexCallback(function (data) {
//   try {
//     var json = decodeSysExToJson(data);
//     var result = handleRpc(json);
//     midiOutput.sendSysex(encodeJsonToSysEx({ result: result }));
//   } catch (e) {
//     midiOutput.sendSysex(encodeJsonToSysEx({ error: String(e && e.message || e) }));
//   }
// });
