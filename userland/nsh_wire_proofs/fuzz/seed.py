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
"""Seed nsh_wire's fuzz targets with one valid body of each kind (docs/design/nsh-v1*.md)."""

import argparse
import pathlib
from struct import pack

SIG, PUB = b"\xe5" * (64 + 3309), b"\x4b" * (32 + 1952)

def b8(x): return bytes([len(x)]) + x
def b16(x): return pack("<H", len(x)) + x
def b32(x): return pack("<I", len(x)) + x
def caps(names): return bytes([len(names)]) + b"".join(b8(n) for n in names)
def hello(tail): return b"NSH1" + pack("<HHH", 2, 1, 0) + b"\x52" * 32 + tail

def grant(kind=1, approvals=()):
    gid, parent = (b"\x11" * 16, b"\0" * 16) if kind == 1 else (b"\x22" * 16, b"\x11" * 16)
    body = b"NSHG" + pack("<HBB", 1, kind, 0) + gid + parent + b"\xf1" * 16 + b"\x5b" * 64
    body += b8(b"ek") + b8(b"auditor") + caps([b"CoreExec", b"IPC"])
    body += pack("<QQ", 1000, 2000) + b"\x15" * 32 + SIG
    return body + bytes([len(approvals)]) + b"".join(a + SIG for a in approvals)

def enrol():
    body = b"NSHE" + pack("<HH", 1, 0) + b"\xf1" * 16 + b8(b"db-1.eu.example")
    body += b"\xac" * 64 + b"\x4b" * 64 + b"".join(bytes([p]) * 32 for p in (0, 1, 2, 7))
    return body + b"\x01\x01" + b"\xb0" * 32 + pack("<Q", 9000) + b"\xaa" * 32 + SIG

def record():
    head = b"NSHR" + pack("<HH", 1, 0) + b"\x5e" * 16 + pack("<Q", 1000)
    head += b"\x7a" * 64 + PUB + b16(enrol()) + b"\xd0" * 64 + b"\x5b" * 64
    head += b8(b"ek") + b8(b"auditor") + b"\x6a" * 64 + caps([b"CoreExec"])
    ents = pack("<IQB", 0, 1100, 1) + b16(b"ls /var/log")
    box = b"\x00" + b"\x58" * 32 + b"\x01" + pack("<i9Q", 1, 1, 1, 0, 0, 0, 0, 0, 0, 0)
    ents += pack("<IQB", 1, 1150, 9) + b16(box) + pack("<IQB", 2, 1200, 7) + b16(pack("<i", 0))
    return head + ents + pack("<IQB", 3, 1300, 1) + b"\xc4" * 64 + SIG

SEEDS = {
    "nsh_frame": [b"\x01" + hello(b"\xee" * 1216), b"\x30" + b"ls\n", b"\x7f\x03\x00"],
    "nsh_hello": [hello(b"\xee" * 1216), hello(b"\xcc" * 1120)],
    "nsh_host_proof": [PUB + b16(enrol()) + b32(b"\xd0" * 300) + b32(b"\x07" * 90) + SIG],
    "nsh_user_auth": [PUB + b16(grant()) + SIG, b16(grant(2, [b"\xa1" * 32]))],
    "nsh_grant": [grant(), grant(2, [b"\xa1" * 32, b"\xa2" * 32])],
    "nsh_enrol": [enrol()],
    "nsh_record": [record()],
    "nsh_session": [b"\x00" + pack("<HH", 80, 24), b"\x02" + pack("<HQ", 0, 5), b"\x04\x01\x09",
                    b"\x01" + b"\x07" * 16 + pack("<I", 900) + caps([b"IPC"])],
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
