# Tom Fill — starting points

Tom punch preset. Goal: round body with clear attack for fills, no boom
ringing into the next hit.
Order: EQ first (shape), FET second (contain).
All values are 0..1 process values for `plugin.set_param`. Adjust by ear.

Preconditions: `Tom` track selected in Cubase, inserts 0-1 loaded as below.
Send order: slot 0 -> 1. Check `session.status` first.

## Slot layout

| Slot | Plugin | Role |
|------|--------|------|
| 0 | Frequency | Shaping (sub cut + body + attack) |
| 1 | VintageCompressor | Contain (mid ratio, fast) |

## Slot 0 — shaping EQ

- Sub low-cut at the very bottom: removes rumble below the shell tone.
- Body boost in the low fundamental region (~0.6, medium Q): the roundness.
- Attack boost in the upper-mid (~0.6): the stick definition.
- JSON carries body boost (one `Gain` + `Q`); other bands manually
  by resolved title (same-slot duplicate `Gain` would overwrite).

## Slot 1 — contain comp (FET)

- Mid ratio direction (~0.4 = around 4:1 feeling).
- Medium-fast attack (~0.3), fast release (~0.25, recovers before next hit).
- Aim for firm reduction. If fills blur together, shorten release first.
- Starting points: Input ~0.55, Ratio ~0.4, Attack ~0.3, Release ~0.25.

## Mixer

- `mixer.set / track=Tom / param=pan / value=0` as neutral start.
  Multiple toms spread L/R per kit layout.
- `mixer.set / track=Tom / param=volume / value=-6.0` as start.

## Sends (`send.set`)

- Assumes send slot 0 -> short ambience FX (shared kit space, one-time setup).
- `send.set / track=Tom / slot=0 / param=level / value=0.3`, on 1, post-fader.

## Error recovery

- `Parameter not found` -> try shorter spelling
  (e.g. `Gain`, `Q`, `Input`, `Ratio`, `Attack`, `Release`).
  Response `param` field shows the resolved Cubase title — use it next time.
- `Plugin mismatch` -> slot number or plugin name is wrong; check insert order.
- `Select track "Tom" in Cubase first` -> select the track, then resend.
