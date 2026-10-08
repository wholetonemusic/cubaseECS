# Hihat EQ study -- starting points

Crisp tick with air, low clutter gone, never harsh. High low-cut is the defining move. Goal: Crisp tick with air, no low clutter, never harsh.
Order: EQ first (shape before level). All values are 0..1 process values for `plugin.set_param`. Adjust by ear.

Preconditions: `Hihat` track selected in Cubase, insert 0 loaded as below.
Send order: slot 0 only. Check `session.status` first.

## Slot layout

| Slot | Plugin | Role |
|------|--------|------|
| 0 | Frequency | Shaping (study bands below) |

## Slot 0 -- shaping EQ

- High low-cut well into the midrange: removes snare bleed and low clutter; the defining move, do not skip. Waist ease in the low-mid (~0.45): keeps rhythm tidy. Attack axis in the upper-mid (~0.55, medium Q): the tick. Air shelf on top (~0.6): the shimmer; pull first if harsh.
- JSON carries the defining band (one `Gain` + `Q`); other bands manually
  by resolved title (same-slot duplicate `Gain` would overwrite).

## Directions

- Mellow direction: ease waist more, keep air small. Sharp direction: higher cut, small waist ease, modest air lift. Always set cut before judging air.

## Mixer

- `mixer.set / track=Hihat / param=pan / value=0` as neutral start.
- `mixer.set / track=Hihat / param=volume / value=-6.0` as start; fix clashes
  with EQ cuts, not with faders.

## Sends

- None by design: study preset stays dry. No `send.set` calls.

## Error recovery

- `Parameter not found` -> try shorter spelling
  (e.g. `Gain`, `Q`).
  Response `param` field shows the resolved Cubase title -- use it next time.
- `Plugin mismatch` -> slot number or plugin name is wrong; check insert order.
- `Select track "Hihat" in Cubase first` -> select the track, then resend.
