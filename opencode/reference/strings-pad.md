# Strings Pad — starting points

String section preset. Goal: lush bed that never buries the vocal —
air on top, no mud below, tube used as saturator (no compression).
All values are 0..1 process values for `plugin.set_param`. Adjust by ear.

Preconditions: `Strings` track selected in Cubase, inserts 0-1 loaded as below.
Send order: slot 0 -> 1. Check `session.status` first.

## Slot layout

| Slot | Plugin | Role |
|------|--------|------|
| 0 | Frequency | Shaping (low-cut + low-mid ease + air) |
| 1 | TubeCompressor | Saturation only (drive color, ~zero reduction) |

## Slot 0 — shaping EQ

- Low-cut toward the low-mid (~120 Hz region): clears bass territory.
- Gentle wide ease in the low-mid (~0.45, wide Q ~0.3): removes blanket tone.
- Air shelf boost on top (~0.65): the bow sheen. If harsh, pull this first.
- JSON carries the air shelf (one `Gain` + `Q`); other bands manually
  by resolved title (same-slot duplicate `Gain` would overwrite).

## Slot 1 — saturator (Tube, no compression)

- This slot compresses nothing: aim for ~zero gain reduction.
- Drive light (~0.3) for warmth; character high (~0.8) so the color sits
  in the upper band and the section never gets buried.
- Input low (~0.4) to stay out of compression.
- Starting points: Input ~0.4, Drive ~0.3, Character ~0.8.

## Mixer

- `mixer.set / track=Strings / param=pan / value=0` as neutral start.
  Section voicings spread L/R per arrangement; keep overall balance even.
- `mixer.set / track=Strings / param=volume / value=-6.0` as start, then tuck
  under the vocal by ear. If it masks, pull level — not vocal EQ.

## Sends (`send.set`)

- Assumes send slot 0 -> hall FX (one-time setup in Cubase).
- `send.set / track=Strings / slot=0 / param=level / value=0.5`, on 1, post-fader.
- Strings blend through shared space; individual dryness is the enemy here.

## Error recovery

- `Parameter not found` -> try shorter spelling
  (e.g. `Gain`, `Q`, `Input`, `Drive`, `Character`).
  Response `param` field shows the resolved Cubase title — use it next time.
- `Plugin mismatch` -> slot number or plugin name is wrong; check insert order.
- `Select track "Strings" in Cubase first` -> select the track, then resend.
