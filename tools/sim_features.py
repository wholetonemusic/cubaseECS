#!/usr/bin/env python3
"""sim_features.py — AI Audio Analyzer writer simulator (no Cubase needed).

Writes synthetic 10 Hz frames into the same OS shared memory the VST3 uses
(`CubaseECS_AudioFeatures_v1`, 1 KB, seqlock), so `analyzer.get_features`
in host mode can be exercised end-to-end on the Windows host:

  1. Start the API on the host: MIDI_MODE=host cargo run --features host-midi
  2. Run: python tools/sim_features.py
  3. POST {"jsonrpc":"2.0","method":"analyzer.get_features","params":{},"id":7}
     to http://localhost:3001/rpc and confirm mode=host-midi, stale=false.

Windows-only (uses mmap tagname = CreateFileMapping). Ctrl+C to stop.
"""
import mmap
import struct
import sys
import time

SHM_NAME = "CubaseECS_AudioFeatures_v1"
SHM_SIZE = 1024
MAGIC = 0x45435346
VERSION = 1

# repr(C): I,H,H,I + 10f, then 4 pad bytes (u64 alignment), Q, 960 pad.
FMT = "<I H H I 10f 4x Q 960x"
assert struct.calcsize(FMT) == SHM_SIZE, struct.calcsize(FMT)


def frame_at(t: float):
    import math

    # Gentle 0.2 Hz level wander + 2 Hz transient pulse.
    wander = 0.5 + 0.5 * math.sin(2 * math.pi * 0.2 * t)
    rms = -24.0 + 12.0 * wander
    peak = rms + 12.0
    crest = 12.0
    low = -30.0 + 4.0 * wander
    lowmid = -22.0 + 3.0 * wander
    mid = -18.0 + 2.0 * wander
    highmid = -16.0 + 2.0 * wander
    high = -15.5 + 1.5 * wander
    pulse = max(0.0, math.sin(2 * math.pi * 2.0 * t)) ** 8
    transient = 0.05 + 0.5 * pulse
    width = 0.1 + 0.05 * math.sin(2 * math.pi * 0.1 * t)
    ts = int(time.time() * 1000)
    return (rms, peak, crest, low, lowmid, mid, highmid, high, transient, width, ts)


def main() -> int:
    if sys.platform != "win32":
        print("Windows-only (needs mmap tagname).", file=sys.stderr)
        return 2
    try:
        mm = mmap.mmap(-1, SHM_SIZE, tagname=SHM_NAME)
    except OSError as e:
        print(f"cannot create mapping: {e}", file=sys.stderr)
        return 1
    print(f"writing 10 Hz frames to {SHM_NAME} (Ctrl+C to stop)...")
    seq = 0
    t0 = time.time()
    try:
        while True:
            t = time.time() - t0
            vals = frame_at(t)
            seq = (seq + 1) | 1  # odd = writing
            mm.seek(0)
            mm.write(struct.pack(FMT, MAGIC, VERSION, 0, seq, *vals))
            seq += 1  # even = stable
            mm.seek(8)
            mm.write(struct.pack("<I", seq))
            time.sleep(0.1)
    except KeyboardInterrupt:
        pass
    finally:
        mm.close()
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
