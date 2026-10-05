# できないことリスト — Cubase自動操作 (cubaseECS)

最終更新: 2026-09-28。対象: `opencode/prompt.md` / `cubase-remote/CubaseECS.js` / `cubase-remote/README.md` の Phase 2〜3 実装時点。

`POST http://localhost:3001/rpc` 経由でできるのは6メソッドのみ:

- `mixer.set`: `volume / volume01 / mute / solo / pan`
- `plugin.set_param`
- `send.set`: `level / on / prepost`
- `command.exec`: `Transport_Play / Transport_Stop / Transport_Record`
- `session.status`, `analyzer.get_features` (読取専用)

以下はすべて**できないこと**として扱う。依頼が来たら断るか、手動操作を案内すること。

## 1. 現在の6メソッドの範囲外

### トラック構造・ミキサー詳細
- トラックの新規作成・削除・複製・改名・並替・フォルダ化・カラー変更
- 入出力ルーティング・入力ゲイン・Phase・モニター・Record Enable変更
- EQ / チャンネルストリップ / VCA / グループ / Cue操作
- オートメーションラインの書込・編集

### プラグイン管理
- 挿入・削除・移動・バイパス・プリセットロード・GUIオープン
- VSTiの音色選択・楽器パラメータの直接操作
- サイドチェイン配線・プラグイン一覧の取得

### センド・ルーティング
- `send.set` は8スロットの `level/on/prepost` のみ (`cubase-remote/CubaseECS.js:300-328`)
- センド宛先の割当・FX/Group/リターントラック作成・出力先変更は不可
  (`cubase-remote/README.md:76-83` の通り、MIDI Remote APIに該当機能なし。Cubase上で手動設定)

### トランスポート詳細・プロジェクト操作
- `command.exec` は Play/Stop/Record のみ (`cubase-remote/CubaseECS.js:276-298`)
- ループ・メトロノーム・テンポ/拍子変更・早送り/巻戻し・パンチ・マーカー/Cycle操作は不可
- 保存・別名保存・開く/閉じる・Undo/Redo・Mixdown/Exportは不可

### 編集操作
- オーディオのカット/コピー/コンピング・VariAudio・フェード/クロスフェード
- MIDIノート/ドラムエディタ・クオンタイズ・コードトラック・テンポトラック編集

### 状態取得の欠落
- `session.status` は `ok / connected / selectedTrack` のみ (`cubase-remote/CubaseECS.js:330-332`)
- トラック一覧・プラグイン一覧・再生位置・テンポ・拍子・プロジェクト名は取れない

## 2. MIDI Remote API v1.x + 選択トラック束縛の制約

- **未選択トラックへの直接操作不可**: `mixer / plugin / send` はすべて事前選択必須。未選択時は `Select track "X" in Cubase first` エラーで止まる (`cubase-remote/CubaseECS.js:194-328`)
- **パラメータ名の曖昧解決**: `findParamTag` は完全一致優先+部分一致fallback。`clickVolume`誤爆のような別物ヒットの可能性あり。パラメータ一覧の列挙APIなし
- **値セマンティクスが狭い**: `volume` はdB→テーパー近似、`plugin value` は0..1プロセス値。文字列・列挙・ファイルパス系パラメータは扱えない
- **ES5のみ**: `CubaseECS.js` は `midiremote_api_v1` の ES5サブセット。ファイルIO・任意のCubase内部コマンド実行は不可

## 3. 運用・環境の手動前提 (自動化できない準備)

- **Dockerからは実機に届かない**: `MIDI_MODE=host` + ホストネイティブ `cargo run --features host-midi` 必須 (`README.md:108-115`)
- **手動セットアップ必須**: loopMIDI作成、`CubaseECS.js` コピー、MIDI Remote Managerで有効化、更新後はCubase再起動 (`cubase-remote/README.md:1-35`)
- **Hub状態ではほぼ不可**: プロジェクト未オープン時は `session.status` のみ。トラック入りプロジェクト必須
- **日本語トラック名の直接不可**: SysExは `F0 7D <7bit-ASCII> F7` のみ。`ボーカル→Vocal` 等の正規化必須 (`opencode/prompt.md:63-64`)
- **Analyzerは読取専用・単一トラック**: 対象トラックに `AI Audio Analyzer.vst3` 挿入必須。10Hz・RMS/5バンド/トランジェント/相関のみ。複数トラック同時・書込は不可

## 4. AIアシスタントとしての固有の不可

- **画面・音が見えない/聞こえない**: Cubase GUIスクショ解析、耳でのミックス判断はできない。判断材料は `analyzer.get_features` の数値のみ
- **ブリッジなしでは触れない**: Rust API・loopMIDI・Cubaseのいずれかが落ちると何もできない。GUIクリックの代行はできない
- **タイミング保証なし**: 既定2000msタイムアウトの応答待ち。サンプル精度の同期演奏・リアルタイムフェーダーライドはできない
- **破壊的操作の自律実行なし**: 録音開始・大音量化などは確認なしにやらない。曖昧な指示 (`いい感じに`) はプリセットなしではJSON-RPCに確定できない

## 関連ファイル

- `opencode/prompt.md:13-45` — 対応メソッド定義
- `cubase-remote/CubaseECS.js:334-349` — RPC分岐 (上記6件のみ)
- `cubase-remote/README.md:60-83` — 値セマンティクスと制約
- `docs/reference-workflow.md:32-37` — What not to do (API追加なし方針)
