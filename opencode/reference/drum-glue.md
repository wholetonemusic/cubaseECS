# Drum Glue — starting points

Drum bus glue preset. Goal: kit gels as one unit before any band shaping.
Order matters: comp first (glue), EQ second (shape). Light touch throughout —
a couple dB of change already moves the impression a lot.
All values are 0..1 process values for `plugin.set_param`. Adjust by ear.

Preconditions: `Drums` group track selected in Cubase, individual kit tracks
routed to it, inserts 0-1 loaded as below.
Send order: slot 0 -> 1. Check `session.status` first.

## Slot layout

| Slot | Plugin | Role |
|------|--------|------|
| 0 | Compressor | Bus glue (low ratio, slow attack, fast release) |
| 1 | Frequency | Shaping (sub cut + low-mid scoop + air) |

## Slot 0 — bus comp

- Low ratio direction (~0.3 = around 2:1 feeling). Never deep on a bus.
- Slow attack (~0.7, lets transients through), fast release (~0.3, auto if available).
- Aim for small gain reduction only (a couple dB). If it pumps, ease threshold up.
- Makeup only to match in/out loudness, not to get louder.
- Starting points: Threshold ~0.6, Ratio ~0.3, Attack ~0.7, Release ~0.3.

## Slot 1 — shaping EQ

- Sub low-cut at the bottom of the audible range (removes rumble, tightens low end).
- Wide shallow cut in the low-mid mud region (~0.4, wide Q ~0.3).
- High shelf small boost for air (~0.6). If cymbals get harsh, pull this first.
  Band-specific: send one `Gain` call, read the resolved title in the response,
  then address the other band by its resolved title (avoids overwriting).
- Starting points: low-cut enable 1, mud-band gain ~0.4 with Q ~0.3, air gain ~0.6.

## Mixer

- `mixer.set / track=Drums / param=pan / value=0` (group pan stays center;
  individual kit panning is preserved through it).
- `mixer.set / track=Drums / param=volume / value=-6.0` as start; group fader
  is the overall kit level. Keep master headroom for later stages.
- If kick triggers the bus comp too much, fix with the comp threshold —
  not by pulling the kick fader.

## Sends (`send.set`)

- Assumes slot 0 -> short ambience FX, slot 1 -> long hall FX.
  FX creation + destination assignment is one-time manual setup (no API).
- Group-level start: slot 0 level 0.3, slot 1 level 0.2, both on 1, post-fader.
- Refine per piece afterwards: kick minimal/none, snare/toms moderate
  (kick-hit / snare-crack carry their own slot-0 sends).
- If the kit washes out, pull sends before touching the bus.

## Error recovery

- `Parameter not found` -> try shorter spelling
  (e.g. `Threshold`, `Ratio`, `Attack`, `Release`, `Gain`, `Q`).
  Response `param` field shows the resolved Cubase title — use it next time.
- `Plugin mismatch` -> slot number or plugin name is wrong; check insert order.
  A dedicated bus comp plugin may be used in slot 0 instead of stock Compressor.
- `Select track "Drums" in Cubase first` -> select the group track, then resend.
