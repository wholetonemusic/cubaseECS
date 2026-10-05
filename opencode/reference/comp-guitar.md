# Comp Guitar — starting points

Guitar comp-only starter. Goal: stable picking with tone intact.
Electric parts take high ratios well; acoustic parts stay low.
Release is the phrase control (short = tight, long = flowing).
All values are 0..1 process values for `plugin.set_param`. Adjust by ear.

Preconditions: `Guitar` track selected in Cubase, stock Compressor
loaded in slot 0. Check `session.status` first.

## Slot layout

| Slot | Plugin | Role |
|------|--------|------|
| 0 | Compressor | Stability vs tone |

## Slot 0 — stability vs tone

- Clean-cutting base: Threshold ~0.5, Ratio ~0.65 (high),
  Attack ~0.25 (fast, evens picking), Release ~0.4 (medium).
  Start here for muted/cutting phrases.
- Crunch edge: Threshold ~0.45, Ratio ~0.55,
  Attack ~0.35, Release ~0.35. If it loses bite, lengthen
  attack slightly before lowering threshold.
- Lead sustain: Threshold ~0.5, Ratio ~0.7,
  Attack ~0.2 (fast, unifies particles), Release ~0.75 (long,
  lets notes sing). Loud; match loudness after engaging.
- Arpeggio flow: Attack ~0.5 (lets heads through),
  Release ~0.6 (long, connects notes). For shimmer use low
  ratio ~0.35; for evenness use high ratio ~0.6.
- Acoustic strum: Ratio ~0.3 (low), Threshold ~0.55,
  Attack ~0.3 (keeps woody knock), Release ~0.25 (short).
  If boomy, ease threshold up rather than shortening release.
- Starting points (electric): Threshold ~0.5, Ratio ~0.65,
  Attack ~0.25, Release ~0.4.

## Mixer

- `mixer.set / track=Guitar / param=pan / value=0` as start
  (spread L/R later only when colliding with keys at center).
- `mixer.set / track=Guitar / param=volume / value=-6.0` as start.
- Makeup is manual by resolved title: match bypassed loudness only.

## Sends (`send.set`)

- Assumes send slot 0 -> room FX (one-time setup in Cubase).
- `send.set / track=Guitar / slot=0 / param=level / value=0.2`, on 1, post-fader.
- If picking blurs, pull the send before touching the comp.

## Error recovery

- `Parameter not found` -> try shorter spelling
  (e.g. `Threshold`, `Ratio`, `Attack`, `Release`).
  Response `param` field shows the resolved Cubase title — use it next time.
- `Plugin mismatch` -> slot number or plugin name is wrong; check insert order.
- `Select track "Guitar" in Cubase first` -> select the track, then resend.
