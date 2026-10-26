#!/usr/bin/env python3
# NONOS Operating System
# Copyright (C) 2026 NONOS Contributors
#
# This program is free software: you can redistribute it and/or modify
# it under the terms of the GNU Affero General Public License as published by
# the Free Software Foundation, either version 3 of the License, or
# (at your option) any later version.
#
# This program is distributed in the hope that it will be useful,
# but WITHOUT ANY WARRANTY; without even the implied warranty of
# MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
# GNU Affero General Public License for more details.
"""The committed image ceiling admits every catalogue capsule's ceiling, no more (B-02).

A make-built kernel bakes nonos-data/trust/policy/image_capability_ceiling.bin
as committed (build.rs); nix writes its own per profile. The value had stayed
at 0x7FFFFFFF after capabilities passed bit 31, so a make-built desktop
refused input_router (InputConsumer) and seven more capsules.
"""

import argparse
import json
import struct
import sys
from pathlib import Path


def union(catalogue: list) -> int:
    u = 0
    for e in catalogue:
        u |= int(e["caps_ceiling"], 16)
    return u


def problems(blob: bytes, catalogue: list) -> list:
    if len(blob) != 8:
        return [f"the ceiling file is {len(blob)} bytes, not 8"]
    have, want = struct.unpack("<Q", blob)[0], union(catalogue)
    out = []
    if want & ~have:
        names = [e["slug"] for e in catalogue if int(e["caps_ceiling"], 16) & ~have]
        out.append(f"ceiling {have:#x} refuses bits {want & ~have:#x}: {' '.join(names)}")
    if have & ~want:
        out.append(f"ceiling {have:#x} admits bits {have & ~want:#x} no capsule declares")
    return out


def main() -> int:
    ap = argparse.ArgumentParser(description=__doc__.split("\n")[0])
    ap.add_argument("--self-test", action="store_true")
    a = ap.parse_args()
    blob = Path("nonos-data/trust/policy/image_capability_ceiling.bin").read_bytes()
    cat = json.loads(Path("tools/nix/capsules.json").read_text())
    if a.self_test:
        grown = [dict(cat[0], caps_ceiling=hex(int(cat[0]["caps_ceiling"], 16) | 1 << 40))] + cat[1:]
        cases = [("as committed", blob, cat, 0), ("a capsule gains a bit", blob, grown, 1),
                 ("InputConsumer dropped", struct.pack("<Q", union(cat) & ~(1 << 36)), cat, 1),
                 ("the old 31-bit value", struct.pack("<Q", 0x7FFFFFFF), cat, 2),
                 ("a short file", blob[:4], cat, 1)]
        bad = [w for w, b, c, n in cases if len(problems(b, c)) != n]
        print(f"image-ceiling self-test: {len(cases)} cases, {len(bad)} wrong {bad if bad else ''}")
        return 1 if bad else 0
    found = problems(blob, cat)
    for p in found:
        print(f"image-ceiling: {p}")
    print(f"image-ceiling: {len(found)} problems")
    return 1 if found else 0


if __name__ == "__main__":
    sys.exit(main())
