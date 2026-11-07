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


"""Seed the fdt target with the real device trees the proofs already read:
QEMU virt with GICv2, GICv3 and an SMMUv3, and the Raspberry Pi 4."""

import argparse
import pathlib

FIXTURES = pathlib.Path(__file__).resolve().parent.parent / "fixtures"


def main():
    ap = argparse.ArgumentParser(description=__doc__)
    ap.add_argument("--out", required=True, help="corpus/fdt")
    out = pathlib.Path(ap.parse_args().out)
    out.mkdir(parents=True, exist_ok=True)
    n = 0
    for f in sorted(FIXTURES.glob("*.dtb")):
        (out / f.name).write_bytes(f.read_bytes())
        n += 1
    print(f"wrote {n} seeds to {out}")


if __name__ == "__main__":
    main()
