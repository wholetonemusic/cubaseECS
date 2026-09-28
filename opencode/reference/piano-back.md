# Piano Back — starting points

Backing piano preset. Goal: supportive bed that steps back from the lead
vocal — same skeleton as piano-sing, deeper cleanup and firmer control.
All values are 0..1 process values for `plugin.set_param`. Adjust by ear.

Preconditions: `Piano` track selected in Cubase, inserts 0-1 loaded as below.
Send order: slot 0 -> 1. Check `session.status` first.

## Slot layout

| Slot | Plugin | Role |
|------|--------|------|
| 0 | Frequency | Cleanup (deeper cut + scoop) |
| 1 | VintageCompressor | Firm control (high ratio, fast) |

## Slot 0 — cleanup EQ

- Low-cut deeper than the lead variant: clears bass and vocal territory.
- Scoop in the muddy low-mid (~0.35, medium Q): the stepping-back move.
- Keep a small presence edge so chords stay readable — without it the part
  disappears entirely.
- JSON carries the scoop (one `Gain` + `Q`); other bands manually
  by resolved title (same-slot duplicate `Gain` would overwrite).

## Slot 1 — firm comp (FET, high ratio)

- High ratio direction (~0.7 = around 8:1 feeling).
- Medium-fast attack (~0.35), fast release (~0.25).
- Aim for firm reduction. If it pumps with the chords, ease threshold up first.
- Punch switch on if available (`Punch` 1).
- Starting points: Input ~0.6, Ratio ~0.7, Attack ~0.35, Release ~0.25, Punch 1.

## Mixer

- `mixer.set / track=Piano / param=pan / value=0` as neutral start.
  Ensemble backing spreads per arrangement.
- `mixer.set / track=Piano / param=volume / value=-6.0` as start, then tuck
  under the vocal by ear.

## Sends (`send.set`)

- Assumes send slot 0 -> medium hall FX (one-time setup in Cubase).
- `send.set / track=Piano / slot=0 / param=level / value=0.35`, on 1, post-fader.
- Slightly less than the lead variant — distance comes from level + EQ,
  not from drowning it.

## Error recovery

- `Parameter not found` -> try shorter spelling
  (e.g. `Gain`, `Q`, `Input`, `Ratio`, `Attack`, `Release`, `Punch`).
  Response `param` field shows the resolved Cubase title — use it next time.
- `Plugin mismatch` -> slot number or plugin name is wrong; check insert order.
- `Select track "Piano" in Cubase first` -> select the track, then resend.
