# Vocal Long — starting points

Lush long hall for ballad vocals. Goal:tails that bloom behind sustained
notes — the hidden hall (vocal-hall) keeps groove verses dry while this one
carries the big moments.
Assumes an FX track named `LongHall` with a hall reverb in slot 0, and Vocal
send slot 3 assigned to it. FX creation + destination assignment is one-time
manual setup in Cubase (no API).
Calls span two tracks: select `LongHall` for the insert/mixer calls, then
select `Vocal` for the send calls. Confirm with `session.status` before
each group. All values are 0..1 process values. Start points.

## FX setup (one-time, in Cubase)

- Create FX track `LongHall`, insert a hall reverb in slot 0.
- On `Vocal`, assign send slot 3 to the LongHall FX. Post-fader.
  (Slot 0 stays on the hidden hall, slot 1 on slap, slot 2 on plate.)

## Long insert (track `LongHall`, slot 0)

- Long tail region (~0.55), Mix 1.0 (wet-only — audible amount rides
  on the send level).
- Clearly longer than the hidden hall: if the two are indistinguishable,
  lengthen this one, not the hidden one.
- If ballad tails step on the next phrase, shorten the FX tail before
  pulling the send.

## FX level (track `LongHall`)

- `mixer.set / track=LongHall / param=volume / value=0.0` (unity baseline).

## Send (track `Vocal`, slot 3)

- `send.set / track=Vocal / slot=3 / param=level / value=0.3`, on 1, post-fader.
- Big choruses feature it; verses bypass via `on` 0 and stay on the hidden hall.

## Error recovery

- `Parameter not found` -> try shorter spelling (e.g. `Time`, `Mix`).
  Response `param` field shows the resolved title — use it next time.
- `Plugin mismatch` -> slot 0 holds a different reverb; adjust the `plugin`
  name to the actual one (the error tells you what is there).
- `Select track "X" in Cubase first` -> select the track, then send its group.
