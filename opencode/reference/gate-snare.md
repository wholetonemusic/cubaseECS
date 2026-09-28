# Gate Snare — starting points

Gated reverb for snare hits. Goal: big room explosion that stops dead —
size with rhythm intact.
Assumes an FX track named `GateVerb` with a reverb in slot 0 and a gate
in slot 1, and Snare send slot 1 assigned to it. FX creation + destination
assignment is one-time manual setup in Cubase (no API).
Calls span two tracks: select `GateVerb` for the insert/mixer calls, then
select `Snare` for the send calls. Confirm with `session.status` before
each group. All values are 0..1 process values. Start points.
Conditional preset: requires a gate-capable plugin in slot 1.

## FX setup (one-time, in Cubase)

- Create FX track `GateVerb`: reverb in slot 0, gate in slot 1.
- On `Snare`, assign send slot 1 to the GateVerb FX. Post-fader.
  (Slot 0 stays on the short ambience from snare-crack.)

## Reverb insert (track `GateVerb`, slot 0)

- Longish tail region (~0.4), Mix 1.0 (wet-only — the gate shapes it next).
- Bigger than tasteful: the gate will cut it down to size.

## Gate insert (track `GateVerb`, slot 1)

- Threshold mid (~0.5): opens on hits, closes after.
- Release short (~0.3): the chop length. Longer for sustain, shorter for snap.
- If it chatters, raise threshold; if it never opens, lower it.

## FX level (track `GateVerb`)

- `mixer.set / track=GateVerb / param=volume / value=0.0` (unity baseline).

## Send (track `Snare`, slot 1)

- `send.set / track=Snare / slot=1 / param=level / value=0.4`, on 1, post-fader.
- Feature fills take more; verses take less or bypass via `on` 0.

## Error recovery

- `Parameter not found` -> try shorter spelling
  (e.g. `Time`, `Mix`, `Threshold`, `Attack`, `Hold`, `Release`).
  Response `param` field shows the resolved title — use it next time.
- `Plugin mismatch` -> slots hold different plugins; adjust the `plugin`
  names to the actual ones (the error tells you what is there).
  No gate plugin available -> skip this preset.
- `Select track "X" in Cubase first` -> select the track, then send its group.
