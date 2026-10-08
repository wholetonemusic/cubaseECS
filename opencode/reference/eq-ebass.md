# Electric-bass EQ study -- starting points

Low pillar sits with kick, mud scooped, attack/air kept. Finger, pick, slap, and genre directions. Goal: Low end sits with the kick, attack intact, no mud.
Order: EQ first (shape before level). All values are 0..1 process values for `plugin.set_param`. Adjust by ear.

Preconditions: `Bass` track selected in Cubase, insert 0 loaded as below.
Send order: slot 0 only. Check `session.status` first.

## Slot layout

| Slot | Plugin | Role |
|------|--------|------|
| 0 | Frequency | Shaping (study bands below) |

## Slot 0 -- shaping EQ

- Deep low pillar (~0.6, medium Q): the foundation; boost the lane below the axis, not the axis itself, to stay forward. Mud scoop in the low-mid (~0.35, medium-narrow Q): the de-mud move. Axis in the mid (~0.55): the outline. Attack in the upper-mid (~0.6, narrow Q) plus small top air: the finger/pick definition.
- JSON carries the defining band (one `Gain` + `Q`); other bands manually
  by resolved title (same-slot duplicate `Gain` would overwrite).

## Directions

- Tidy direction: scoop low-mid, add small attack/air. Reggae direction: deep pillar plus low-mid spine, eased mids, top closed. Jazz direction: tamed lows, favored axis, softened top. Metal direction: scooped low-mid, favored spine/attack/edge. Vintage direction: favored mids, eased top.

## Mixer

- `mixer.set / track=Bass / param=pan / value=0` as neutral start.
- `mixer.set / track=Bass / param=volume / value=-6.0` as start; fix clashes
  with EQ cuts, not with faders.

## Sends

- None by design: study preset stays dry. No `send.set` calls.

## Error recovery

- `Parameter not found` -> try shorter spelling
  (e.g. `Gain`, `Q`).
  Response `param` field shows the resolved Cubase title -- use it next time.
- `Plugin mismatch` -> slot number or plugin name is wrong; check insert order.
- `Select track "Bass" in Cubase first` -> select the track, then resend.
