# Strings EQ study -- starting points

Ensemble weight vs edge, violin bow and harmonics. Fast passages need edge, lush beds need weight. Goal: Strings lush without mud, solos forward without harshness.
Order: EQ first (shape before level). All values are 0..1 process values for `plugin.set_param`. Adjust by ear.

Preconditions: `Strings` track selected in Cubase, insert 0 loaded as below.
Send order: slot 0 only. Check `session.status` first.

## Slot layout

| Slot | Plugin | Role |
|------|--------|------|
| 0 | Frequency | Shaping (study bands below) |

## Slot 0 -- shaping EQ

- Ensemble weight: small low shelf plus low-mid body (~0.55): the hall and wood. Ensemble edge: eased low/low-mid plus mid bow lift (~0.55) plus top edge (~0.6, narrow Q): for fast lines. Violin harmonics shelf on top (~0.6): the beauty; watch the meter. Bow/bite in the upper-mid (~0.6): the rosined attack for exposed lines.
- JSON carries the defining band (one `Gain` + `Q`); other bands manually
  by resolved title (same-slot duplicate `Gain` would overwrite).

## Directions

- Deep-bed direction: favor low shelf plus body. Fast-line direction: cut lows, favor bow plus top. Country direction: cut low clutter, favor outline plus glow. Tango direction: balanced low/mid/top pillars for drama across registers.

## Mixer

- `mixer.set / track=Strings / param=pan / value=0` as neutral start.
- `mixer.set / track=Strings / param=volume / value=-6.0` as start; fix clashes
  with EQ cuts, not with faders.

## Sends

- None by design: study preset stays dry. No `send.set` calls.

## Error recovery

- `Parameter not found` -> try shorter spelling
  (e.g. `Gain`, `Q`).
  Response `param` field shows the resolved Cubase title -- use it next time.
- `Plugin mismatch` -> slot number or plugin name is wrong; check insert order.
- `Select track "Strings" in Cubase first` -> select the track, then resend.
