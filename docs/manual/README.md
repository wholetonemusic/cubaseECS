# Cubase Artist 15 マニュアル (ローカル参照用)

Steinberg Cubase Artist 15.0 Operation Manual (日本語版) および
Plug-in Reference (日本語版) のテキスト抽出版をローカル参照用に配置しています。

## 配置

| パス | 内容 |
| ---- | ---- |
| `docs/manual/cubase-artist15-manual/index.md` | 操作マニュアル: 全63章の章インデックス (ページ番号付き) |
| `docs/manual/cubase-artist15-manual/chXX-*.md` | 操作マニュアル: 章ごとの分割テキスト (63章分 + index = 64ファイル、約3.5MB) |
| `docs/manual/cubase-artist15-plugin-reference-ja.md` | プラグインリファレンス: 単一md (約432KB、202ページ、VSTエフェクト + MIDIエフェクト + 索引) |

## 原典 (PDF・単一md、リポジトリ外)

- PDF (約28MB、図・スクリーンショット確認用):
  `C:\Users\wholetone\Vocalo\docs\manual\Cubase_Artist_15_0_Operation_Manual_ja.pdf`
- 単一md (約3.5MB、分割版と同内容の結合版):
  `C:\Users\wholetone\Vocalo\docs\manual\cubase-artist15-operation-manual-ja.md`
- コピー元ディレクトリ:
  `C:\Users\wholetone\Vocalo\docs\manual\cubase-artist15-manual\`
- プラグインリファレンス PDF (約16MB、図・スクリーンショット確認用):
  `C:\Users\wholetone\Vocalo\docs\manual\Cubase_Artist_15_0_Plug-in_Reference_ja.pdf`
- プラグインリファレンス単一md (約432KB、本プロジェクトへのコピー元):
  `C:\Users\wholetone\Vocalo\docs\manual\cubase-artist15-plugin-reference-ja.md`

PDF と単一md は重複・大容量のためプロジェクトにはコピーしていません。
図表が必要な場合は上記パス (または元ディレクトリ) の PDF を直接参照してください。

## 参照方法

- 操作マニュアル: まず `cubase-artist15-manual/index.md` で章と PDF ページ番号を特定し、
  対応する `chXX-*.md` を読んでください。
- 例: MIDI Remote 関連は `ch41-0855.md` (PDF p.855-890) が起点です。
- プラグインリファレンス: `cubase-artist15-plugin-reference-ja.md` の冒頭の完全目次
  (178項目、PDFページ番号付き) で対象プラグインを特定し、`<!-- p.N -->` マーカーを
  目印に対応箇所を読んでください。例: Compressor 系は Dynamics (p.73〜)、
  空間系は Reverb (p.142〜)、MIDIエフェクトは (p.169〜)。
- `opencode/prompt.md` からもこのディレクトリを参照するよう案内しています。

## 注意

- テキスト抽出に由来する文字化けが一部にあります。正確な記述は PDF 原典で確認してください。
- マニュアル本文は Steinberg の著作物のため、**git 管理外 (local only)** としています。
  `.gitignore` で `docs/manual/cubase-artist15-manual/` と PDF/単一md
  (`cubase-artist15-operation-manual-ja.md`、
  `cubase-artist15-plugin-reference-ja.md`) を除外しています。
  この README のみコミット対象です。
  公開リポジトリに push してもマニュアル本文は含まれません。
