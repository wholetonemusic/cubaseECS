# Keyboard EQ study -- starting points

EP tine, organ snap vs fat, synth strings/brass/bass roles. Leave each part its lane in the arrangement. Goal: Keys with role clarity, no band crowding.
Order: EQ first (shape before level). All values are 0..1 process values for `plugin.set_param`. Adjust by ear.

Preconditions: `Keys` track selected in Cubase, insert 0 loaded as below.
Send order: slot 0 only. Check `session.status` first.

## Slot layout

| Slot | Plugin | Role |
|------|--------|------|
| 0 | Frequency | Shaping (study bands below) |

## Slot 0 -- shaping EQ

- Electric piano: mid axis (~0.55) for forward tone, top edge (~0.6) for tine/bell. Organ: upper-mid lift (~0.6) for percussive snap; fat version adds low/low-mid pillars and closes top. Synth strings: eased low-mid plus edged top for grain, or low pillars plus eased mids for weight. Synth brass: mid spine plus attack plus top shelf for bright, or low/mid pillars plus attack for heavy. Synth bass: low pillar plus eased conflict mids plus edged top to coexist with electric bass.
- JSON carries the defining band (one `Gain` + `Q`); other bands manually
  by resolved title (same-slot duplicate `Gain` would overwrite).

## Directions

- Forward-dry parts favor axis/attack; lush parts favor low pillars with eased mids. Always check keys against guitar/vocal lanes before adding top.

## Mixer

- `mixer.set / track=Keys / param=pan / value=0` as neutral start.
- `mixer.set / track=Keys / param=volume / value=-6.0` as start; fix clashes
  with EQ cuts, not with faders.

## Sends

- None by design: study preset stays dry. No `send.set` calls.

## Error recovery

- `Parameter not found` -> try shorter spelling
  (e.g. `Gain`, `Q`).
  Response `param` field shows the resolved Cubase title -- use it next time.
- `Plugin mismatch` -> slot number or plugin name is wrong; check insert order.
- `Select track "Keys" in Cubase first` -> select the track, then resend.
