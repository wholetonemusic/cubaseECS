# Vocal Main — starting points

Main vocal front-preset. Goal: vocal sits in front without harshness.
All values below are 0..1 process values for `plugin.set_param`.
Adjust by ear; these are start points, not fixed answers.

Preconditions: `Vocal` track selected in Cubase, inserts 0-4 loaded as below.
Send order: slot 0 -> 1 -> 2 -> 3 -> 4. Check `session.status` first.

## Slot layout

| Slot | Plugin | Role |
|------|--------|------|
| 0 | Frequency | Cleanup (low-cut + harsh-mid notches) |
| 1 | TubeCompressor | Leveling (slow, gentle) |
| 2 | VintageCompressor | Peak control (medium) |
| 3 | Frequency | Shaping (low shelf + presence + air) |
| 4 | DeEsser | Sibilance taming |

## Slot 0 — cleanup EQ

- Low-cut on, around the 100 Hz region. Deepen toward 250 Hz for backing use.
- Two narrow notches (high Q) in the harsh mid band (2-4 kHz region), cut direction.
- Starting points: low-cut enable 1, notch gains ~0.3 (cut side), Q ~0.7 (narrow).

## Slot 1 — leveling comp (Tube, LOW ratio)

- Slow attack (~0.5), long release (~0.7), low drive (~0.3).
- Aim for light gain reduction (a few dB). If it pumps, lengthen release.
- Starting points: Input ~0.55, Attack ~0.5, Release ~0.7, Drive ~0.3.

## Slot 2 — peak comp (FET, mid ratio)

- Mid ratio (~0.5 = around 4:1 feeling), medium attack (~0.4), medium release (~0.4).
- Aim for moderate gain reduction. Stronger = lower threshold direction (~0.35).
- Backing use: higher ratio direction (~0.7), faster release (~0.2), deeper reduction.

## Slot 3 — shaping EQ

- Low shelf small boost (~0.6), presence band small boost (~0.6), high shelf small boost (~0.6).
- Backing use: low shelf cut direction (~0.4), high shelf cut direction (~0.4) to step back.
- If muddy, pull the low shelf first. If buried, push presence before adding level.

## Slot 4 — de-esser

- Threshold ~0.4 (lower = stronger), reduction amount ~0.5.
- If lispy, raise threshold (weaken). If harsh, lower threshold slightly.
- Too much reduction sounds unnatural; prefer more reduction amount over too-low threshold.

## Mixer

- `mixer.set / track=Vocal / param=pan / value=0` (center).
- `mixer.set / track=Vocal / param=volume / value=-6.0` as level start; keep master headroom.
- Fader rule: if vocal is quiet, pull others down first instead of pushing vocal up.

## Sends (`send.set`)

- Assumes send slot 0 -> hall reverb FX. One-time setup in Cubase: create the
  FX track, dial the hall (short pre-delay, ~0.5 s tail, wet-only mix), assign
  this send's destination to it. No API exists for creation/routing.
- `send.set / track=Vocal / slot=0 / param=level / value=0.2` (hidden),
  `param=on / value=1`, `param=prepost / value=0` (post-fader).
- Keep it hidden: if reverb is obvious, pull the send down, not the vocal up.

## Error recovery

- `Parameter not found` -> response shows nothing; try shorter param spelling
  (e.g. `Gain`, `Threshold`, `Ratio`, `Attack`, `Release`, `Input`, `Drive`).
  Response `param` field shows the resolved Cubase title — use it next time.
- `Plugin mismatch` -> slot number or plugin name is wrong; check insert order.
- `Select track "Vocal" in Cubase first` -> select the track, then resend.
