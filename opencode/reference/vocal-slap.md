# Vocal Slap — starting points

Slapback delay for vocal phrases. Goal: depth without wash — a single short
repeat sitting behind the lead.
Assumes an FX track named `Delay` with a delay insert in slot 0, and Vocal
send slot 1 assigned to it. FX creation + destination assignment is one-time
manual setup in Cubase (no API).
Calls span two tracks: select `Delay` for the insert/mixer calls, then select
`Vocal` for the send calls. Confirm with `session.status` before each group.
All values are 0..1 process values. Start points — verify time by ear.

## FX setup (one-time, in Cubase)

- Create FX track `Delay`, insert a delay in slot 0.
- On `Vocal`, assign send slot 1 to the Delay FX. Post-fader.

## Delay insert (track `Delay`, slot 0)

- Short slap region (~0.3), low feedback (~0.25: one audible repeat),
  Mix 1.0 (wet-only — the audible amount rides on the send level).
- If it doubles instead of slapping, shorten time.
  If it echoes, lower feedback.

## FX level (track `Delay`)

- `mixer.set / track=Delay / param=volume / value=0.0` (unity fader baseline).

## Send (track `Vocal`, slot 1)

- `send.set / track=Vocal / slot=1 / param=level / value=0.35`, on 1, post-fader.
- Ride per phrase in Cubase; the static level here is the start.

## Error recovery

- `Parameter not found` -> try shorter spelling (e.g. `Delay`, `Feedback`, `Mix`).
  Response `param` field shows the resolved title — use it next time.
- `Plugin mismatch` -> slot 0 holds a different delay; adjust the `plugin`
  name to the actual one (the error tells you what is there).
- `Select track "X" in Cubase first` -> select the track, then send its group.
