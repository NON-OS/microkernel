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
#
# You should have received a copy of the GNU Affero General Public License
# along with this program. If not, see <https://www.gnu.org/licenses/>.

"""Write the seed corpus for the v4_parse target: well-formed v4 trailers of
every kind, so the fuzzer starts from bytes the parser accepts instead of
having to find both magics on its own."""

import argparse
import pathlib
import struct

# (selector byte the target maps to a kind, the kind byte on the wire)
KINDS = [(0, 0), (1, 1), (2, 3)]


def v3_path(depth):
    dirs = (depth + 7) // 8
    return b"NZKPATH1" + bytes([depth]) + bytes(32 * depth) + bytes(dirs)


def v4(kind, path, proof):
    return (b"NATTV4\0\0" + bytes([kind]) + struct.pack("<I", len(path)) + path
            + struct.pack("<I", len(proof)) + proof)


def main():
    ap = argparse.ArgumentParser(description=__doc__)
    ap.add_argument("--out", default="corpus/v4_parse", help="corpus directory")
    args = ap.parse_args()
    out = pathlib.Path(args.out)
    out.mkdir(parents=True, exist_ok=True)
    n = 0
    for sel, kind in KINDS:
        for depth in (1, 8, 32):
            for bound, proof in ((1, b"\x01" * 64), (3, b"\x02" * 150)):
                seed = bytes([sel, bound]) + v4(kind, v3_path(depth), proof)
                (out / f"seed-{sel}-{depth}-{bound}").write_bytes(seed)
                n += 1
    print(f"wrote {n} seeds to {out}")


if __name__ == "__main__":
    main()
