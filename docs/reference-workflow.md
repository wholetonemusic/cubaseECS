# Reference workflow

How to grow `opencode/reference/` + `opencode/presets/` as new material arrives.

## Rules

- Paraphrase only. Never paste source text verbatim.
- No source notation in reference/preset files.
- Keep each reference file 2-5 KB. Split topics instead of growing one file.
- ASCII param/track names only (SysEx 7bit-ASCII limit, SysEx path carries no book content anyway).
- `plugin.set_param` values are 0..1 process values (`mixer.set volume` in dB excepted).
- Send convention: slot 0 = primary space FX, slot 1 = secondary space FX.
  `send.set` values: level 0..1, on 1, prepost 0 (post-fader;
  prepost 1 = pre-fader, for parallel/FX-only tricks like parallel-ny).
  FX creation + destination assignment is one-time manual setup (no API).
- Preset JSON carries `{method, params}` only; `id` is assigned at send time.

## Steps per new topic

1. Distill: chain order, per-slot role, 0..1 start points, direction semantics
   (which way is stronger/brighter/deeper), 2-3 adjust-by-ear hints.
2. Write `opencode/reference/<name>.md` following `vocal-main.md` headings
   (Slot layout / per-slot / Mixer / Error recovery).
3. Write `opencode/presets/<name>.json` following `vocal-main.json` shape,
   `session.status` first, then mixer, then slots in order, then `send.set` calls.
   Multi-track presets (e.g. FX programming): group calls per track; the user
   selects each track before its group (`session.status` confirms).
4. Register both in `opencode/reference/index.md`.
5. Validate: `python -c json.load`, `cargo test`, `node --check cubase-remote/CubaseECS.js`.
   Real-Cubase spot check when available (response `param` field confirms resolution).

## What not to do

- No new Rust/JS API for presets (current methods suffice; toolchain pin sensitive).
- No FX creation / destination assignment / track creation in presets
  (no API — one-time manual setup in Cubase).
- No prompt.md growth: pointer lines only, details stay in reference files.
