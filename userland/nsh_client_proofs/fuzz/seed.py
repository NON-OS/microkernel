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
"""Seed nsh_client's fuzz targets with one input of each shape (docs/design/nsh-v1*.md)."""

import argparse
import pathlib
from struct import pack

SIG, PUB = b"\xe5" * (64 + 3309), b"\x4b" * (32 + 1952)

def b8(x): return bytes([len(x)]) + x
def b16(x): return pack("<H", len(x)) + x
def b32(x): return pack("<I", len(x)) + x
def caps(names): return bytes([len(names)]) + b"".join(b8(n) for n in names)
def sealed(inner): return b16(inner)

def enrol():
    body = b"NSHE" + pack("<HH", 1, 0) + b"\xf1" * 16 + b8(b"db-1.eu.example")
    body += b"\xac" * 64 + b"\x4b" * 64 + b"".join(bytes([p]) * 32 for p in (0, 1, 2, 7))
    return body + b"\x01\x01" + b"\xb0" * 32 + pack("<Q", 9000000) + b"\xaa" * 32 + SIG

def host_proof():
    doc = b"TDOC" + b"\x00" * 32 + b"\xac" * 64 + b"".join(bytes([p]) * 32 for p in (0, 1, 2, 7))
    return b"\x10" + PUB + b16(enrol()) + b32(doc) + b32(b"\x07" * 90) + SIG

def server_hello():
    body = b"NSH1" + pack("<HHH", 1, 1, 0) + b"\x53" * 32 + b"\xcc" * 1120
    return pack("<I", 1 + len(body)) + b"\x02" + body

AUTH_OK = b"\x21" + b"\x5e" * 16 + pack("<I", 600) + caps([b"CoreExec", b"IPC"])
SESSION = [b"\x31ls\n", b"\x50" + pack("<i", 0), b"\x51\x01NSHR", b"\x52",
           b"\x41" + pack("<HQ", 33, 0)]

SEEDS = {
    "nsh_client_feed": [
        b"\x00" + server_hello(), b"\x00" + pack("<I", 3) + b"\x7f\x02\x00",
        b"\x01" + sealed(host_proof()), b"\x01" + sealed(b"\x7f\x03\x00"),
        b"\x02" + sealed(AUTH_OK), b"\x02" + sealed(b"\x22" + pack("<H", 24)),
        b"\x03" + b"".join(sealed(f) for f in SESSION),
        b"\x04" + pack("<I", 40) + b"\x00" * 40,
        b"\x05", b"\x05" + b"\x00" * 16, b"\x05\x90",
        b"\x05" + bytes([0, 0, 1, 0, 2, 3, 0, 0, 0, 0, 1, 2, 3]),
    ],
    "nsh_record_verify": [
        b"\x01", b"\x03", b"\x07", b"\x05" + pack("<IB", 3000, 1),
        b"\x00" + b"NSHR" + pack("<HH", 1, 0) + b"\x5e" * 16 + pack("<Q", 1000) + b"\x7a" * 64,
    ],
}

def main():
    ap = argparse.ArgumentParser(description=__doc__)
    ap.add_argument("--out", default="corpus", help="corpus root directory")
    root = pathlib.Path(ap.parse_args().out)
    for target, seeds in SEEDS.items():
        (root / target).mkdir(parents=True, exist_ok=True)
        for i, seed in enumerate(seeds):
            (root / target / f"seed{i}").write_bytes(seed)
    print(f"wrote {sum(map(len, SEEDS.values()))} seeds under {root}")

if __name__ == "__main__":
    main()
