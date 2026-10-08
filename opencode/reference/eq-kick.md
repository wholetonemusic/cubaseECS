# Kick EQ study -- starting points

Kick weight with beater definition, boxiness removed. Tight vs heavy low-end choices, kick/bass separation first. Goal: Kick weight plus beater definition, no boxiness.
Order: EQ first (shape before level). All values are 0..1 process values for `plugin.set_param`. Adjust by ear.

Preconditions: `Kick` track selected in Cubase, insert 0 loaded as below.
Send order: slot 0 only. Check `session.status` first.

## Slot layout

| Slot | Plugin | Role |
|------|--------|------|
| 0 | Frequency | Shaping (study bands below) |

## Slot 0 -- shaping EQ

- Low weight in the deep low region (process ~0.65, medium Q): the foundation. Box cut in the low-mid mud region (~0.35, medium-wide Q): the tightening move that leaves room for bass. Presence lift in the upper-mid attack region (~0.6, medium-narrow Q): the beater point. Top shimmer only a touch; too much turns papery.
- JSON carries the defining band (one `Gain` + `Q`); other bands manually
  by resolved title (same-slot duplicate `Gain` would overwrite).

## Directions

- Tight direction: cut low mud, keep a narrow low pillar plus edge. Heavy direction: lift deep low, ease the low-mid, keep edge small. If kick and bass mask, cut one side first, check both at low monitor level, never just push faders.

## Mixer

- `mixer.set / track=Kick / param=pan / value=0` as neutral start.
- `mixer.set / track=Kick / param=volume / value=-6.0` as start; fix clashes
  with EQ cuts, not with faders.

## Sends

- None by design: study preset stays dry. No `send.set` calls.

## Error recovery

- `Parameter not found` -> try shorter spelling
  (e.g. `Gain`, `Q`).
  Response `param` field shows the resolved Cubase title -- use it next time.
- `Plugin mismatch` -> slot number or plugin name is wrong; check insert order.
- `Select track "Kick" in Cubase first` -> select the track, then resend.
