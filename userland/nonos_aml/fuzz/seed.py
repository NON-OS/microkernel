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


"""Seed the aml target. The machine's own firmware tables, when this host
exposes them (/sys/firmware/acpi/tables), are real DSDT and SSDT bytes, given
whole with the mode byte that loads them as they are. A few small hand-built
method bodies follow, wrapped in a good header, so the interpreter's paths
are reached from the first run."""

import argparse
import pathlib

FIRMWARE = pathlib.Path("/sys/firmware/acpi/tables")
RAW_WHOLE = bytes([(63 << 2) | 2])
WRAPPED_WHOLE = bytes([(63 << 2) | 0])


def pkg_len(n):
    """A PkgLength covering n bytes after it, in the one- or two-byte form."""
    if n + 1 < 0x40:
        return bytes([n + 1])
    n += 2
    return bytes([0x40 | (n & 0x0F), (n >> 4) & 0xFF])


def method(name, args, body):
    inner = name + bytes([args]) + body
    return b"\x14" + pkg_len(len(inner)) + inner


BODIES = [
    # Return (Add (Arg0, 0x10))
    method(b"MADD", 1, b"\xa4\x72\x68\x0a\x10\x00"),
    # While (Arg0) { Decrement (Arg0) } Return (Arg0)
    method(b"MLOP", 1, b"\xa2\x05\x68\x76\x68" + b"\xa4\x68"),
    # OperationRegion (REG0, SystemIO, 0x80, 4) and a field read
    b"\x5b\x80REG0\x01\x0a\x80\x0a\x04"
    + b"\x5b\x81\x0bREG0\x01FLD0\x20"
    + method(b"MRD_", 0, b"\xa4FLD0"),
]


def main():
    ap = argparse.ArgumentParser(description=__doc__)
    ap.add_argument("--out", required=True, help="corpus/aml")
    out = pathlib.Path(ap.parse_args().out)
    out.mkdir(parents=True, exist_ok=True)
    n = 0
    if FIRMWARE.is_dir():
        for f in sorted(FIRMWARE.iterdir()):
            if f.name.startswith(("DSDT", "SSDT")) and f.is_file():
                try:
                    (out / f"firmware-{f.name}").write_bytes(RAW_WHOLE + f.read_bytes())
                    n += 1
                except OSError:
                    continue
    for i, body in enumerate(BODIES):
        (out / f"body-{i}").write_bytes(WRAPPED_WHOLE + body)
        n += 1
    print(f"wrote {n} seeds to {out}")


if __name__ == "__main__":
    main()
