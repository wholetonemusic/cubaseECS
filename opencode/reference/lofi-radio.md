# LoFi Radio — starting points

Lo-fi radio/megaphone effect. Goal: small narrow band with grit — a featured
effect, not a full-mix tone.
Applies to a dedicated track: duplicate the part onto a track named `LoFi`
in Cubase, select it, then apply. Keeps the clean chain untouched.
All values are 0..1 process values for `plugin.set_param`. Start points.

Preconditions: `LoFi` track selected in Cubase, EQ in slot 0,
distortion in slot 1. Check `session.status` first.

## Slot layout

| Slot | Plugin | Role |
|------|--------|------|
| 0 | Frequency | Bandpass character (cut lows + highs, push mids) |
| 1 | Distortion | Grit (moderate drive, fully wet) |

## Slot 0 — bandpass EQ

- Cut the lows hard (telephone has no bottom) and the highs hard (no air).
- Push the mid band (~0.6, medium Q): the nasal megaphone core.
- JSON carries the mid push (one `Gain` + `Q`); the cuts manually
  by resolved title (same-slot duplicate `Gain` would overwrite).

## Slot 1 — grit

- Moderate drive (~0.5): audible breakup that still reads as the part.
- Mix fully wet (1.0): the effect commits — blend with the clean track's
  fader in Cubase, not inside this preset.
- If it screams, lower drive before touching anything else.
- Starting points: Boost ~0.5, Mix ~1.0.

## Mixer

- `mixer.set / track=LoFi / param=pan / value=0` as neutral start
  (match the source part's placement by ear).
- `mixer.set / track=LoFi / param=volume / value=-6.0` as start; feature
  moments ride up, verses tuck under the clean part.

## Error recovery

- `Parameter not found` -> try shorter spelling
  (e.g. `Gain`, `Q`, `Boost`, `Mix`, `Drive`, `Tone`).
  Response `param` field shows the resolved Cubase title — use it next time.
- `Plugin mismatch` -> slots hold different plugins; adjust the `plugin`
  names to the actual ones (the error tells you what is there).
- `Select track "LoFi" in Cubase first` -> select the track, then resend.
