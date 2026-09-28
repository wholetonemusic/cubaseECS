# Tempo Dotted8 — starting points

Dotted-eighth delay for guitar phrases. Goal: rhythmic repeats that lock to
tempo and widen the part without washing it.
Assumes an FX track named `TempoDelay` with a ping-pong/tempo delay in
slot 0, and Guitar send slot 1 assigned to it. FX creation + destination
assignment is one-time manual setup in Cubase (no API).
Calls span two tracks: select `TempoDelay` for the insert/mixer calls, then
select `Guitar` for the send calls. Confirm with `session.status` before
each group. All values are 0..1 process values. Start points.

## FX setup (one-time, in Cubase)

- Create FX track `TempoDelay`, insert a tempo-capable delay in slot 0.
- Set the delay itself to dotted-eighth note sync in Cubase if available;
  the process value below is the start point — verify against the click.
- On `Guitar`, assign send slot 1 to the TempoDelay FX. Post-fader.

## Delay insert (track `TempoDelay`, slot 0)

- Mid delay time region (~0.4), moderate feedback (~0.3: a few repeats),
  Mix 1.0 (wet-only — audible amount rides on the send level).
- If repeats fight the phrase, lower feedback before touching time.
- For widening, offset left/right times slightly in Cubase (a few ms apart).

## FX level (track `TempoDelay`)

- `mixer.set / track=TempoDelay / param=volume / value=0.0` (unity baseline).

## Send (track `Guitar`, slot 1)

- `send.set / track=Guitar / slot=1 / param=level / value=0.3`, on 1, post-fader.
- Feature phrases take more; rhythm bedding takes less. Ride per section.

## Error recovery

- `Parameter not found` -> try shorter spelling
  (e.g. `Delay`, `Feedback`, `Mix`, or `Time` for the delay time).
  Response `param` field shows the resolved title — use it next time.
- `Plugin mismatch` -> slot 0 holds a different delay; adjust the `plugin`
  name to the actual one (the error tells you what is there).
- `Select track "X" in Cubase first` -> select the track, then send its group.
