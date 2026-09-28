# Vocal Back — starting points

Backing chorus pulled-back preset. Goal: sit one step behind the main vocal.
Same 5-slot skeleton as vocal-main, with three changes: deeper low-cut,
stronger second comp, shaping EQ in cut direction.
All values are 0..1 process values for `plugin.set_param`. Adjust by ear.

Preconditions: `Chorus` track selected in Cubase, inserts 0-4 loaded as below.
Send order: slot 0 -> 1 -> 2 -> 3 -> 4. Check `session.status` first.

## Slot layout

| Slot | Plugin | Role |
|------|--------|------|
| 0 | Frequency | Cleanup (deeper low-cut + harsh-mid notches) |
| 1 | TubeCompressor | Leveling (slow, gentle — same as main) |
| 2 | VintageCompressor | Firm control (high ratio, fast) |
| 3 | Frequency | Shaping (cut direction — steps back) |
| 4 | DeEsser | Sibilance taming (same as main) |

## Slot 0 — cleanup EQ

- Low-cut deeper than main (toward the 250-350 Hz region). This alone pushes it back.
- Same two narrow notches in the harsh mid band, cut direction.
- Starting points: low-cut enable 1, notch gains ~0.3, Q ~0.7.

## Slot 1 — leveling comp (Tube, LOW ratio)

- Same as main: slow attack (~0.5), long release (~0.7), low drive (~0.3).
- Light gain reduction. No change needed from vocal-main.
- Starting points: Input ~0.55, Attack ~0.5, Release ~0.7, Drive ~0.3.

## Slot 2 — firm comp (FET, high ratio)

- High ratio direction (~0.7 = around 8:1 feeling), faster attack (~0.3),
  fast release (~0.2). Deeper reduction than main.
- Stronger = lower threshold direction (~0.3). If it chokes, ease threshold up first.
- Starting points: Input ~0.6, Ratio ~0.7, Attack ~0.3, Release ~0.2.

## Slot 3 — shaping EQ (cut direction)

- Low shelf cut (~0.4), presence small boost (~0.6), high shelf cut (~0.4).
- The two cut shelves are what steps it back. Keep the presence band —
  without it the backing disappears entirely.
- If it masks the main, deepen the high shelf cut before touching level.

## Slot 4 — de-esser

- Same as main: Threshold ~0.4, reduction ~0.5.
- Backing sibilance stacks across voices; fix it here, not with level.

## Mixer

- `mixer.set / track=Chorus / param=pan / value=0` (center; spread only if arrangement allows).
- `mixer.set / track=Chorus / param=volume / value=-6.0` as start, then balance
  against the main by ear — typically a few dB under the main.
- Rule: if backing covers the main, pull backing EQ/level down first.

## Sends (`send.set`)

- Same hall FX as main (send slot 0); backing takes slightly more: level 0.3.
- `send.set / track=Chorus / slot=0 / param=level / value=0.3`, on 1, post-fader.
- Destination assignment is one-time manual setup in Cubase (no API).

## Error recovery

- `Parameter not found` -> try shorter spelling
  (e.g. `Gain`, `Threshold`, `Ratio`, `Attack`, `Release`, `Input`, `Drive`).
  Response `param` field shows the resolved Cubase title — use it next time.
- `Plugin mismatch` -> slot number or plugin name is wrong; check insert order.
- `Select track "Chorus" in Cubase first` -> select the track, then resend.
