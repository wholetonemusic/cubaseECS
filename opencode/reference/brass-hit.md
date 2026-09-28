# Brass Hit — starting points

Horn section preset. Goal: punchy stabs with controlled resonance and
a slightly dirty edge — never shrill.
Order: EQ first (pin the resonance), tube second (punch + dirt).
All values are 0..1 process values for `plugin.set_param`. Adjust by ear.

Preconditions: `Brass` track selected in Cubase, inserts 0-1 loaded as below.
Send order: slot 0 -> 1. Check `session.status` first.

## Slot layout

| Slot | Plugin | Role |
|------|--------|------|
| 0 | Frequency | Cleanup (low-cut + low-mid ease + resonance notch + presence) |
| 1 | TubeCompressor | Punch + dirt (firm, HIGH ratio) |

## Slot 0 — cleanup EQ

- Low-cut toward the low-mid (~90 Hz region): tightens the section bottom.
- Gentle ease in the low-mid (~0.45, medium Q).
- Narrow notch at the horn resonance (upper-mid ~3 kHz region, high Q ~0.75,
  cut ~0.3): find by sweeping a narrow boost, then cut. The anti-shrill move.
- Presence shelf small boost (~0.6): the bite.
- JSON carries the resonance notch (one `Gain` + `Q`); other bands manually
  by resolved title (same-slot duplicate `Gain` would overwrite).

## Slot 1 — punch comp (Tube, HIGH ratio)

- Firm ratio direction (~0.65), fast attack (~0.3), medium release (~0.5).
- Drive moderate (~0.45) for a slightly dirty edge; character mid (~0.45).
- Aim for firm reduction. If it gets nasal, ease drive down first.
- Starting points: Input ~0.55, Attack ~0.3, Release ~0.5, Drive ~0.45.

## Mixer

- `mixer.set / track=Brass / param=pan / value=0` as neutral start.
  Section voicings spread per arrangement; keep overall balance even.
- `mixer.set / track=Brass / param=volume / value=-6.0` as start.
- Stabs should punch through: fix with presence/drive before pushing level.

## Sends (`send.set`)

- Assumes send slot 0 -> short room FX (one-time setup in Cubase).
- `send.set / track=Brass / slot=0 / param=level / value=0.3`, on 1, post-fader.
- Brass needs a little space to sit in — not enough to soften the attacks.

## Error recovery

- `Parameter not found` -> try shorter spelling
  (e.g. `Gain`, `Q`, `Input`, `Attack`, `Release`, `Drive`).
  Response `param` field shows the resolved Cubase title — use it next time.
- `Plugin mismatch` -> slot number or plugin name is wrong; check insert order.
- `Select track "Brass" in Cubase first` -> select the track, then resend.
