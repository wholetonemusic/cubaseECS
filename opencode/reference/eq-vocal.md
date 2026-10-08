# Vocal EQ study -- starting points

Mud cut plus presence lift, pillar vs edge vs air. Filter voices and male/female directions without harshness. Goal: Vocal forward and intelligible, never harsh.
Order: EQ first (shape before level). All values are 0..1 process values for `plugin.set_param`. Adjust by ear.

Preconditions: `Vocal` track selected in Cubase, insert 0 loaded as below.
Send order: slot 0 only. Check `session.status` first.

## Slot layout

| Slot | Plugin | Role |
|------|--------|------|
| 0 | Frequency | Shaping (study bands below) |

## Slot 0 -- shaping EQ

- Low-cut at the bottom plus low-mid mud ease (~0.4, medium-narrow Q): the clarifier. Pillar in the low-mid to mid (~0.5): the chest and note center; small moves. Axis/edge in the upper-mid to presence (~0.6, medium Q): the intelligibility; the forward lever. Air shelf on top (~0.55): the breath; also where sibilance and lip noise live.
- JSON carries the defining band (one `Gain` + `Q`); other bands manually
  by resolved title (same-slot duplicate `Gain` would overwrite).

## Directions

- Forward direction: eased low-mid plus favored axis/edge/air. Fat direction: favored pillar with eased surroundings plus small air. Filter voices: radio keeps mids only (both ends closed); whisper closes resonance plus eases attack; old-mic stacks gentle high eases then saturates lightly. Narration: low-cut plus deep narrow mud ease plus small edge lift.

## Mixer

- `mixer.set / track=Vocal / param=pan / value=0` as neutral start.
- `mixer.set / track=Vocal / param=volume / value=-6.0` as start; fix clashes
  with EQ cuts, not with faders.

## Sends

- None by design: study preset stays dry. No `send.set` calls.

## Error recovery

- `Parameter not found` -> try shorter spelling
  (e.g. `Gain`, `Q`).
  Response `param` field shows the resolved Cubase title -- use it next time.
- `Plugin mismatch` -> slot number or plugin name is wrong; check insert order.
- `Select track "Vocal" in Cubase first` -> select the track, then resend.
