# Acoustic-guitar EQ study -- starting points

Body, axis, attack, and shimmer balanced per technique. Strum vs arpeggio, solo vs vocal blend. Goal: Natural acoustic body with clear attack and shimmer.
Order: EQ first (shape before level). All values are 0..1 process values for `plugin.set_param`. Adjust by ear.

Preconditions: `AcousticGuitar` track selected in Cubase, insert 0 loaded as below.
Send order: slot 0 only. Check `session.status` first.

## Slot layout

| Slot | Plugin | Role |
|------|--------|------|
| 0 | Frequency | Shaping (study bands below) |

## Slot 0 -- shaping EQ

- Body in the low-mid (~0.55, medium Q): the wood; move to taste per instrument. Axis in the mid (~0.5): the note center; too much loses transparency. Attack in the upper-mid (~0.55): the pick/string edge. Shimmer shelf on top (~0.55): the steel glow; also where noise lives. Low-cut at the bottom removes room boom and bleed.
- JSON carries the defining band (one `Gain` + `Q`); other bands manually
  by resolved title (same-slot duplicate `Gain` would overwrite).

## Directions

- Arpeggio direction: eased low-mid, favored attack plus shimmer. Strum direction: eased low-mid cloud, favored attack. Singer-strummer direction: eased vocal-range mids to blend. Warm direction: favored low-mid axis, eased top. Line-pickup direction: cut low resonances first, then add attack/shimmer.

## Mixer

- `mixer.set / track=AcousticGuitar / param=pan / value=0` as neutral start.
- `mixer.set / track=AcousticGuitar / param=volume / value=-6.0` as start; fix clashes
  with EQ cuts, not with faders.

## Sends

- None by design: study preset stays dry. No `send.set` calls.

## Error recovery

- `Parameter not found` -> try shorter spelling
  (e.g. `Gain`, `Q`).
  Response `param` field shows the resolved Cubase title -- use it next time.
- `Plugin mismatch` -> slot number or plugin name is wrong; check insert order.
- `Select track "AcousticGuitar" in Cubase first` -> select the track, then resend.
