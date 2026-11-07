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
"""Seed nsh_server's fuzz target with one input of each shape (docs/nsh/nsh-v1*.md)."""

import argparse
import pathlib
from struct import pack

SIG, PUB = b"\xe5" * (64 + 3309), b"\x4b" * (32 + 1952)

def b8(x): return bytes([len(x)]) + x
def b16(x): return pack("<H", len(x)) + x
def caps(names): return bytes([len(names)]) + b"".join(b8(n) for n in names)

def hello():
    body = b"NSH1" + pack("<HHH", 2, 1, 0) + b"\x11" * 32 + b"\x22" * 1216
    return pack("<I", 1 + len(body)) + b"\x01" + body

def grant(kind):
    body = b"NSHG" + pack("<HH", 1, 0) + bytes([kind]) + b"\x11" * 16 + b"\x00" * 16
    body += b"\xf1" * 16 + b"\x5a" * 64 + b8(b"ek") + b8(b"operator")
    body += caps([b"CoreExec", b"IPC"]) + pack("<QQ", 1, 2) + b"\xaa" * 32 + SIG + b"\x00"
    return body

def seeds():
    yield "hello", b"\x00" + hello()
    yield "hello-version", b"\x00" + hello()[:9] + b"\x09" + hello()[10:]
    yield "user-auth", b"\x01" + b16(b"\x20" + PUB + b16(grant(1)) + SIG)
    yield "abort", b"\x01" + b16(b"\x7f\x03\x00")
    yield "input", b"\x02" + b16(b"\x30hello\n")
    yield "window", b"\x02" + b16(b"\x32" + pack("<HH", 80, 24))
    yield "signal", b"\x02" + b16(b"\x33\x01")
    yield "escalate", b"\x02" + b16(b"\x40" + b16(grant(2)))
    yield "close", b"\x02" + b16(b"\x52") + b16(b"\x30late")

def main():
    ap = argparse.ArgumentParser(description=__doc__)
    ap.add_argument("--out", required=True, help="the corpus directory to fill")
    out = pathlib.Path(ap.parse_args().out) / "nsh_server_feed"
    out.mkdir(parents=True, exist_ok=True)
    for name, data in seeds():
        (out / name).write_bytes(data)

if __name__ == "__main__":
    main()
