# Comp Mixbus — starting points

Two-mix comp-only starter. Goal: glue and depth with the
shallowest reduction in the whole set (1-2 dB moves the record).
Insert on the master bus; loudness comes from balance, not depth.
All values are 0..1 process values for `plugin.set_param`. Adjust by ear.

Preconditions: `Mixbus` master/group track selected in Cubase,
stock Compressor loaded in slot 0. Check `session.status` first.

## Slot layout

| Slot | Plugin | Role |
|------|--------|------|
| 0 | Compressor | Record glue (shallowest) |

## Slot 0 — record glue

- Natural base: Threshold ~0.7 (shallowest, 1-2 dB),
  Ratio ~0.35 (low), Attack ~0.5 (lets kick/snare heads out),
  Release ~0.35. Start here for any genre.
- Loudness direction: Threshold ~0.55, Ratio ~0.35,
  Attack ~0.55, Release ~0.55 (longer, pushes forward).
  If low end pumps, ease threshold up first.
- Width/depth direction: Attack ~0.7 (slow, misses rhythmic
  heads), Release ~0.75 (long, lifts room/reverb tails).
  Small steps; 1 dB already changes depth a lot.
- Punch direction (rock/club): Ratio ~0.55 (high),
  Threshold ~0.6 (still shallow), Attack ~0.4,
  Release ~0.3 (short, crisp). If snare sticks out too far,
  lengthen attack slightly.
- Advanced routings are manual (no API): kick-triggered
  ducking, pre-comp EQ shaping, mid/side split, multiband
  split, and final ceiling limiting all live outside this
  single-slot preset. Nail the shallow base first.
- Starting points: Threshold ~0.7, Ratio ~0.35,
  Attack ~0.5, Release ~0.35.

## Mixer

- `mixer.set / track=Mixbus / param=pan / value=0` (stereo bus stays center).
- `mixer.set / track=Mixbus / param=volume / value=0.0` as start
  (master fader stays at unity; balance members instead).
- Makeup is manual by resolved title: match bypassed loudness only.

## Sends

- None by design: master bus stays dry. No `send.set` calls in this preset.

## Error recovery

- `Parameter not found` -> try shorter spelling
  (e.g. `Threshold`, `Ratio`, `Attack`, `Release`).
  Response `param` field shows the resolved Cubase title — use it next time.
- `Plugin mismatch` -> slot number or plugin name is wrong; check insert order.
- `Select track "Mixbus" in Cubase first` -> select the track, then resend.
