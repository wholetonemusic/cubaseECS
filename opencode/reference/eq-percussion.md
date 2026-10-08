# Percussion EQ study -- starting points

Shaker edge, conga snap vs resonance, tambourine skin vs jingle. Pan apart when ranges overlap drums. Goal: Percussion that grooves without crowding drums.
Order: EQ first (shape before level). All values are 0..1 process values for `plugin.set_param`. Adjust by ear.

Preconditions: `Percussion` track selected in Cubase, insert 0 loaded as below.
Send order: slot 0 only. Check `session.status` first.

## Slot layout

| Slot | Plugin | Role |
|------|--------|------|
| 0 | Frequency | Shaping (study bands below) |

## Slot 0 -- shaping EQ

- Shaker: ease low-mid honk (~0.45), lift presence edge (~0.55) for light ticks or ease top for mellow shakes. Conga: ease low-mid woof (~0.4), lift upper-mid slap (~0.6) for attack or lift low plus low-mid warmth for resonance. Tambourine: ease low-mid cloud (~0.35), lift top jingle (~0.6) for bright or lift low skin (~0.55) and ease top for skin-forward.
- JSON carries the defining band (one `Gain` + `Q`); other bands manually
  by resolved title (same-slot duplicate `Gain` would overwrite).

## Directions

- Light/bright vs mellow/warm are opposite pairings of the same two bands. Wide-range tambourine can carry a groove alone: small low pillar plus eased vocal-range mids plus top glow.

## Mixer

- `mixer.set / track=Percussion / param=pan / value=0` as neutral start.
- `mixer.set / track=Percussion / param=volume / value=-6.0` as start; fix clashes
  with EQ cuts, not with faders.

## Sends

- None by design: study preset stays dry. No `send.set` calls.

## Error recovery

- `Parameter not found` -> try shorter spelling
  (e.g. `Gain`, `Q`).
  Response `param` field shows the resolved Cubase title -- use it next time.
- `Plugin mismatch` -> slot number or plugin name is wrong; check insert order.
- `Select track "Percussion" in Cubase first` -> select the track, then resend.
