# ADT Double — starting points

Automatic double-tracking thickener. Goal: one voice sounds like two —
width and weight with no audible echo.
**No FX track needed**: this is a pure insert recipe on the source track.
Assumes a short delay (e.g. MonoDelay) loaded in `Vocal` insert slot 5.
All values are 0..1 process values for `plugin.set_param`. Start points.

Preconditions: `Vocal` track selected in Cubase, delay in slot 5.
Check `session.status` first.

## Slot layout

| Slot | Plugin | Role |
|------|--------|------|
| 5 | MonoDelay | Doubling (very short time, no feedback, blended mix) |

## Slot 5 — doubling delay

- Very short time region (~0.25: doubling zone, well below slap).
  If it flanges, shorten slightly; if it echoes, it is far too long.
- Feedback near zero (~0.1: single pass, no repeats).
- Mix below half (~0.4): the delay sits under the dry voice, never equal.
- If phasey/hollow in mono, lower the mix first.

## Mixer / sends

- No mixer or send changes in this preset. It layers onto vocal-main
  (slots 0-4) without touching it — apply vocal-main first, then this.

## Error recovery

- `Parameter not found` -> try shorter spelling (e.g. `Delay`, `Feedback`, `Mix`).
  Response `param` field shows the resolved Cubase title — use it next time.
- `Plugin mismatch` -> slot 5 holds something else; adjust the `plugin`
  name to the actual delay (the error tells you what is there).
- `Select track "Vocal" in Cubase first` -> select the track, then resend.
