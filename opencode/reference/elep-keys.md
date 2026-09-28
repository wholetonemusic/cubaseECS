# Elep Keys — starting points

Electric piano preset. Goal: bell-like tine with warmth, sits with guitar
instead of fighting it.
Order: EQ first (scoop + tine), tube second (warmth + light glue).
All values are 0..1 process values for `plugin.set_param`. Adjust by ear.

Preconditions: `Keys` track selected in Cubase, inserts 0-1 loaded as below.
Send order: slot 0 -> 1. Check `session.status` first.

## Slot layout

| Slot | Plugin | Role |
|------|--------|------|
| 0 | Frequency | Shaping (low-cut + scoop + tine) |
| 1 | TubeCompressor | Warmth (gentle, LOW ratio) |

## Slot 0 — shaping EQ

- Low-cut at the bottom: leaves room for bass.
- Scoop in the boxy low-mid (~0.35, medium Q): the clearing move.
- Tine presence boost in the upper-mid (~0.65): the bell character.
- JSON carries the scoop (one `Gain` + `Q`); other bands manually
  by resolved title (same-slot duplicate `Gain` would overwrite).

## Slot 1 — warmth comp (Tube, LOW ratio)

- Low ratio, medium attack (~0.45), medium release (~0.4), light drive (~0.35).
- Aim for light reduction. If barky, ease input down first.
- Starting points: Input ~0.55, Attack ~0.45, Release ~0.4, Drive ~0.35.

## Mixer

- `mixer.set / track=Keys / param=pan / value=0` as neutral start.
  Clashing with guitar at center? Spread them L/R per arrangement.
- `mixer.set / track=Keys / param=volume / value=-6.0` as start.

## Sends (`send.set`)

- Assumes send slot 0 -> small room FX (one-time setup in Cubase).
- `send.set / track=Keys / slot=0 / param=level / value=0.3`, on 1, post-fader.

## Error recovery

- `Parameter not found` -> try shorter spelling
  (e.g. `Gain`, `Q`, `Input`, `Attack`, `Release`, `Drive`).
  Response `param` field shows the resolved Cubase title — use it next time.
- `Plugin mismatch` -> slot number or plugin name is wrong; check insert order.
- `Select track "Keys" in Cubase first` -> select the track, then resend.
