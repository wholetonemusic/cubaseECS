# Vocal Hall — starting points

Hidden hall program for the shared vocal reverb. Goal: space you feel,
not hear. This preset programs the FX itself; the feeds already live in
vocal-main (Vocal slot 0, 0.2) and vocal-back (Chorus slot 0, 0.3).
Assumes an FX track named `Hall` with a reverb insert in slot 0.
FX creation is one-time manual setup in Cubase (no API).
All values are 0..1 process values. Start points — adjust tail by ear.

## FX setup (one-time, in Cubase)

- Create FX track `Hall`, insert a reverb in slot 0.
- Assign Vocal send slot 0 and Chorus send slot 0 to it. Post-fader.

## Reverb insert (track `Hall`, slot 0)

- Short tail region (~0.3), Mix 1.0 (wet-only — audible amount rides
  on each source's send level).
- If it washes, shorten the tail on the FX — not the send levels first.

## FX level (track `Hall`)

- `mixer.set / track=Hall / param=volume / value=0.0` (unity fader baseline).

## Error recovery

- `Parameter not found` -> try shorter spelling (e.g. `Time`, `Mix`).
  Response `param` field shows the resolved title — use it next time.
- `Plugin mismatch` -> slot 0 holds a different reverb; adjust the `plugin`
  name to the actual one (the error tells you what is there).
- `Select track "Hall" in Cubase first` -> select the track, then resend.
