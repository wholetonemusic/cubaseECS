# Comp Drumbus — starting points

Kit-bus comp-only starter. Goal: whole kit gels as one unit.
Gentle by design: shallow threshold, low ratio, small reduction.
Tempo sets release (slow songs longer, fast songs shorter).
All values are 0..1 process values for `plugin.set_param`. Adjust by ear.

Preconditions: `Drums` group track selected in Cubase, kit tracks
routed to it, stock Compressor loaded in slot 0.
Check `session.status` first.

## Slot layout

| Slot | Plugin | Role |
|------|--------|------|
| 0 | Compressor | Bus glue (shallow, low ratio) |

## Slot 0 — bus glue

- Base direction: Threshold ~0.65 (shallow, a couple dB reduction),
  Ratio ~0.3 (low), Attack ~0.35 (fast-mid, catches peaks but
  keeps stick transients), Release tempo-linked (see below).
- Slow-feel songs: Release ~0.4 (longer, laid-back tail).
- Up-tempo songs: Release ~0.2 (shorter, tightens the groove).
  If the bus pumps on kick hits, ease threshold up first —
  never fix it with the kick fader.
- Club-oriented loops: Ratio ~0.5 with the same shallow threshold
  for extra bounce. Back off at the first sign of harsh hats.
- Single hand-percussion tracks (not the bus): same slot works
  hotter — Threshold ~0.35, Ratio ~0.7, Release matched to the
  instrument tail. Keep the bus itself gentle.
- Starting points: Threshold ~0.65, Ratio ~0.3,
  Attack ~0.35, Release ~0.3.

## Mixer

- `mixer.set / track=Drums / param=pan / value=0` (group stays center).
- `mixer.set / track=Drums / param=volume / value=-6.0` as start.
- Makeup is manual by resolved title: match bypassed loudness only.

## Sends (`send.set`)

- Assumes slot 0 -> short ambience FX (one-time setup in Cubase).
- `send.set / track=Drums / slot=0 / param=level / value=0.2`, on 1, post-fader.
- Refine per piece afterwards on the member tracks, not the bus.

## Error recovery

- `Parameter not found` -> try shorter spelling
  (e.g. `Threshold`, `Ratio`, `Attack`, `Release`).
  Response `param` field shows the resolved Cubase title — use it next time.
- `Plugin mismatch` -> slot number or plugin name is wrong; check insert order.
- `Select track "Drums" in Cubase first` -> select the group track, then resend.
