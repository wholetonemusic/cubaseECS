# Piano Sing — starting points

Lead piano preset. Goal: clear singing tone that carries a ballad without
boomy resonance or harsh ringing.
Order: EQ first (remove resonance), tube second (gentle warmth).
All values are 0..1 process values for `plugin.set_param`. Adjust by ear.

Preconditions: `Piano` track selected in Cubase, inserts 0-1 loaded as below.
Send order: slot 0 -> 1. Check `session.status` first.

## Slot layout

| Slot | Plugin | Role |
|------|--------|------|
| 0 | Frequency | Cleanup (low-cut + mud scoop + resonance notch + presence) |
| 1 | TubeCompressor | Warmth (gentle, LOW ratio) |

## Slot 0 — cleanup EQ

- Low-cut toward the low-mid (~80 Hz region): removes boom, leaves room for bass.
- Wide scoop in the muddy low-mid (~0.35, wide Q ~0.4): the clearing move.
- Narrow notch at the ringing resonance (upper-mid, high Q ~0.75, cut ~0.3):
  find it by sweeping a narrow boost, then cut.
- Presence shelf small boost (~0.6): the singing edge.
- JSON carries the mud scoop (one `Gain` + `Q`); other bands manually
  by resolved title (same-slot duplicate `Gain` would overwrite).

## Slot 1 — warmth comp (Tube, LOW ratio)

- Low ratio, medium attack (~0.5), medium-long release (~0.6), light drive (~0.3).
- Aim for light reduction. Drive adds warmth; stop before it hardens.
- Starting points: Input ~0.55, Attack ~0.5, Release ~0.6, Drive ~0.3.

## Mixer

- `mixer.set / track=Piano / param=pan / value=0` as neutral start.
  Solo/lead piano stays center; ensemble backing spreads per arrangement.
- `mixer.set / track=Piano / param=volume / value=-6.0` as start.
- If boomy against bass, deepen the low-cut before touching level.

## Sends (`send.set`)

- Assumes send slot 0 -> medium hall FX (one-time setup in Cubase).
- `send.set / track=Piano / slot=0 / param=level / value=0.45`, on 1, post-fader.
- Piano takes more reverb than most — if it washes, shorten the FX tail
  before pulling the send.

## Error recovery

- `Parameter not found` -> try shorter spelling
  (e.g. `Gain`, `Q`, `Input`, `Attack`, `Release`, `Drive`).
  Response `param` field shows the resolved Cubase title — use it next time.
- `Plugin mismatch` -> slot number or plugin name is wrong; check insert order.
- `Select track "Piano" in Cubase first` -> select the track, then resend.
