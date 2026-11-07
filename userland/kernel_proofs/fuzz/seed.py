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

"""Seed syscall_gate with every call the ABI registry names. A syscall id is
a four-letter tag, which a fuzzer seldom guesses; each seed is one 32-byte
step per registered call (number, capability bits, address, length), once
with every capability and once with none, and one seed runs them all in turn."""

import argparse
import pathlib
import re
import struct

REGISTRY = pathlib.Path(__file__).resolve().parents[3] / "src/syscall/abi/registry"
TAG = re.compile(rb'b"([A-Za-z0-9_]{4})"')


def tags():
    found = set()
    for f in sorted(REGISTRY.glob("*.rs")):
        found.update(TAG.findall(f.read_bytes()))
    return sorted(found)


def step(tag, caps, addr, length):
    return struct.pack("<QQQQ", int.from_bytes(tag, "little"), caps, addr, length)


def main():
    ap = argparse.ArgumentParser(description=__doc__)
    ap.add_argument("--out", required=True, help="corpus/syscall_gate")
    out = pathlib.Path(ap.parse_args().out)
    out.mkdir(parents=True, exist_ok=True)
    every = tags()
    for tag in every:
        name = tag.decode()
        (out / f"all-{name}").write_bytes(step(tag, (1 << 64) - 1, 0x1000, 64))
        (out / f"none-{name}").write_bytes(step(tag, 0, 0x7fff_ffff_f000, 0x1001))
    (out / "sequence").write_bytes(b"".join(step(t, 0x19, 0x40_0000, 4096) for t in every))
    print(f"wrote {2 * len(every) + 1} seeds for {len(every)} calls to {out}")


if __name__ == "__main__":
    main()
