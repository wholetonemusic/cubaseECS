# Tom Long — starting points

Long-tail send for tom fills. Goal: fills bloom into the kit space without
muddying the groove.
Layers onto tom-fill (slot 0 ambience stays); this preset adds the long
hall feed on slot 1. The long hall FX itself is dialed once on the FX track
(tail ~1.5-3.5 s, wet-only) — same destination as Drums group slot 1.
Assumes Tom send slot 1 assigned to the long hall FX. Destination assignment
is one-time manual setup in Cubase (no API).
All values are 0..1 process values. Start points.

Preconditions: `Tom` track selected in Cubase. Apply tom-fill first, then this.
Check `session.status` first.

## Sends (`send.set`)

- `send.set / track=Tom / slot=1 / param=level / value=0.4`, on 1, post-fader.
- Feature fills take more; groove fills take less or bypass via `on` 0.
- If fills wash into each other, pull this send before touching tom-fill.

## Mixer / inserts

- No mixer or insert changes in this preset. It layers onto tom-fill
  without touching it.

## Error recovery

- `Select track "Tom" in Cubase first` -> select the track, then resend.
- Long tail too washy -> shorten the FX tail itself, not just this send.
