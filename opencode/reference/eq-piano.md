# Piano EQ study -- starting points

Warmth without mud, tight lows, clear attack and overtones. Ballad, tight, honky-tonk, and vintage directions. Goal: Piano warmth without mud, attack and overtones clear.
Order: EQ first (shape before level). All values are 0..1 process values for `plugin.set_param`. Adjust by ear.

Preconditions: `Piano` track selected in Cubase, insert 0 loaded as below.
Send order: slot 0 only. Check `session.status` first.

## Slot layout

| Slot | Plugin | Role |
|------|--------|------|
| 0 | Frequency | Shaping (study bands below) |

## Slot 0 -- shaping EQ

- Warmth pillar in the deep low (~0.55, narrow Q): small moves only or it gets woolly. Mud scoop in the low-mid (~0.4, medium Q): the clarifier. Attack in the upper-mid (~0.6, medium Q): the hammer edge. Overtone shelf on top (~0.55): the glow; ease the harsh edge when warming.
- JSON carries the defining band (one `Gain` + `Q`); other bands manually
  by resolved title (same-slot duplicate `Gain` would overwrite).

## Directions

- Warm/ballad direction: small low pillar, eased harsh edge. Tight direction: cut low/low-mid sustain, favor attack. Honky-tonk direction: cut lows/low-mid, favor attack plus top. Vintage direction: favored mids, both ends closed.

## Mixer

- `mixer.set / track=Piano / param=pan / value=0` as neutral start.
- `mixer.set / track=Piano / param=volume / value=-6.0` as start; fix clashes
  with EQ cuts, not with faders.

## Sends

- None by design: study preset stays dry. No `send.set` calls.

## Error recovery

- `Parameter not found` -> try shorter spelling
  (e.g. `Gain`, `Q`).
  Response `param` field shows the resolved Cubase title -- use it next time.
- `Plugin mismatch` -> slot number or plugin name is wrong; check insert order.
- `Select track "Piano" in Cubase first` -> select the track, then resend.
