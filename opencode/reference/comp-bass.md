# Comp Bass — starting points

Bass comp-only starter. Goal: even notes with attack intact.
Two-axis move: attack sets finger/pick definition, release sets
body length. Match release to song tempo (faster song = shorter).
All values are 0..1 process values for `plugin.set_param`. Adjust by ear.

Preconditions: `Bass` track selected in Cubase, stock Compressor
loaded in slot 0. Check `session.status` first.

## Slot layout

| Slot | Plugin | Role |
|------|--------|------|
| 0 | Compressor | Evening out (attack vs body) |

## Slot 0 — evening out

- Finger-style base: Threshold ~0.55 (a few dB reduction),
  Ratio ~0.4 (mid), Attack ~0.5 (keeps transient), Release ~0.35.
  Start here, then shorten release for fast phrases.
- Wild/rock direction: Threshold ~0.4 (deeper), Ratio ~0.4,
  Attack ~0.45, Release ~0.25. If it dulls the attack,
  lengthen attack before adding threshold depth.
- Slap/mute definition: Attack ~0.6 (slow, lets the slap through),
  Release ~0.3, Ratio ~0.35. Reduction shallow on purpose.
- Synth-bass color: Ratio ~0.65 (high), Attack ~0.25 (fast,
  brings harmonics forward), Release matched to note length
  (~0.3 short, ~0.6 for swell/uplift). Back off if it warbles.
- Upright feel: keep reduction shallowest (~0.65 threshold);
  preserve the acoustic bloom, even out only the loudest notes.
- Starting points: Threshold ~0.55, Ratio ~0.4,
  Attack ~0.5, Release ~0.35.

## Mixer

- `mixer.set / track=Bass / param=pan / value=0` (low end stays center).
- `mixer.set / track=Bass / param=volume / value=-6.0` as start.
- Makeup is manual by resolved title: match bypassed loudness only.

## Sidechain (manual — no routing API yet)

- Kick-ducking setup is manual: bass insert comp sidechain input
  fed from kick, kept subtle. Fix masking with release/threshold
  first, ducking second.

## Sends

- None by design: bass stays dry. No `send.set` calls in this preset.

## Error recovery

- `Parameter not found` -> try shorter spelling
  (e.g. `Threshold`, `Ratio`, `Attack`, `Release`).
  Response `param` field shows the resolved Cubase title — use it next time.
- `Plugin mismatch` -> slot number or plugin name is wrong; check insert order.
- `Select track "Bass" in Cubase first` -> select the track, then resend.
