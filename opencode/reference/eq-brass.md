# Brass EQ study -- starting points

Sax warmth vs standout, trumpet bell vs attack. Harmonic-aware stacking caution with saturation. Goal: Brass punchy with edge, never shrill.
Order: EQ first (shape before level). All values are 0..1 process values for `plugin.set_param`. Adjust by ear.

Preconditions: `Brass` track selected in Cubase, insert 0 loaded as below.
Send order: slot 0 only. Check `session.status` first.

## Slot layout

| Slot | Plugin | Role |
|------|--------|------|
| 0 | Frequency | Shaping (study bands below) |

## Slot 0 -- shaping EQ

- Warmth/body in the low-mid (~0.55, medium Q): the wood and breath. Core in the mid (~0.55): the note center; too much gets boxy. Attack in the upper-mid (~0.6, medium Q): the tongue and bite. Bell/metal on top (~0.6, narrow Q): the shine; also where harshness lives, so ease first when warming.
- JSON carries the defining band (one `Gain` + `Q`); other bands manually
  by resolved title (same-slot duplicate `Gain` would overwrite).

## Directions

- Warm/ballad direction: favored low-mid body, eased/closed top. Standout direction: favored low-mid plus attack with a small vocal-range ease to de-clutter. Bell direction: eased main body slightly, favored top metal. Stacking harmonics (EQ plus saturation plus mastering) distorts fast: boost fundamentals lightly and in decaying steps.

## Mixer

- `mixer.set / track=Brass / param=pan / value=0` as neutral start.
- `mixer.set / track=Brass / param=volume / value=-6.0` as start; fix clashes
  with EQ cuts, not with faders.

## Sends

- None by design: study preset stays dry. No `send.set` calls.

## Error recovery

- `Parameter not found` -> try shorter spelling
  (e.g. `Gain`, `Q`).
  Response `param` field shows the resolved Cubase title -- use it next time.
- `Plugin mismatch` -> slot number or plugin name is wrong; check insert order.
- `Select track "Brass" in Cubase first` -> select the track, then resend.
