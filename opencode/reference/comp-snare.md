# Comp Snare — starting points

Snare comp-only starter. Goal: pick body-plus-snap balance
(natural / fat / tight / drive) with one Compressor slot.
Snare takes higher ratios well; move ratio first, attack second.
All values are 0..1 process values for `plugin.set_param`. Adjust by ear.

Preconditions: `Snare` track selected in Cubase, stock Compressor
loaded in slot 0. Check `session.status` first.

## Slot layout

| Slot | Plugin | Role |
|------|--------|------|
| 0 | Compressor | Snap vs body balance |

## Slot 0 — snap vs body

- Natural direction: Threshold ~0.6 (shallow, 1-2 dB reduction),
  Ratio ~0.45 (mid), Attack ~0.55 (slightly slow, keeps punch),
  Release ~0.35 (medium). Start here for live snares.
- Fat direction: Threshold ~0.45 (deeper), Ratio ~0.55,
  Attack ~0.65 (slow, lets attack through), Release ~0.7 (long,
  lifts the tail). For a laid-back feel lengthen release, not attack.
- Tight direction: Threshold ~0.55, Ratio ~0.7 (high),
  Attack ~0.3, Release ~0.2 (short, crisp). If it gets harsh,
  ease threshold up before shortening release further.
- Drive direction: Threshold ~0.35, Ratio ~0.55,
  Attack ~0.1 (fastest), Release ~0.2 (short, stretches the tail
  into grit). Loud; pull the fader down first, match loudness after.
- Rimshot-led playing: favor the natural direction with a touch
  slower attack so the stick crack survives the first dB.
- Starting points (natural): Threshold ~0.6, Ratio ~0.45,
  Attack ~0.55, Release ~0.35.

## Mixer

- `mixer.set / track=Snare / param=pan / value=0` (center — rhythm core).
- `mixer.set / track=Snare / param=volume / value=-6.0` as start.
- Makeup is manual by resolved title: match bypassed loudness only.

## Sends (`send.set`)

- Assumes send slot 0 -> short ambience FX (one-time setup in Cubase).
- `send.set / track=Snare / slot=0 / param=level / value=0.3`, on 1, post-fader.
- If washes out, pull the send before touching the comp.

## Error recovery

- `Parameter not found` -> try shorter spelling
  (e.g. `Threshold`, `Ratio`, `Attack`, `Release`).
  Response `param` field shows the resolved Cubase title — use it next time.
- `Plugin mismatch` -> slot number or plugin name is wrong; check insert order.
- `Select track "Snare" in Cubase first` -> select the track, then resend.
