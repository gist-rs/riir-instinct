#!/usr/bin/env python3
"""One-shot smoke stub: a valid format-v1 TETMLP1 weights file with tiny
deterministic weights (h1=8, h2=8, tanh) — the round-5 A/B bench mechanics
smoke ONLY (blend executes, probe counts, guards read, record writes).
Never a real critic; never used past the smoke."""
import struct, sys

h1, h2 = 8, 8
out = bytearray()
out += b"TETMLP1 "
out += struct.pack("<III", 33, 7, 3)
out += struct.pack("<II", h1, h2)
out += struct.pack("<I", 1)  # tanh hidden

def f64s(vals):
    return b"".join(struct.pack("<d", v) for v in vals)

# scale: lo=-1, hi=+2 per coordinate (the unit-test convention)
out += f64s([-1.0] * 33)
out += f64s([2.0] * 33)

def gen(n, seed):
    return [((i * seed + 11) % 19 - 9) / 48.0 for i in range(n)]

out += f64s(gen(h1 * 43, 3))
out += f64s(gen(h1, 5))
out += f64s(gen(h2 * h1, 7))
out += f64s(gen(h2, 9))
out += f64s(gen(h2, 11))
out += f64s([0.02])

path = sys.argv[1] if len(sys.argv) > 1 else ".raw/tetris_stub_mlp.bin"
with open(path, "wb") as f:
    f.write(out)
print(f"wrote {path} ({len(out)} B, h1={h1} h2={h2} tanh)")
