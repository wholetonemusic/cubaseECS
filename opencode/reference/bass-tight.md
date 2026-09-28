# Bass Tight — starting points

Finger/pick bass tightening preset. Goal: low end sits with the kick
without mud, transients intact.
Order: EQ first (remove mud before comp), comp second (even out).
All values are 0..1 process values for `plugin.set_param`. Adjust by ear.

Preconditions: `Bass` track selected in Cubase, inserts 0-1 loaded as below.
Send order: slot 0 -> 1. Check `session.status` first.

## Slot layout

| Slot | Plugin | Role |
|------|--------|------|
| 0 | Frequency | Cleanup (sub cut + low weight + mud scoop + top cut) |
| 1 | Compressor | Evening out (mid ratio, transient-aware) |

## Slot 0 — cleanup EQ

- Sub low-cut at the bottom (~60 Hz region): removes rumble the speakers cannot reproduce.
- Small low boost just above the cut (~0.6): keeps weight after the cut.
- Wide scoop in the mud band (200-600 Hz region, ~0.35, wide Q ~0.5): this is the tightening move.
- High-cut toward the top (4-6 kHz region): removes string noise and harshness.
- JSON carries the mud scoop (one `Gain` + `Q`); set the low boost / cuts
  manually by resolved title (same-slot duplicate `Gain` would overwrite).

## Slot 1 — evening comp

- Mid ratio direction (~0.4 = around 3:1 feeling), soft knee if available.
- Medium attack (~0.45, keeps pick/finger transient), fast release (~0.3).
- Aim for moderate reduction. If it dulls the attack, lengthen attack first.
- Makeup only to match in/out loudness.
- Starting points: Threshold ~0.45, Ratio ~0.4, Attack ~0.45, Release ~0.3.

## Mixer

- `mixer.set / track=Bass / param=pan / value=0` (center — low end stays in the middle).
- `mixer.set / track=Bass / param=volume / value=-6.0` as start; balance against
  the kick by ear. If they clash, cut the bass EQ side (never just push level).
- Kick/bass masking is fixed with EQ + sidechain, not with faders.

## Sidechain (manual — no routing API yet)

- Ducking setup is manual: bass insert comp sidechain input fed from kick,
  enabled, threshold acting as on/off for the kick hits. Keep it subtle.

## Sends

- None by design: bass stays dry. No `send.set` calls in this preset.

## Error recovery

- `Parameter not found` -> try shorter spelling
  (e.g. `Threshold`, `Ratio`, `Attack`, `Release`, `Gain`, `Q`).
  Response `param` field shows the resolved Cubase title — use it next time.
- `Plugin mismatch` -> slot number or plugin name is wrong; check insert order.
- `Select track "Bass" in Cubase first` -> select the track, then resend.
