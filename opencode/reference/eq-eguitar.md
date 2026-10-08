# Electric-guitar EQ study -- starting points

Body, axis, and edge balanced per part. Clean, crunch, distortion, jazz, and metal directions. Goal: Guitar body, axis, and edge balanced per part.
Order: EQ first (shape before level). All values are 0..1 process values for `plugin.set_param`. Adjust by ear.

Preconditions: `Guitar` track selected in Cubase, insert 0 loaded as below.
Send order: slot 0 only. Check `session.status` first.

## Slot layout

| Slot | Plugin | Role |
|------|--------|------|
| 0 | Frequency | Shaping (study bands below) |

## Slot 0 -- shaping EQ

- Body in the low-mid (~0.55, medium Q): the wood. Axis in the mid (~0.55): the note center. Attack/edge in the upper-mid to presence (~0.6, medium Q): the pick. Top air small: the brightness; noisy amps need it eased. Low-cut at the bottom removes mud when stacking overdubs.
- JSON carries the defining band (one `Gain` + `Q`); other bands manually
  by resolved title (same-slot duplicate `Gain` would overwrite).

## Directions

- Clean direction: eased lows, favored axis plus presence. Crunch direction: favored low-mid plus presence harmonics. Distortion direction: eased low flab, favored distortion harmonics on top. Jazz direction: favored low warmth plus axis, top closed. Metal direction: deep pillar plus edge, mids left to bass.

## Mixer

- `mixer.set / track=Guitar / param=pan / value=0` as neutral start.
- `mixer.set / track=Guitar / param=volume / value=-6.0` as start; fix clashes
  with EQ cuts, not with faders.

## Sends

- None by design: study preset stays dry. No `send.set` calls.

## Error recovery

- `Parameter not found` -> try shorter spelling
  (e.g. `Gain`, `Q`).
  Response `param` field shows the resolved Cubase title -- use it next time.
- `Plugin mismatch` -> slot number or plugin name is wrong; check insert order.
- `Select track "Guitar" in Cubase first` -> select the track, then resend.
