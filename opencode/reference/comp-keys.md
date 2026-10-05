# Comp Keys — starting points

Keys comp-only starter. Goal: control wide dynamics without
flattening the performance. Release is the tone control:
short = crisp, long = bloom. High ratios are safe on
overtone-rich electric pianos, risky on acoustic piano.
All values are 0..1 process values for `plugin.set_param`. Adjust by ear.

Preconditions: `Keys` track selected in Cubase, stock Compressor
loaded in slot 0. Check `session.status` first.

## Slot layout

| Slot | Plugin | Role |
|------|--------|------|
| 0 | Compressor | Dynamics vs bloom |

## Slot 0 — dynamics vs bloom

- Piano backing base: Threshold ~0.55 (a few dB reduction),
  Ratio ~0.45 (mid), Attack ~0.5 (keeps hammer heads),
  Release ~0.25 (short, crisp). Start here.
- Piano processed edge: Ratio ~0.6, Threshold ~0.5,
  Attack ~0.45, Release ~0.2. If it barks, ease threshold
  up before lengthening attack.
- Rhodes swell: Attack ~0.55 (lets heads jump out),
  Release ~0.5 (long, stretches the low bloom).
  Hunt the release sweet spot by ear in small steps.
- Rhodes riff swing: Attack ~0.7 (slow, hooks on the tail),
  Release ~0.25. Heads loud, tails tucked = swing feel.
- Electric-piano glue: Ratio ~0.7 (high), Threshold ~0.5,
  Attack ~0.4, Release ~0.35. Overtone-rich sources stay
  musical under deep settings; acoustic piano does not.
- Velocities first: even out MIDI velocities before comp;
  comp shapes tone, not performance gaps.
- Starting points: Threshold ~0.55, Ratio ~0.45,
  Attack ~0.5, Release ~0.3.

## Mixer

- `mixer.set / track=Keys / param=pan / value=0` as start
  (spread L/R later only when colliding with guitar at center).
- `mixer.set / track=Keys / param=volume / value=-6.0` as start.
- Makeup is manual by resolved title: match bypassed loudness only.

## Sends (`send.set`)

- Assumes send slot 0 -> small room FX (one-time setup in Cubase).
- `send.set / track=Keys / slot=0 / param=level / value=0.25`, on 1, post-fader.

## Error recovery

- `Parameter not found` -> try shorter spelling
  (e.g. `Threshold`, `Ratio`, `Attack`, `Release`).
  Response `param` field shows the resolved Cubase title — use it next time.
- `Plugin mismatch` -> slot number or plugin name is wrong; check insert order.
- `Select track "Keys" in Cubase first` -> select the track, then resend.
