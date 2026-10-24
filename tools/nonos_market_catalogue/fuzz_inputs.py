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
"""The inputs test_fuzz.py sweeps with: anything JSON could hold, and real
packages mangled by bit flips, cuts, splices and repeats."""

import gzip
import random
from pathlib import Path

from .apk_fixture import ELF, make

VALID = make([("usr/bin/a", ELF + b"a"), ("usr/share/a/x", b"data")])
VECTOR = (Path(__file__).resolve().parent / "vectors/nitrokey-udev-rules-1.0.0-r2.apk").read_bytes()
WORDS = ["MIT", "TTY", "NET_ANON", "x", "../x", "", "linux.a", "\x00", "1-r0", "FILES_USER", "ff" * 65]


def anything(r: random.Random, depth=0):
    pick = r.randrange(7 if depth < 2 else 4)
    if pick == 0:
        return r.choice(WORDS)
    if pick == 1:
        return "".join(chr(r.randrange(0x20, 0x250)) for _ in range(r.randrange(12)))
    if pick == 2:
        return r.choice([None, 0, -1, 1.5, True])
    if pick == 3:
        return bytes(r.randrange(256) for _ in range(r.randrange(8))).hex()
    if pick == 4:
        return [anything(r, depth + 1) for _ in range(r.randrange(3))]
    keys = ["package", "licence", "mask", "name", "text", "calls", "dests", "P", "V", "L", "base",
            "listing_id", "schema_version", "entries"]
    return {r.choice(keys): anything(r, depth + 1) for _ in range(r.randrange(7))}


def mangled(r: random.Random) -> bytes:
    raw = bytearray(r.choice([VALID, VECTOR, gzip.compress(r.randbytes(r.randrange(600)))]))
    for _ in range(r.randrange(1, 6)):
        op, at = r.randrange(4), r.randrange(len(raw) or 1)
        if op == 0 and raw:
            raw[at] ^= 1 << r.randrange(8)
        elif op == 1:
            del raw[at:]
        elif op == 2:
            raw[at:at] = r.randbytes(r.randrange(40))
        else:
            raw += raw[: r.randrange(len(raw) + 1)]
    return bytes(raw)
