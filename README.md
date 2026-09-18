# Cubase External Control System (cubaseECS)

Control Cubase with natural language. This project exposes Cubase mixer, plugin,
and transport operations as an external API so that a chat UI (OpenCode) can
drive them through structured commands.

## Architecture

```text
OpenCode Chat UI (natural language)
        |
        v  JSON-RPC 2.0  (POST /rpc)
Rust JSON-RPC API Server  (rust-api/)
        |
        v  SysEx(JSON): F0 7D <JSON> F7
Cubase MIDI Remote Script  (cubase-remote/CubaseECS.js)
        |
        v
Cubase Mixer / Plugins / Transport
```

See `docs/cubase-external-control-system-plan.md` for the full design
(JSON-RPC spec, OpenCode prompt template, Rust module layout, Cubase script
examples, roadmap) and `docs/session-log.md` for the session history.

## Repository layout

| Path            | Description                                                      |
| --------------- | ---------------------------------------------------------------- |
| `rust-api/`     | Rust API server: `POST /rpc`, JSON-RPC dispatch, SysEx encode/decode |
| `cubase-remote/`| Cubase MIDI Remote script stub + setup notes (Phase 2 target)    |
| `opencode/`     | OpenCode prompt template (natural language to JSON-RPC)          |
| `docs/`         | Design doc and session log (Japanese)                            |
| `docker-compose.yml` | Dev environment (hot reload via `cargo-watch`)              |

## Prerequisites

- Windows + Docker Desktop (daemon running)
- Git
- For real Cubase connection (Phase 2): a host-native Rust toolchain with the
  `host-midi` feature. Docker containers cannot reach host MIDI ports on
  Windows, so Phase 1 runs against a mock MIDI port.

## Quick start (mock mode)

```powershell
docker compose up --build
```

Health check:

```powershell
Invoke-RestMethod -Uri http://localhost:3001/healthz
```

Send a command (example: set Vocal volume):

```powershell
Invoke-RestMethod -Method Post -Uri http://localhost:3001/rpc `
  -ContentType "application/json" `
  -Body '{"jsonrpc":"2.0","method":"mixer.set","params":{"track":"Vocal","param":"volume","value":-6.0},"id":1}'
```

Run tests:

```powershell
docker compose run --rm api cargo test
```

Stop:

```powershell
docker compose down
```

## JSON-RPC API (`POST /rpc`, default port 3001)

| Method            | Params                                                        | Description              |
| ----------------- | ------------------------------------------------------------- | ------------------------ |
| `mixer.set`       | `{ track, param, value }`                                     | Mixer parameter change   |
| `plugin.set_param`| `{ track, slot, plugin, param, value }`                       | Plugin parameter change  |
| `command.exec`    | `{ id }` (e.g. `"Transport_Play"`)                            | Execute a Cubase command |
| `session.status`  | `{}`                                                          | Get session status       |

Requests and responses follow JSON-RPC 2.0. Unknown methods return `-32601`,
invalid params return `-32602`. In Phase 1 the dispatcher replies with a mock
result (`mode: "mock"`); waiting for real Cubase SysEx responses is Phase 2 work.

### SysEx encoding

JSON envelopes are encoded as `F0 7D <7-bit ASCII JSON> F7` (`0x7D` =
development/non-commercial ID). Non-ASCII payloads (e.g. Japanese track names)
are rejected at encode time; normalize track names to Latin script
(see `opencode/prompt.md`).

## MIDI modes

- `MIDI_MODE=mock` (default, used in Docker): `MockPort`, no hardware access.
- `MIDI_MODE=host` + `cargo run --features host-midi` (host-native only):
  `HostMidiPort` skeleton, real send/receive to be implemented in Phase 2.

## Cubase setup (Phase 2)

1. Copy `cubase-remote/CubaseECS.js` into the Cubase MIDI Remote scripts folder.
2. Enable the script in Cubase's MIDI Remote Manager.
3. Start the API on the host with `MIDI_MODE=host`.
4. Send `command.exec / Transport_Play` to `POST http://localhost:3001/rpc`.

Details: `cubase-remote/README.md`.

## OpenCode usage

Load `opencode/prompt.md` as the assistant prompt. It instructs the model to
translate user utterances into JSON-RPC commands sent to
`POST http://localhost:3001/rpc`. Example:

> "thicken the vocal comp" -> `plugin.set_param / track=Vocal /
> plugin=Compressor / param=Threshold / value=-6`

## Toolchain notes

- Pinned to Rust 1.82 (`rust:1.82-slim`, `rust-version = "1.82"`).
  Dependencies are version-pinned in `rust-api/Cargo.toml` because recent
  crates (e.g. `time-core 0.1.9`) require `edition2024`, which Cargo 1.82
  cannot parse. When upgrading the toolchain, unpin and re-resolve.
- `Cargo.lock` is committed for reproducible builds.

## Roadmap

- **Phase 1** (done): JSON-RPC DSL, `/rpc` + SysEx mock, Docker dev env, prompt/scaffold.
- **Phase 2**: Cubase Remote script implementation, real MIDI send/receive,
  `transport.play` / `mixer.set` / `plugin.set_param` verification.
- **Phase 3**: error handling, `session.status`, chat-driven operation.
- **Phase 4**: voice UI, auto-mix, presets, external integrations.
