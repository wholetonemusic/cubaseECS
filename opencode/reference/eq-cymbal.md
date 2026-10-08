# Cymbal EQ study -- starting points

Shimmer that sits in the mix, low weight tamed, attack kept. Cut-to-lift technique included. Goal: Shimmer that sits in the mix, low weight tamed, attack kept.
Order: EQ first (shape before level). All values are 0..1 process values for `plugin.set_param`. Adjust by ear.

Preconditions: `Cymbal` track selected in Cubase, insert 0 loaded as below.
Send order: slot 0 only. Check `session.status` first.

## Slot layout

| Slot | Plugin | Role |
|------|--------|------|
| 0 | Frequency | Shaping (study bands below) |

## Slot 0 -- shaping EQ

- Low weight ease in the deep low region: removes gong-like heaviness. Attack lift in the upper-mid (~0.55, medium Q): the stick definition. Edge lift in the presence region (~0.55): the cut. Shimmer shelf on top (~0.55): the glow; small moves only.
- JSON carries the defining band (one `Gain` + `Q`); other bands manually
  by resolved title (same-slot duplicate `Gain` would overwrite).

## Directions

- Ride-forward direction uses cuts around the wash to expose the bell without any boost (cut-to-lift). Thin direction: higher low-cut plus eased mids. Mellow direction: ease top, keep a little mid body.

## Mixer

- `mixer.set / track=Cymbal / param=pan / value=0` as neutral start.
- `mixer.set / track=Cymbal / param=volume / value=-6.0` as start; fix clashes
  with EQ cuts, not with faders.

## Sends

- None by design: study preset stays dry. No `send.set` calls.

## Error recovery

- `Parameter not found` -> try shorter spelling
  (e.g. `Gain`, `Q`).
  Response `param` field shows the resolved Cubase title -- use it next time.
- `Plugin mismatch` -> slot number or plugin name is wrong; check insert order.
- `Select track "Cymbal" in Cubase first` -> select the track, then resend.
