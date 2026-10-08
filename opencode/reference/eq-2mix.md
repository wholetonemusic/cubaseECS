# Two-mix EQ study -- starting points

Gentle bus shaping: groove lift, tight lows, vocal depth, hall glow, vintage warmth. Small moves only. Goal: Two-mix glued gently, groove and depth kept.
Order: EQ first (shape before level). All values are 0..1 process values for `plugin.set_param`. Adjust by ear.

Preconditions: `MixBus` track selected in Cubase, insert 0 loaded as below.
Send order: slot 0 only. Check `session.status` first.

## Slot layout

| Slot | Plugin | Role |
|------|--------|------|
| 0 | Frequency | Shaping (study bands below) |

## Slot 0 -- shaping EQ

- Groove lift: small low pillar plus small top glow (~0.55 each): the energy. Tight direction: low-cut plus small low pillar plus small top lift so small speakers still balance. Forward direction: small lifts on low/attack/top pillars. Vocal-back direction: favored low pillar plus attack with eased vocal-range mids. Hall direction: small low bloom plus low ease plus top glow. Vintage direction: small low shelf plus mid presence plus closed top for analog-like warmth.
- JSON carries the defining band (one `Gain` + `Q`); other bands manually
  by resolved title (same-slot duplicate `Gain` would overwrite).

## Directions

- Radio-voice bus effect keeps mids only (both ends closed) for intros. Bus moves stay small: wide gentle lifts beat narrow deep ones. Loudness comes from balance, not from big bus boosts.

## Mixer

- `mixer.set / track=MixBus / param=pan / value=0` as neutral start.
- `mixer.set / track=MixBus / param=volume / value=-6.0` as start; fix clashes
  with EQ cuts, not with faders.

## Sends

- None by design: study preset stays dry. No `send.set` calls.

## Error recovery

- `Parameter not found` -> try shorter spelling
  (e.g. `Gain`, `Q`).
  Response `param` field shows the resolved Cubase title -- use it next time.
- `Plugin mismatch` -> slot number or plugin name is wrong; check insert order.
- `Select track "MixBus" in Cubase first` -> select the track, then resend.
