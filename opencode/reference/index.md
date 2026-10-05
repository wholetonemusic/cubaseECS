# Reference index

Paraphrased starting points for Cubase operation. Short notes, not verbatim copies.
No source notation inside these files by project rule.

Send convention: slot 0 = primary space FX (hall/room/ambience),
slot 1 = secondary space FX (long verb or delay).
FX creation + send destination assignment is one-time manual setup in Cubase
(no API for it); presets drive level/on/prepost via `send.set`.

## Files

| File | Use when user says |
|------|--------------------|
| `vocal-main.md` | main vocal should come forward / vocal harshness / vocal chain start |
| `vocal-back.md` | backing chorus should step back / backing covers the main |
| `drum-glue.md` | kit should gel as one / drum bus glue / muddy drums |
| `bass-tight.md` | bass and kick clash / muddy low end / bass tightening |
| `guitar-strum.md` | strum harshness / guitar buried / acoustic strum start |
| `kick-hit.md` | kick needs punch / kick and bass clash / kick body plus click |
| `snare-crack.md` | snare boxy / snare needs crack / snare body plus snap |
| `piano-sing.md` | piano boomy or ringing / lead piano / ballad piano |
| `strings-pad.md` | string bed / strings bury vocal / lush section without mud |
| `brass-hit.md` | horn stabs / brass shrill / punchy section with edge |
| `vocal-slap.md` | vocal slapback / phrase depth without wash / short delay throw |
| `vocal-hall.md` | shared vocal reverb program / hidden hall / space you feel not hear |
| `tom-fill.md` | tom fills blur / round toms with attack / kit fills |
| `overhead-shine.md` | dull cymbals / kick bleed in overheads / airy kit top |
| `hihat-tick.md` | muddy hihat / snare bleed / crisp tick without harshness |
| `elep-keys.md` | boxy electric piano / bell tine buried / keys vs guitar clash |
| `piano-back.md` | backing piano covers vocal / supportive chords / tuck piano back |
| `parallel-ny.md` | drums need weight / NY parallel crush / density under dry kit |
| `tempo-dotted8.md` | rhythmic guitar delay / dotted-eighth / tempo-locked repeats |
| `adt-double.md` | thin vocal / double-tracking / width without echo |
| `lofi-radio.md` | megaphone effect / lo-fi feature / telephone vocal |
| `gate-snare.md` | gated reverb / big snare that stops dead / 80s snap |
| `tom-long.md` | tom fills need bloom / long tail on fills / fill washes into groove |
| `snare-plate.md` | lead snare sheen / glossy backbeats / plate sustain |
| `vocal-plate.md` | vocal gloss / sustained ballad notes / share plate with snare |
| `vocal-long.md` | ballad big chorus / lush vocal tail / hidden hall not enough |
| `vocal-tempo.md` | vocal phrase gaps / tempo repeats on vocal / delay without wash |
| `comp-kick.md` | kick comp finish / natural fat tight drive / beater vs tail |
| `comp-snare.md` | snare comp snap / body balance / rimshot punch |
| `comp-drumbus.md` | kit-bus comp glue / tempo-linked release / gentle bus |
| `comp-bass.md` | bass comp evenness / finger slap synth / attack vs body |
| `comp-guitar.md` | guitar comp stability / cutting lead arp / electric vs acoustic |
| `comp-keys.md` | keys comp bloom / piano rhodes electric-piano / release tone |
| `comp-vocal.md` | vocal comp steadiness / main double chorus / genre base |
| `comp-2mix.md` | two-mix comp glue / shallow master bus / loudness vs depth |
| `analyzer-guide.md` | live spectrum/level readings available / which-eq-first decisions (read-only, no preset JSON) |

## Presets

| File | Content |
|------|---------|
| `../presets/vocal-main.json` | `vocal-main.md` as send-ordered RPC calls (id assigned at send time) |
| `../presets/vocal-back.json` | `vocal-back.md` as send-ordered RPC calls (id assigned at send time) |
| `../presets/drum-glue.json` | `drum-glue.md` as send-ordered RPC calls (id assigned at send time) |
| `../presets/bass-tight.json` | `bass-tight.md` as send-ordered RPC calls (id assigned at send time) |
| `../presets/guitar-strum.json` | `guitar-strum.md` as send-ordered RPC calls (id assigned at send time) |
| `../presets/kick-hit.json` | `kick-hit.md` as send-ordered RPC calls (id assigned at send time) |
| `../presets/snare-crack.json` | `snare-crack.md` as send-ordered RPC calls (id assigned at send time) |
| `../presets/piano-sing.json` | `piano-sing.md` as send-ordered RPC calls (id assigned at send time) |
| `../presets/strings-pad.json` | `strings-pad.md` as send-ordered RPC calls (id assigned at send time) |
| `../presets/brass-hit.json` | `brass-hit.md` as send-ordered RPC calls (id assigned at send time) |
| `../presets/vocal-slap.json` | `vocal-slap.md` as send-ordered RPC calls (id assigned at send time) |
| `../presets/vocal-hall.json` | `vocal-hall.md` as send-ordered RPC calls (id assigned at send time) |
| `../presets/tom-fill.json` | `tom-fill.md` as send-ordered RPC calls (id assigned at send time) |
| `../presets/overhead-shine.json` | `overhead-shine.md` as send-ordered RPC calls (id assigned at send time) |
| `../presets/hihat-tick.json` | `hihat-tick.md` as send-ordered RPC calls (id assigned at send time) |
| `../presets/elep-keys.json` | `elep-keys.md` as send-ordered RPC calls (id assigned at send time) |
| `../presets/piano-back.json` | `piano-back.md` as send-ordered RPC calls (id assigned at send time) |
| `../presets/parallel-ny.json` | `parallel-ny.md` as send-ordered RPC calls (id assigned at send time) |
| `../presets/tempo-dotted8.json` | `tempo-dotted8.md` as send-ordered RPC calls (id assigned at send time) |
| `../presets/adt-double.json` | `adt-double.md` as send-ordered RPC calls (id assigned at send time) |
| `../presets/lofi-radio.json` | `lofi-radio.md` as send-ordered RPC calls (id assigned at send time) |
| `../presets/gate-snare.json` | `gate-snare.md` as send-ordered RPC calls (id assigned at send time) |
| `../presets/tom-long.json` | `tom-long.md` as send-ordered RPC calls (id assigned at send time) |
| `../presets/snare-plate.json` | `snare-plate.md` as send-ordered RPC calls (id assigned at send time) |
| `../presets/vocal-plate.json` | `vocal-plate.md` as send-ordered RPC calls (id assigned at send time) |
| `../presets/vocal-long.json` | `vocal-long.md` as send-ordered RPC calls (id assigned at send time) |
| `../presets/vocal-tempo.json` | `vocal-tempo.md` as send-ordered RPC calls (id assigned at send time) |
| `../presets/comp-kick.json` | `comp-kick.md` as send-ordered RPC calls (id assigned at send time) |
| `../presets/comp-snare.json` | `comp-snare.md` as send-ordered RPC calls (id assigned at send time) |
| `../presets/comp-drumbus.json` | `comp-drumbus.md` as send-ordered RPC calls (id assigned at send time) |
| `../presets/comp-bass.json` | `comp-bass.md` as send-ordered RPC calls (id assigned at send time) |
| `../presets/comp-guitar.json` | `comp-guitar.md` as send-ordered RPC calls (id assigned at send time) |
| `../presets/comp-keys.json` | `comp-keys.md` as send-ordered RPC calls (id assigned at send time) |
| `../presets/comp-vocal.json` | `comp-vocal.md` as send-ordered RPC calls (id assigned at send time) |
| `../presets/comp-2mix.json` | `comp-2mix.md` as send-ordered RPC calls (id assigned at send time) |

## Adding a new preset (repeat per topic)

1. Add `opencode/reference/<name>.md`: slot layout + 0..1 start points + direction semantics + error recovery. Keep it 2-5 KB, ASCII param/track names.
2. Add `opencode/presets/<name>.json`: `{name, description, track, notes, calls:[{method, params}]}` in send order, `session.status` first. Values 0..1 (mixer `volume` in dB is the exception).
3. Register both files in the tables above.
4. Validate: JSON parses, `cargo test` passes (mock), spot-check 1-2 calls against real Cubase when possible.
