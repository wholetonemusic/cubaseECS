# Analyzer Guide - AI Audio Analyzer readings

Read-only analysis feed from the `AI Audio Analyzer` VST3 insert.
Use it to pick a starting reference/preset, then drive Cubase with the
existing `mixer.set / plugin.set_param / send.set` methods.
Never invent new RPC methods for analyzer output.

## Acquisition order

1. `session.status` first (connection + `selectedTrack` check).
2. Select the target track in Cubase, with `AI Audio Analyzer` inserted.
   Only the selected track is analyzed (single-slot operation).
3. Start playback (levels/bands need running audio).
4. `analyzer.get_features` with `{}` params. Repeat reads at ~10 Hz max.

## Value semantics

- `rms / peak`: dBFS. Silence reads -120. `crest` = peak - rms (>= 0).
- `bands.{low,lowmid,mid,highmid,high}`: per-band RMS in dBFS.
  Edges: 20-120 / 120-500 / 500-2.5k / 2.5-6k / 6-20k Hz.
- `transient`: linear abs diff of short vs long RMS. Near 0 = steady,
  0.3+ = percussive attack present.
- `stereo_width`: L/R correlation. +1 = mono/centered, 0 = wide,
  -1 = out of phase. Mono tracks read 1.0.
- `stale`: true when the frame is older than 500 ms (stopped transport
  or bypassed insert). Re-read after starting playback.
- `mode`: `mock` (synthetic example values, no Cubase) or `host-midi`
  (live shared-memory values).

## Bands to reference map

| Reading | Likely move | Start from |
|---------|-------------|------------|
| `low` hot vs rest (+6 dB or more) | cleanup low end | `bass-tight.md` / vocal cleanup slot |
| `lowmid` buildup, muddy | narrow cuts 200-500 Hz | `drum-glue.md` / `vocal-main.md` slot 0 |
| `mid` harsh (2-4 kHz up) | notch harsh mids | `vocal-main.md` slot 0 / `guitar-strum.md` |
| `highmid` low, buried presence | presence lift ~5 kHz | `vocal-main.md` slot 3 |
| `high` dull | air lift | `overhead-shine.md` / `vocal-main.md` slot 3 |
| `transient` high + `crest` high | peak comp first | `drum-glue.md` / `kick-hit.md` |
| `transient` near 0, dense | leveling comp | `vocal-main.md` slot 1 |
| `stereo_width` near 0 on lead | narrow before FX | `adt-double.md` (width tricks) |

Values above are start points. All `plugin.set_param` values stay 0..1
process values; only `mixer.set volume` takes dB.

## Error recovery

- `{"result":{"ok":false,"error":"analyzer not running..."}}` ->
  insert `AI Audio Analyzer` on the selected track in Cubase first.
- `stale:true` -> start playback, wait ~1 s, re-read.
- `mode:"mock"` -> Docker/CI synthetic values. Start the host API
  (`MIDI_MODE=host`) with the VST3 inserted for live data.
- Readings jump between tracks -> one insert per project at a time;
  re-select the target track and re-read.
