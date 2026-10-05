# Comp Vocal — starting points

Vocal comp-only starter. Goal: steady words with breath intact.
Main stays light, doubles sit deeper behind it, chorus deepest
and most stable. Sibilance is a separate de-esser step, not comp.
All values are 0..1 process values for `plugin.set_param`. Adjust by ear.

Preconditions: `Vocal` track selected in Cubase, stock Compressor
loaded in slot 0 (de-esser, if any, lives in a later slot).
Check `session.status` first.

## Slot layout

| Slot | Plugin | Role |
|------|--------|------|
| 0 | Compressor | Steadiness (genre sets attack/release) |

## Slot 0 — steadiness

- Ballad main: Threshold ~0.55 (2-3 dB on peaks), Ratio ~0.55,
  Attack ~0.4 (natural heads), Release ~0.6 (long, steadies).
  If breath pumps, lengthen release slightly.
- Pop main: Threshold ~0.5 (a few dB), Ratio ~0.45,
  Attack ~0.55 (lets heads jump), Release ~0.45.
  Harmonics should read clearly, not loudly.
- Rock main: Threshold ~0.45 (deeper, up to ~6 dB on peaks),
  Ratio ~0.5, Attack ~0.55, Release ~0.35 (fast, in-your-face).
- Double behind main: one step deeper threshold than its main
  (~0.45), Ratio ~0.45, Release matched to main (ballad long,
  pop/rock short). Goal: support, never cover.
- Chorus bed: Threshold ~0.35 (deep), Ratio ~0.6 (high),
  Attack ~0.4, Release ~0.3. Split high/low parts by ear so
  stacked takes blend instead of fighting.
- Order note: pitch clean first, then de-esser, then this comp.
  If sibilance jumps after comp, fix the de-esser, not this slot.
- Starting points (pop main): Threshold ~0.5, Ratio ~0.45,
  Attack ~0.55, Release ~0.45.

## Mixer

- `mixer.set / track=Vocal / param=pan / value=0` (center).
- `mixer.set / track=Vocal / param=volume / value=-6.0` as start.
- Makeup is manual by resolved title: match bypassed loudness only.

## Sends (`send.set`)

- Assumes send slot 0 -> hall FX (one-time setup in Cubase).
- `send.set / track=Vocal / slot=0 / param=level / value=0.2`, on 1, post-fader.
- Keep it hidden: if wash is obvious, pull the send first.

## Error recovery

- `Parameter not found` -> try shorter spelling
  (e.g. `Threshold`, `Ratio`, `Attack`, `Release`).
  Response `param` field shows the resolved Cubase title — use it next time.
- `Plugin mismatch` -> slot number or plugin name is wrong; check insert order.
- `Select track "Vocal" in Cubase first` -> select the track, then resend.
