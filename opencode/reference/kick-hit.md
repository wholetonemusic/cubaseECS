# Kick Hit — starting points

Kick punch preset. Goal: low body plus beater attack, no boxiness.
Order: EQ first (shape), FET second (contain).
All values are 0..1 process values for `plugin.set_param`. Adjust by ear.

Preconditions: `Kick` track selected in Cubase, inserts 0-1 loaded as below.
Send order: slot 0 -> 1. Check `session.status` first.

## Slot layout

| Slot | Plugin | Role |
|------|--------|------|
| 0 | Frequency | Shaping (sub cut + body + box cut + click) |
| 1 | VintageCompressor | Contain (mid ratio, fast) |

## Slot 0 — shaping EQ

- Sub low-cut at the very bottom (~20 Hz region): removes inaudible rumble.
- Body boost in the low thump region (~0.65, medium Q): the weight.
- Boxiness cut in the low-mid (~0.35, medium Q): the tightening move.
- Click boost in the upper-mid attack region (~0.65): the beater definition.
- JSON carries body boost (one `Gain` + `Q`); other bands manually
  by resolved title (same-slot duplicate `Gain` would overwrite).

## Slot 1 — contain comp (FET)

- Mid ratio direction (~0.4 = around 4:1 feeling).
- Medium-fast attack (~0.35, keeps the click), fast release (~0.25,
  recovers before the next hit).
- Aim for firm reduction. If it clicks too much, ease threshold up first.
- Punch switch on if available (`Punch` 1).
- Starting points: Input ~0.55, Ratio ~0.4, Attack ~0.35, Release ~0.25, Punch 1.

## Mixer

- `mixer.set / track=Kick / param=pan / value=0` (center — always).
- `mixer.set / track=Kick / param=volume / value=-6.0` as start; balance
  against bass by ear. Clashes are fixed with EQ, not level.
- Kick gets minimal or no reverb send (keeps it upfront).

## Sends

- None by design: kick stays dry. No `send.set` calls in this preset.

## Error recovery

- `Parameter not found` -> try shorter spelling
  (e.g. `Gain`, `Q`, `Input`, `Ratio`, `Attack`, `Release`, `Punch`).
  Response `param` field shows the resolved Cubase title — use it next time.
- `Plugin mismatch` -> slot number or plugin name is wrong; check insert order.
- `Select track "Kick" in Cubase first` -> select the track, then resend.
