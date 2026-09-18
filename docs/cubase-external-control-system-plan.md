# 📘 Cubase External Control System  
**自然言語で Cubase を操作する外部APIシステム（OpenCode対応）**

このプロジェクトは、OpenCode の自然言語チャット UI を使い、  
ユーザーが「話すだけ」で Cubase のミキサー・プラグイン・トランスポートを操作できる  
外部APIシステムを構築するためのものです。

---

## 🏗️ システム構成

```
OpenCode Chat UI（自然言語）
        ↓
LLM: NL → JSON-RPC Command
        ↓
Rust JSON-RPC API Server
        ↓ SysEx(JSON)
Cubase MIDI Remote Script
        ↓
Cubase Mixer / Plugins / Commands
```

---

# 1. OpenCode チャットUI層

OpenCode は自然言語を **JSON-RPC コマンド** に変換する役割を持つ。

## 1.1 JSON-RPC 基本フォーマット

```json
{
  "jsonrpc": "2.0",
  "method": "<method_name>",
  "params": { ... },
  "id": <number>
}
```

---

## 1.2 サポートするメソッド一覧

### mixer.set  
ミキサーのパラメータ変更。

```json
{
  "jsonrpc": "2.0",
  "method": "mixer.set",
  "params": {
    "track": "Vocal",
    "param": "volume",
    "value": -6.0
  },
  "id": 1
}
```

### plugin.set_param  
プラグインのパラメータ変更。

```json
{
  "jsonrpc": "2.0",
  "method": "plugin.set_param",
  "params": {
    "track": "Vocal",
    "slot": 2,
    "plugin": "Compressor",
    "param": "Threshold",
    "value": -18.0
  },
  "id": 2
}
```

### command.exec  
Cubase のコマンド実行。

```json
{
  "jsonrpc": "2.0",
  "method": "command.exec",
  "params": {
    "id": "Transport_Play"
  },
  "id": 3
}
```

### session.status  
Cubase の状態取得。

```json
{
  "jsonrpc": "2.0",
  "method": "session.status",
  "params": {},
  "id": 4
}
```

---

## 1.3 OpenCode プロンプトテンプレート

OpenCode に設定するプロンプト：

```
あなたは Cubase 操作用アシスタントです。
ユーザーの自然言語を JSON-RPC 形式の操作コマンドに変換してください。

出力は必ず以下の形式に従ってください：

{
  "jsonrpc": "2.0",
  "method": "<method>",
  "params": { ... },
  "id": <number>
}

対応メソッド：
- mixer.set
- plugin.set_param
- command.exec
- session.status

例：
「ボーカルのコンプを強くして」
→ plugin.set_param / track=Vocal / plugin=Compressor / param=Threshold / value=-6

自然言語を必ず JSON-RPC に変換して返してください。
```

---

# 2. Rust API サーバ層

Rust は JSON-RPC を受け取り、SysEx に変換して Cubase に送信する。

## 2.1 Rust の役割

- JSON-RPC の受信  
- メソッドごとの処理分岐  
- JSON → SysEx 変換  
- Cubase Script へ送信  
- 応答を JSON-RPC 形式で返す  

---

## 2.2 API エンドポイント

### `POST /rpc`  
JSON-RPC リクエストを受け取る唯一のエンドポイント。

---

## 2.3 Rust 内部モジュール構成（推奨）

```
src/
 ├── main.rs
 ├── rpc/
 │    ├── handler.rs        // JSON-RPCメソッド分岐
 │    ├── types.rs          // mixer.set / plugin.set_param の型
 ├── midi/
 │    ├── sysex.rs          // JSON → SysEx 変換
 │    ├── port.rs           // 仮想MIDIポート管理
 ├── cubase/
 │    ├── dispatcher.rs     // SysEx送信・応答待ち
```

---

## 2.4 JSON-RPC → SysEx 変換フロー

1. JSON-RPC を受信  
2. `method` に応じて Rust の処理関数を呼ぶ  
3. JSON を SysEx にエンコード  
4. Cubase Script に送信  
5. 応答 SysEx を JSON に戻す  
6. JSON-RPC Response として返す  

---

# 3. Cubase MIDI Remote Script 層

Cubase 内部で動く JavaScript。  
Rust から送られた SysEx(JSON) を受け取り、Cubase API を実行する。

---

## 3.1 Script の役割

- SysEx を受信  
- JSON に復元  
- `method` に応じて Cubase API を呼ぶ  
- 結果を SysEx(JSON) で Rust に返す  

---

## 3.2 SysEx 受信ハンドラ例

```js
midiInput.setSysexCallback(function (data) {
    var json = decodeSysExToJson(data);
    handleRpc(json);
});
```

---

## 3.3 メソッド実装例

### mixer.set

```js
function mixerSet(params) {
    var ch = hostAccess.mMixConsole.getChannelByName(params.track);
    if (params.param === "volume") {
        ch.mVolume.setValue(params.value);
    }
}
```

### plugin.set_param

```js
function pluginSetParam(params) {
    var track = hostAccess.mTrackSelection.getTrackByName(params.track);
    var plugin = track.getPlugin(params.slot);
    var p = plugin.getParameterByName(params.param);
    p.setValue(params.value);
}
```

### command.exec

```js
function commandExec(params) {
    hostAccess.mApplication.performCommand(params.id);
}
```

---

# 4. 音声UIへの拡張（将来）

OpenCode の入力を「音声 → テキスト」に変えるだけで、  
バックエンドはそのまま流用できる。

```
音声入力
    ↓ Speech-to-Text
OpenCode Chat UI
    ↓ JSON-RPC
Rust API
    ↓ SysEx
Cubase
```

---

# 5. 開発ロードマップ

### Phase 1 — 基盤構築
- JSON-RPC DSL の確定  
- OpenCode プロンプト設定  
- Rust API `/rpc` 実装  
- 仮想MIDIポート作成  
- SysEx エンコード/デコード実装  

### Phase 2 — Cubase連携
- Cubase MIDI Remote Script の最小実装  
- transport.play の動作確認  
- mixer.set の動作確認  
- plugin.set_param の動作確認  

### Phase 3 — 実用化
- エラーハンドリング  
- 状態取得（session.status）  
- OpenCode チャットUIで自然言語操作  

### Phase 4 — 拡張
- 音声UI追加  
- 自動ミックス機能  
- プラグインプリセット適用  
- 外部ツール連携  

---

# 6. ハンドオフまとめ

このドキュメントを渡せば、  
OpenCode・Rust・Cubase のどの担当者でも開発を開始できます。

含まれる内容：

- JSON-RPC仕様  
- OpenCodeプロンプト  
- Rust API設計  
- Cubase Script設計  
- 全体アーキテクチャ  
- 開発ロードマップ  
