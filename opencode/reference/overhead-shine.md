# Overhead Shine — starting points

Overhead cymbal preset. Goal: airy cymbals with presence, no harsh wash,
kick bleed kept out of the way.
Order: EQ first (shape), tube second (air + light glue).
All values are 0..1 process values for `plugin.set_param`. Adjust by ear.

Preconditions: `Overhead` track selected in Cubase, inserts 0-1 loaded as below.
Send order: slot 0 -> 1. Check `session.status` first.

## Slot layout

| Slot | Plugin | Role |
|------|--------|------|
| 0 | Frequency | Shaping (low-cut + presence + air) |
| 1 | TubeCompressor | Air + light glue (gentle drive) |

## Slot 0 — shaping EQ

- Low-cut toward the low-mid: removes kick bleed and room rumble.
- Presence boost in the upper-mid (~0.6): the stick and bell definition.
- Air shelf boost on top (~0.65): the shimmer. If harsh, pull this first.
- JSON carries presence (one `Gain` + `Q`); other bands manually
  by resolved title (same-slot duplicate `Gain` would overwrite).

## Slot 1 — air comp (Tube, LOW ratio)

- Low ratio, medium attack (~0.4), medium release (~0.4).
- Drive moderate (~0.45) for air and shimmer; stop before harshness.
- Aim for light reduction only.
- Starting points: Input ~0.55, Attack ~0.4, Release ~0.4, Drive ~0.45.

## Mixer

- `mixer.set / track=Overhead / param=pan / value=0` as neutral start
  (overheads are usually stereo; keep the pair's own image).
- `mixer.set / track=Overhead / param=volume / value=-6.0` as start.

## Sends (`send.set`)

- Assumes send slot 0 -> short ambience FX (shared kit space, one-time setup).
- `send.set / track=Overhead / slot=0 / param=level / value=0.25`, on 1, post-fader.
- Overheads already carry room; keep the send low.

## Error recovery

- `Parameter not found` -> try shorter spelling
  (e.g. `Gain`, `Q`, `Input`, `Attack`, `Release`, `Drive`).
  Response `param` field shows the resolved Cubase title — use it next time.
- `Plugin mismatch` -> slot number or plugin name is wrong; check insert order.
- `Select track "Overhead" in Cubase first` -> select the track, then resend.
