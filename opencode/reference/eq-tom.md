# Tom EQ study -- starting points

Round toms with clear attack. Resonance per drum size, one shared attack point, low clutter cut. Goal: Round toms with clear attack, no low clutter.
Order: EQ first (shape before level). All values are 0..1 process values for `plugin.set_param`. Adjust by ear.

Preconditions: `Tom` track selected in Cubase, insert 0 loaded as below.
Send order: slot 0 only. Check `session.status` first.

## Slot layout

| Slot | Plugin | Role |
|------|--------|------|
| 0 | Frequency | Shaping (study bands below) |

## Slot 0 -- shaping EQ

- Resonance lift in the low-mid (~0.6, medium Q): the sing; move frequency per drum size (smaller drum higher, floor drum lower). Core in the mid (~0.55): the firm note. Attack lift in the upper-mid (~0.6, medium-narrow Q): shared across toms. Low-cut at the bottom plus low-mid ease cleans mud and kick/bass overlap.
- JSON carries the defining band (one `Gain` + `Q`); other bands manually
  by resolved title (same-slot duplicate `Gain` would overwrite).

## Directions

- Roomy direction: favor resonance, keep attack small. Fast-fill direction: cut lows harder, favor attack. One EQ per tom when sizes share a track.

## Mixer

- `mixer.set / track=Tom / param=pan / value=0` as neutral start.
- `mixer.set / track=Tom / param=volume / value=-6.0` as start; fix clashes
  with EQ cuts, not with faders.

## Sends

- None by design: study preset stays dry. No `send.set` calls.

## Error recovery

- `Parameter not found` -> try shorter spelling
  (e.g. `Gain`, `Q`).
  Response `param` field shows the resolved Cubase title -- use it next time.
- `Plugin mismatch` -> slot number or plugin name is wrong; check insert order.
- `Select track "Tom" in Cubase first` -> select the track, then resend.
