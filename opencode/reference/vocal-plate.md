# Vocal Plate — starting points

Plate sheen for lead vocal. Goal: glossy top that flatters sustained notes —
shares the `Plate` FX with snare-plate, so vocal and snare sit in one space.
Feed-only preset: the FX itself is programmed by snare-plate.
Assumes Snare send slot 2 destination (`Plate` FX) is set up, and Vocal send
slot 2 assigned to the same FX. Destination assignment is one-time manual
setup in Cubase (no API).
All values are 0..1 process values. Start points.

Preconditions: `Vocal` track selected in Cubase. Apply vocal-main first,
then this. Check `session.status` first.

## Sends (`send.set`)

- `send.set / track=Vocal / slot=2 / param=level / value=0.3`, on 1, post-fader.
  (Slot 0 stays on the hidden hall, slot 1 on slap.)
- Sustained ballad phrases take more; fast verses take less or bypass via `on` 0.
- If the vocal leaves the front row, pull this send first — never push EQ yet.

## Mixer / inserts / FX program

- No mixer, insert, or FX changes in this preset. It layers onto vocal-main
  without touching it. Plate tail/density stays as snare-plate programmed it;
  retune there if the shared space itself needs changing.

## Error recovery

- `Select track "Vocal" in Cubase first` -> select the track, then resend.
- Too glossy -> lower this send before touching the DeEsser.
