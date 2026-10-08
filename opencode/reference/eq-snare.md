# Snare EQ study -- starting points

Snare body plus snap with cardboard removed. Weight vs warmth vs snap choices, extreme boosts need input trim. Goal: Snare body plus snap, no cardboard boxiness.
Order: EQ first (shape before level). All values are 0..1 process values for `plugin.set_param`. Adjust by ear.

Preconditions: `Snare` track selected in Cubase, insert 0 loaded as below.
Send order: slot 0 only. Check `session.status` first.

## Slot layout

| Slot | Plugin | Role |
|------|--------|------|
| 0 | Frequency | Shaping (study bands below) |

## Slot 0 -- shaping EQ

- Body in the low fundamental region (~0.6, medium Q): the weight. Warmth/box in the low-mid (~0.5): ease to uncardboard. Attack in the mid attack region (~0.6, medium-narrow Q): the stick point. Snap/air shelf on top (~0.6): the wires. Deep low-cut toward the low-mid removes kick bleed.
- JSON carries the defining band (one `Gain` + `Q`); other bands manually
  by resolved title (same-slot duplicate `Gain` would overwrite).

## Directions

- Warm direction: favor low body, ease box slightly. Crack direction: favor attack plus air, keep body small. Standout direction: ease masking mids, lift top slightly. Extreme narrow boosts distort inside the EQ: trim input first and watch the meter.

## Mixer

- `mixer.set / track=Snare / param=pan / value=0` as neutral start.
- `mixer.set / track=Snare / param=volume / value=-6.0` as start; fix clashes
  with EQ cuts, not with faders.

## Sends

- None by design: study preset stays dry. No `send.set` calls.

## Error recovery

- `Parameter not found` -> try shorter spelling
  (e.g. `Gain`, `Q`).
  Response `param` field shows the resolved Cubase title -- use it next time.
- `Plugin mismatch` -> slot number or plugin name is wrong; check insert order.
- `Select track "Snare" in Cubase first` -> select the track, then resend.
