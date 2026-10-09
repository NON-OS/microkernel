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
"""Make large.gz: two gzip members that inflate to 6 MiB together, past the
inflater's 4 MiB default and inside the installer's bound, the shape of an
index larger than the default allowed (Alpine community, Kali Packages)."""

import argparse
import gzip
from pathlib import Path


def main() -> int:
    ap = argparse.ArgumentParser(description=__doc__)
    ap.add_argument("--out", type=Path, default=Path(__file__).with_name("large.gz"))
    args = ap.parse_args()
    line = b"Package: nonos-test\nVersion: 1\nFilename: pool/x.deb\n\n"
    half = (line * (3 * 1024 * 1024 // len(line) + 1))[: 3 * 1024 * 1024]
    one = gzip.compress(half, mtime=0)
    args.out.write_bytes(one + gzip.compress(half, mtime=0))
    print(f"{args.out}: {args.out.stat().st_size} bytes, {2 * len(half)} inflated")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
