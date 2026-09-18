# cubase-remote 配置メモ

1. `CubaseECS.js` を Cubase の MIDI Remote Script フォルダにコピーする。
2. Cubase 起動 → MIDI Remote Manager でスクリプトを有効化する。
3. Rust API 側をホスト実行で立ち上げる:
   `cargo run --features host-midi` (`MIDI_MODE=host`)
4. `POST http://localhost:3001/rpc` で `command.exec / Transport_Play` を送り動作確認する。

詳細手順は Phase 2 で追記する。
