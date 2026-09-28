# Snare Crack — starting points

Snare crack preset. Goal: body plus snap, no cardboard boxiness.
Order: EQ first (shape), FET second (contain hard).
All values are 0..1 process values for `plugin.set_param`. Adjust by ear.

Preconditions: `Snare` track selected in Cubase, inserts 0-1 loaded as below.
Send order: slot 0 -> 1. Check `session.status` first.

## Slot layout

| Slot | Plugin | Role |
|------|--------|------|
| 0 | Frequency | Shaping (sub cut + body + box cut + snap) |
| 1 | VintageCompressor | Contain hard (high ratio, fast) |

## Slot 0 — shaping EQ

- Low-cut toward the lower-mid (~140 Hz region): removes kick bleed and rumble.
- Body boost in the low fundamental region (~0.65): the weight.
- Boxiness cut in the mid (~0.35, medium Q): the cardboard remover.
- Snap boost/shelf in the upper-mid brilliance region (~0.65): the crack.
- JSON carries the box cut (one `Gain` + `Q`); other bands manually
  by resolved title (same-slot duplicate `Gain` would overwrite).

## Slot 1 — contain comp (FET)

- High ratio direction (~0.8 = around 20:1 feeling): snare takes limiting well.
- Fast attack (~0.25), medium release (~0.4, recovers per hit).
- Aim for firm reduction. If it gets thin, ease threshold up first.
- Punch switch on if available (`Punch` 1).
- Starting points: Input ~0.6, Ratio ~0.8, Attack ~0.25, Release ~0.4, Punch 1.

## Mixer

- `mixer.set / track=Snare / param=pan / value=0` (center — rhythm core).
- `mixer.set / track=Snare / param=volume / value=-6.0` as start.
- Short ambience send is the classic snare helper (slot 0, post-fader, low).
  If too wet, pull the send — not the snare.

## Sends (`send.set`)

- Assumes send slot 0 -> short ambience FX (one-time setup in Cubase).
- `send.set / track=Snare / slot=0 / param=level / value=0.4`, on 1, post-fader.

## Error recovery

- `Parameter not found` -> try shorter spelling
  (e.g. `Gain`, `Q`, `Input`, `Ratio`, `Attack`, `Release`, `Punch`).
  Response `param` field shows the resolved Cubase title — use it next time.
- `Plugin mismatch` -> slot number or plugin name is wrong; check insert order.
- `Select track "Snare" in Cubase first` -> select the track, then resend.
