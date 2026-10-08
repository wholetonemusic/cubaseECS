# Rhythm-machine EQ study -- starting points

Machine low weight kept tight, mud scooped, attack kept. Lofi warmth vs hifi extension contrast. Goal: Machine weight kept tight, attack kept, hats contrasted.
Order: EQ first (shape before level). All values are 0..1 process values for `plugin.set_param`. Adjust by ear.

Preconditions: `Rhythm` track selected in Cubase, insert 0 loaded as below.
Send order: slot 0 only. Check `session.status` first.

## Slot layout

| Slot | Plugin | Role |
|------|--------|------|
| 0 | Frequency | Shaping (study bands below) |

## Slot 0 -- shaping EQ

- Kick: deep low pillar (~0.6, medium Q) for weight, low-mid scoop (~0.4) for de-mud, small upper-mid lift (~0.55) for attack. Snare: low pillar for fat or low-mid scoop plus top lift for light/bright. Hat: low-mid scoop (~0.4) for de-clutter plus top lift (~0.6) for tight; low-mid plus mid lift for lofi warmth vs scooped low-mid plus extended top shelf for hifi stretch.
- JSON carries the defining band (one `Gain` + `Q`); other bands manually
  by resolved title (same-slot duplicate `Gain` would overwrite).

## Directions

- Fat/heavy direction: favor low pillars, ease mids/top. Light/bright direction: scoop low-mid, favor attack/top. Leave bass its own lane: scoop the machine where bass lives.

## Mixer

- `mixer.set / track=Rhythm / param=pan / value=0` as neutral start.
- `mixer.set / track=Rhythm / param=volume / value=-6.0` as start; fix clashes
  with EQ cuts, not with faders.

## Sends

- None by design: study preset stays dry. No `send.set` calls.

## Error recovery

- `Parameter not found` -> try shorter spelling
  (e.g. `Gain`, `Q`).
  Response `param` field shows the resolved Cubase title -- use it next time.
- `Plugin mismatch` -> slot number or plugin name is wrong; check insert order.
- `Select track "Rhythm" in Cubase first` -> select the track, then resend.
