# Effects EQ study -- starting points

Pre-EQ before modulation, reverb low/high shaping, delay tuck, wah rings. Effect character over level. Goal: Effects characterful, originals intact.
Order: EQ first (shape before level). All values are 0..1 process values for `plugin.set_param`. Adjust by ear.

Preconditions: `Effects` track selected in Cubase, insert 0 loaded as below.
Send order: slot 0 only. Check `session.status` first.

## Slot layout

| Slot | Plugin | Role |
|------|--------|------|
| 0 | Frequency | Shaping (study bands below) |

## Slot 0 -- shaping EQ

- Pre-shape before the effect (one small mid/upper-mid lift ~0.55): feeds flanger/chorus/wah/phaser so the sweep bites. Reverb: ease deep lows plus low-mid weight plus top shimmer to keep the dry forward and the tail tidy; snare verbs favor low body, ease boxy mids, keep small top glow. Delay: favor low-mid warmth slightly, close top to tuck repeats behind phrases. Distortion bass: small low pillars first so lows and highs distort evenly. Wah: three small rings (low, mid, upper-mid) to open the sweep everywhere.
- JSON carries the defining band (one `Gain` + `Q`); other bands manually
  by resolved title (same-slot duplicate `Gain` would overwrite).

## Directions

- Shape-then-effect order matters: EQ into the effect, not just after. When a tail clouds, cut the effect return rather than pushing the dry.

## Mixer

- `mixer.set / track=Effects / param=pan / value=0` as neutral start.
- `mixer.set / track=Effects / param=volume / value=-6.0` as start; fix clashes
  with EQ cuts, not with faders.

## Sends

- None by design: study preset stays dry. No `send.set` calls.

## Error recovery

- `Parameter not found` -> try shorter spelling
  (e.g. `Gain`, `Q`).
  Response `param` field shows the resolved Cubase title -- use it next time.
- `Plugin mismatch` -> slot number or plugin name is wrong; check insert order.
- `Select track "Effects" in Cubase first` -> select the track, then resend.
