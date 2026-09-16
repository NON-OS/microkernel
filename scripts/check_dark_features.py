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
"""Find kernel features that gate code and that no build lane ever turns on.

A cfg site naming a feature Cargo.toml does not declare fails outright. A
declared feature with cfg sites and no lane is dark, and the dark set is held
against scripts/baselines/dark-features.txt: a feature may leave the list when
a lane starts enabling it, and may not join it. --write-baseline records the
current set; --self-test proves the check bites on a decoy. The lanes read
are the default and profile lists in Cargo.toml, every --features in the
makefiles and workflows including matrix lists, and the capsule selection.
Not read: build.rs, the bootloader, the capsules, the nonos-mk submodule; a
compiled feature is not a booted one.
"""

import argparse
import sys
from pathlib import Path

from dark_features_lanes import BASELINE  # noqa: E402
from dark_features_scan import analyse  # noqa: E402
from dark_features_selftest import self_test  # noqa: E402


def main():
    ap = argparse.ArgumentParser(description=__doc__)
    ap.add_argument("--root", type=Path, default=Path("."))
    ap.add_argument("--write-baseline", action="store_true")
    ap.add_argument("--self-test", action="store_true")
    args = ap.parse_args()
    if args.self_test:
        self_test()
        return 0
    dark, undeclared, sites = analyse(args.root)
    if undeclared:
        print("dark-features: cfg sites name features Cargo.toml does not declare, "
              "so that code can never compile in:", file=sys.stderr)
        for f in undeclared:
            print(f"  {f} ({sites[f]} sites)", file=sys.stderr)
        return 1
    if args.write_baseline:
        (args.root / BASELINE).write_text("".join(f"{f}\n" for f in dark))
        print(f"dark-features: baseline written with {len(dark)} features")
        return 0
    known = set((args.root / BASELINE).read_text().split()) if (args.root / BASELINE).exists() else set()
    new = [f for f in dark if f not in known]
    for f in dark:
        print(f"dark-features: {f} gates {sites[f]} cfg sites and no lane enables it")
    if new:
        print("dark-features: features that joined the dark set:", file=sys.stderr)
        for f in new:
            print(f"  {f}", file=sys.stderr)
        print("either wire a lane that builds it or remove the switch; the baseline "
              "may only shrink", file=sys.stderr)
        return 1
    print(f"dark-features: {len(dark)} known dark features, none new")
    return 0


if __name__ == "__main__":
    sys.exit(main())
