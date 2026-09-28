# Snare Plate — starting points

Plate sheen for lead snare. Goal: glossy sustain that flatters backbeats —
complements the dry crack (snare-crack) and the chop (gate-snare) instead
of replacing them.
Assumes an FX track named `Plate` with a plate-type reverb in slot 0, and
Snare send slot 2 assigned to it. FX creation + destination assignment is
one-time manual setup in Cubase (no API).
Calls span two tracks: select `Plate` for the insert/mixer calls, then
select `Snare` for the send calls. Confirm with `session.status` before
each group. All values are 0..1 process values. Start points.

## FX setup (one-time, in Cubase)

- Create FX track `Plate`, insert a plate-type reverb in slot 0.
  (Snare slot 0 stays on ambience, slot 1 on gate — slot 2 is free.)
- On `Snare`, assign send slot 2 to the Plate FX. Post-fader.

## Plate insert (track `Plate`, slot 0)

- Mid tail region (~0.35: plate density without hall wash),
  Mix 1.0 (wet-only — audible amount rides on the send level).
- Prefer the plate algorithm/type on the plugin itself in Cubase;
  the process value below is the start point.
- If it washes, shorten the tail on the FX — not the send first.

## FX level (track `Plate`)

- `mixer.set / track=Plate / param=volume / value=0.0` (unity baseline).

## Send (track `Snare`, slot 2)

- `send.set / track=Snare / slot=2 / param=level / value=0.35`, on 1, post-fader.
- Backbeats feature it; ghost notes should stay almost dry — ride per part
  or automate in Cubase.

## Error recovery

- `Parameter not found` -> try shorter spelling (e.g. `Time`, `Mix`).
  Response `param` field shows the resolved title — use it next time.
- `Plugin mismatch` -> slot 0 holds a different reverb; adjust the `plugin`
  name to the actual one (the error tells you what is there).
- `Select track "X" in Cubase first` -> select the track, then send its group.
