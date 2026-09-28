# Vocal Tempo — starting points

Tempo delay for vocal phrases. Goal: repeats that breathe with the song —
fill the gaps between phrases without stepping on the next line.
Feed-only preset: shares the `TempoDelay` FX programmed by tempo-dotted8.
Assumes Vocal send slot 4 assigned to the TempoDelay FX. Destination
assignment is one-time manual setup in Cubase (no API).
All values are 0..1 process values. Start points.

Preconditions: `Vocal` track selected in Cubase. Check `session.status` first.

## Sends (`send.set`)

- `send.set / track=Vocal / slot=4 / param=level / value=0.3`, on 1, post-fader.
  (Slot 0 stays on the hidden hall, slot 1 on slap, slot 2 on plate,
  slot 3 on the long hall.)
- Phrase gaps take more; dense verses bypass via `on` 0 and stay dry-ish.
- If repeats fight the lyric, pull this send before touching the FX time.

## Mixer / inserts / FX program

- No mixer, insert, or FX changes in this preset. It layers onto vocal-main
  without touching it. Delay time/feedback stays as tempo-dotted8 programmed
  it; retune there (prefer note-sync on the plugin itself) if the shared
  timing needs changing.

## Error recovery

- `Select track "Vocal" in Cubase first` -> select the track, then resend.
- Too washy -> lower this send, or shorten the shared FX feedback.
