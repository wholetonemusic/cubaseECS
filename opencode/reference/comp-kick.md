# Comp Kick — starting points

Kick comp-only starter. Goal: choose one of four finishes
(natural / fat / tight / drive) with the same single Compressor slot.
Order: comp only. EQ and level stay where they are; change finish
by moving threshold / ratio / attack / release, not the fader.
All values are 0..1 process values for `plugin.set_param`. Adjust by ear.

Preconditions: `Kick` track selected in Cubase, stock Compressor
loaded in slot 0. Check `session.status` first.

## Slot layout

| Slot | Plugin | Role |
|------|--------|------|
| 0 | Compressor | Finish selector (attack vs tail balance) |

## Slot 0 — finish selector

- Natural direction: Threshold ~0.55 (shallow, a couple dB reduction),
  Ratio ~0.4 (mid), Attack ~0.5 (lets the beater through),
  Release ~0.3 (recovers before the next hit). Start here for live kicks.
- Fat direction: Threshold ~0.4 (deeper), Ratio ~0.4,
  Attack ~0.5, Release ~0.7 (long, lifts the tail on purpose).
  If it pumps too much, ease threshold up first.
- Tight direction: Threshold ~0.5, Ratio ~0.6 (high),
  Attack ~0.4, Release ~0.25 (short, glues attack to tail).
  If it gets thin, ease ratio down before touching attack.
- Drive direction: Threshold ~0.25 (deepest), Ratio ~0.35 (low),
  Attack ~0.1 (fastest), Release ~0.2 (short). Loud; pull the
  channel fader down before engaging, match in/out loudness after.
- Long-tail electronic kicks: set release so recovery ends just
  before the tail ends. Too short chops the tail, too long dulls
  the next hit. Move release in small steps.
- Starting points (natural): Threshold ~0.55, Ratio ~0.4,
  Attack ~0.5, Release ~0.3.

## Mixer

- `mixer.set / track=Kick / param=pan / value=0` (center — always).
- `mixer.set / track=Kick / param=volume / value=-6.0` as start.
  Clashes with bass are fixed on the bass side, not by pushing kick.
- Makeup is manual by resolved title: match bypassed loudness only.

## Sends

- None by design: kick stays dry. No `send.set` calls in this preset.

## Error recovery

- `Parameter not found` -> try shorter spelling
  (e.g. `Threshold`, `Ratio`, `Attack`, `Release`).
  Response `param` field shows the resolved Cubase title — use it next time.
- `Plugin mismatch` -> slot number or plugin name is wrong; check insert order.
- `Select track "Kick" in Cubase first` -> select the track, then resend.
