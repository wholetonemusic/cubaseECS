# Parallel NY — starting points

New York parallel compression for drums. Goal: explosive weight under an
untouched dry kit — the parallel FX adds density while transients survive
on the dry path.
Key technique: **pre-fader send** (`prepost` 1). The parallel feed ignores
the group fader, so the crush stays constant while balancing.
Assumes an FX track named `Parallel` with a compressor in slot 0, and Drums
group send slot 2 assigned to it. FX creation + destination assignment is
one-time manual setup in Cubase (no API).
Calls span two tracks: select `Parallel` for the comp/mixer calls, then
select `Drums` for the send calls. Confirm with `session.status` before
each group. All values are 0..1 process values. Start points.

## FX setup (one-time, in Cubase)

- Create FX track `Parallel`, insert a compressor in slot 0.
- On `Drums`, assign send slot 2 to the Parallel FX.

## Crush comp (track `Parallel`, slot 0)

- Deep setting: low threshold (~0.3), high ratio (~0.7),
  fast attack (~0.2), fast release (~0.2).
- This path is meant to sound overdone soloed — it blends underneath.
- Starting points: Threshold ~0.3, Ratio ~0.7, Attack ~0.2, Release ~0.2.

## FX level (track `Parallel`)

- `mixer.set / track=Parallel / param=volume / value=0.0` (unity baseline;
  blend rides on the send level).

## Send (track `Drums`, slot 2)

- `send.set / track=Drums / slot=2 / param=level / value=1.0` (full feed),
  on 1, **`param=prepost / value=1` (pre-fader)**.
- Blend by lowering the send level (try 0.5-0.8): the dry kit stays intact.
- If it gets washy, shorten the FX comp release before pulling the send.

## Error recovery

- `Parameter not found` -> try shorter spelling
  (e.g. `Threshold`, `Ratio`, `Attack`, `Release`).
  Response `param` field shows the resolved Cubase title — use it next time.
- `Plugin mismatch` -> slot 0 holds a different comp; adjust the `plugin`
  name to the actual one (the error tells you what is there).
- `Select track "X" in Cubase first` -> select the track, then send its group.
