# Guitar Strum — starting points

Strummed acoustic guitar preset. Goal: bright strum character without
harsh overtones, sits beside bass and vocal instead of covering them.
Order: EQ first (tame harshness), gentle tube second (warmth + light glue).
All values are 0..1 process values for `plugin.set_param`. Adjust by ear.

Preconditions: `Guitar` track selected in Cubase, inserts 0-1 loaded as below.
Send order: slot 0 -> 1. Check `session.status` first.

## Slot layout

| Slot | Plugin | Role |
|------|--------|------|
| 0 | Frequency | Cleanup + character (sub cut + presence + harsh-taming) |
| 1 | TubeCompressor | Warmth + light glue (gentle, LOW ratio) |

## Slot 0 — cleanup/character EQ

- Sub low-cut at the bottom (~60 Hz region): leaves room for bass/kick.
- Presence boost in the upper-mid (~0.65, medium Q): this is the strum character.
- Harsh metallic overtones (5 kHz region) get narrow dynamic-style taming;
  statically, a narrow cut (~0.3, high Q ~0.8) at the ringing spot.
- High shelf small boost for air (~0.6).
- JSON carries presence boost (one `Gain` + `Q`); other bands manually
  by resolved title (same-slot duplicate `Gain` would overwrite).

## Slot 1 — warmth comp (Tube, LOW ratio)

- Low ratio, medium attack (~0.45), medium release (~0.5), light drive (~0.35).
- Aim for light reduction (a couple dB). Drive adds warmth; stop before
  distortion is audible.
- Starting points: Input ~0.55, Attack ~0.45, Release ~0.5, Drive ~0.35.

## Mixer

- `mixer.set / track=Guitar / param=pan / value=0` as neutral start.
  Harmony parts colliding with piano/keys at center should be spread L/R —
  decide per arrangement, then set pan accordingly.
- `mixer.set / track=Guitar / param=volume / value=-6.0` as start.
- If buried, push presence EQ before pushing level.

## Sends (`send.set`)

- Assumes send slot 0 -> small room FX (one-time setup in Cubase).
- `send.set / track=Guitar / slot=0 / param=level / value=0.25`, on 1, post-fader.
- Keeps strums from sounding dry without washing the rhythm.
  Slot 1 stays free for delay throws.

## Error recovery

- `Parameter not found` -> try shorter spelling
  (e.g. `Gain`, `Q`, `Threshold`, `Input`, `Attack`, `Release`, `Drive`).
  Response `param` field shows the resolved Cubase title — use it next time.
- `Plugin mismatch` -> slot number or plugin name is wrong; check insert order.
- `Select track "Guitar" in Cubase first` -> select the track, then resend.
