# cubase-remote 配置・動作確認手順 (Phase 2, Cubase Artist 15)

`CubaseECS.js` は MIDI Remote API v1.x の実機ドライバ（ES5）。
Rust API との往復: `F0 7D <ASCII JSON> F7`（リクエストに `id` を含め、
応答 `{"jsonrpc":"2.0","id":N,"result":{...}}` を返す）。

## 1. loopMIDI 準備

1. loopMIDI を起動し、ポート名に `loopMIDI Port` を含むポートを用意する
   （例: `loopMIDI Port`、`loopMIDI Port 1` のいずれも検出可）。
2. ポート名を変えた場合は後述の `MIDI_PORT` に合わせる。

## 2. スクリプト配置

1. `CubaseECS.js` を Cubase の MIDI Remote scripts フォルダにコピーする。
2. Cubase 起動 → MIDI Remote Manager を開く。
3. デバイス `WholeTone / CubaseECS` が loopMIDI ポートで検出されることを確認し、
   有効化する（Input/Output ともに上記 loopMIDI ポート）。

## 3. Rust API をホスト実行で起動

Docker からはホスト MIDI に届かないため、Windows ホストの Rust で実行する:

```powershell
$env:MIDI_MODE = "host"
$env:MIDI_PORT = "loopMIDI Port"   # 部分一致。既定値なので省略可
cargo run --features host-midi
```

応答待ちタイムアウト（既定 2000ms）の変更:

```powershell
$env:MIDI_RESPONSE_TIMEOUT_MS = "5000"
```

## 4. 疎通確認（順番に実行）

```powershell
# session.status
Invoke-RestMethod -Method Post -Uri http://localhost:3001/rpc `
  -ContentType "application/json" `
  -Body '{"jsonrpc":"2.0","method":"session.status","params":{},"id":4}'

# transport 再生
Invoke-RestMethod -Method Post -Uri http://localhost:3001/rpc `
  -ContentType "application/json" `
  -Body '{"jsonrpc":"2.0","method":"command.exec","params":{"id":"Transport_Play"},"id":3}'

# mixer (volume は dB。0..1 直値は param "volume01")
Invoke-RestMethod -Method Post -Uri http://localhost:3001/rpc `
  -ContentType "application/json" `
  -Body '{"jsonrpc":"2.0","method":"mixer.set","params":{"track":"Vocal","param":"volume","value":-6.0},"id":1}'

# plugin (対象トラックを Cubase で選択状態にしてから実行すること)
Invoke-RestMethod -Method Post -Uri http://localhost:3001/rpc `
  -ContentType "application/json" `
  -Body '{"jsonrpc":"2.0","method":"plugin.set_param","params":{"track":"Vocal","slot":0,"plugin":"Compressor","param":"Threshold","value":0.5},"id":2}'
```

## 5. 値セマンティクスと制約（Phase 2）

- `mixer.set / volume`: dB 値を受け、実測テーパーで 0..1 に変換する
  （校正点: 1.00→+6.02dB、0.75→-0.81dB、0.50→-8.31dB、0.25119→-20.3dB。
  3次対数式 `dB=6.02+61.87x+64.92x²+58.22x³ (x=log10(p))` を二分法で逆変換。
  -6dB指定→-5.95dB表示を確認。`-60dB以下→0`、`+6dB以上→1`）。
  `volume01` で 0..1 直値、`mute`/`solo`（0/1）、`pan`（-1..1）も可。
  駆動は選択中トラック束縛の隠しフェーダ（DirectAccessの名前解決は使わない。
  列挙重複・表記ゆれ・clickVolume誤爆の教訓）。
- `mixer.set`・`plugin.set_param` とも対象トラックが Cubase で選択されていることが前提。
  未選択時は `Select track "X" in Cubase first` エラーが返る。
- `plugin.set_param / value`: 0..1 はプロセス値直指定。範囲外の数値は
  プレーーン値（dB 等）とみなし `convertParameterPlainToProcessValue`
  で変換を試みる（API 1.3 の Cubase 15 で利用可）。
- `slot` は 0 始まりのインサート番号。`plugin` 名は不一致時に照合エラーになる。

## 6. トラブルシューティング

- **デバイスが無効/未検出**: MIDI Remote Manager で `WholeTone / CubaseECS` を
  追加・有効化し、Input/Output とも同一 loopMIDI ポートを選ぶ。
  複数候補(`loopMIDI Port` / `loopMIDI Port 1`)がある場合は無印の
  `loopMIDI Port` を推奨(API既定 `MIDI_PORT` と一致)。
- **スクリプト更新後**: `CubaseECS.js` 配置後は Cubase を再起動する
  (起動時に読み込まれるため)。
- **timeout の切り分け**:
  1. `GET /healthz` の `midi_out_ports` / `midi_in_ports` でポート可視化。
  2. APIログの `connecting MIDI out port=... available=...` で選択ポート確認。
  3. 同一ポートの反響(self-echo)は Rust/JS 双方で無視する実装
     (応答に `result`/`error` がないもの・要求に `method` がないものは破棄)。
     それでも timeout する場合は Cubase 側未受信(デバイス無効・別ポート束縛)を疑う。
- **Hub状態(プロジェクト未オープン)**: `session.status` のみ可。
  mixer/plugin にはトラック入りのプロジェクトが必要。
- **plugin は事前に Cubase 上で対象トラックを選択** (未選択時は
  `Select track "X" in Cubase first` が返る)。
