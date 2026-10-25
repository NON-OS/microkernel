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
"""Seed nsh_attest's fuzz targets with well-formed bodies (docs/design/k1-attest-doc-v4.md)."""

import argparse
import hashlib
import pathlib
from struct import pack

V4_PCRS = [0, 1, 2, 4, 7, 8, 9, 14]

def sha(x): return hashlib.sha256(x).digest()
def tpm2b(x): return pack(">H", len(x)) + x
def b32(x): return pack(">I", len(x)) + x

def spec_id(banks=((4, 20), (11, 32))):
    body = b"Spec ID Event03\0" + pack("<I", 0) + bytes([0, 2, 0, 2]) + pack("<I", len(banks))
    body += b"".join(pack("<HH", a, s) for a, s in banks) + b"\0"
    return pack("<II", 0, 3) + b"\0" * 20 + pack("<I", len(body)) + body

def event(pcr, kind, d, body=b"event data"):
    digests = pack("<H", 4) + b"\x5a" * 20 + pack("<H", 11) + d
    return pack("<III", pcr, kind, 2) + digests + pack("<I", len(body)) + body

def log():
    out = spec_id()
    out += event(0, 3, b"\0" * 32, b"StartupLocality\0\x03")
    for pcr in range(8):
        out += event(pcr, 4, sha(b"separator"))
    out += event(4, 0x80000003, sha(b"boot manager")) + event(4, 0x80000003, sha(b"loader"))
    return out

def attest(extra=b"\x42" * 32):
    out = pack(">IH", 0xFF544347, 0x8018) + tpm2b(b"\0\x0b" + b"\x3c" * 32) + tpm2b(extra)
    out += pack(">QIIB", 0x0102030405060708, 7, 2, 1) + pack(">Q", 0x20240101)
    return out + pack(">IHB", 1, 11, 3) + bytes([0x97, 0x43, 0]) + tpm2b(sha(b"pcrs"))

def doc(version=5):
    out = b"NONOSATT" + pack(">I", version) + b"\x11" * 32 + b"\x22" * 32 + pack(">I", 3)
    out += bytes([1, 1, 1]) + pack(">I", 0) + (pack(">I", 0) if version >= 4 else b"")
    if version == 5:
        out += bytes([8]) + b"".join(bytes([i]) + bytes([i]) * 32 for i in V4_PCRS)
        out += b32(log())
    sig = pack(">HH", 0x18, 0x0B) + tpm2b(b"\x33" * 32) + tpm2b(b"\x44" * 32)
    return out + b32(attest()) + b32(sig) + b32(b"\x55" * 64)

SEEDS = {
    "attest_doc": [doc(), doc(4), doc(3)],
    "attest_tcg_log": [log(), spec_id(), spec_id(((11, 32),)) + event(4, 1, sha(b"x"))[:-1]],
    "attest_quote": [attest(), attest(b"")],
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
