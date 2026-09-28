# Hihat Tick — starting points

Hihat definition preset. Goal: crisp tick with air, no low clutter, never harsh.
Order: EQ first (clean + air), tube second as saturator (no compression).
All values are 0..1 process values for `plugin.set_param`. Adjust by ear.

Preconditions: `Hihat` track selected in Cubase, inserts 0-1 loaded as below.
Send order: slot 0 -> 1. Check `session.status` first.

## Slot layout

| Slot | Plugin | Role |
|------|--------|------|
| 0 | Frequency | Cleaning (high low-cut + air) |
| 1 | TubeCompressor | Saturation only (drive color, ~zero reduction) |

## Slot 0 — cleaning EQ

- High low-cut (well into the midrange): removes snare bleed and clutter.
  This is the defining move — do not skip it.
- Air shelf boost on top (~0.6): the crispness. If harsh, pull this first.
- JSON carries the air shelf (one `Gain` + `Q`); low-cut manually
  by resolved title.

## Slot 1 — saturator (Tube, no compression)

- This slot compresses nothing: aim for ~zero gain reduction.
- Drive light (~0.3), character high (~0.8) so color sits up top.
- Input low (~0.4) to stay out of compression.
- Starting points: Input ~0.4, Drive ~0.3, Character ~0.8.

## Mixer

- `mixer.set / track=Hihat / param=pan / value=0` as neutral start.
  Kit-layout placement (slightly off-center) per arrangement.
- `mixer.set / track=Hihat / param=volume / value=-6.0` as start.

## Sends (`send.set`)

- Assumes send slot 0 -> short ambience FX (shared kit space, one-time setup).
- `send.set / track=Hihat / slot=0 / param=level / value=0.15`, on 1, post-fader.
- Hihat stays mostly dry; the send is only a trace of glue.

## Error recovery

- `Parameter not found` -> try shorter spelling
  (e.g. `Gain`, `Q`, `Input`, `Drive`, `Character`).
  Response `param` field shows the resolved Cubase title — use it next time.
- `Plugin mismatch` -> slot number or plugin name is wrong; check insert order.
- `Select track "Hihat" in Cubase first` -> select the track, then resend.
