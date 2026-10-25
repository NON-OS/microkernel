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
"""Every display driver the compositor tries ships in the full desktop image.
Each on src/services/registry/displays.rs with a capsule in capsules.json is
in microkernel-full-gui (what `make` builds) or in the may-only-shrink list
scripts/baselines/display_profiles.txt; else its display runs on firmware's.
"""

import json
import re
import sys
from pathlib import Path
import gate

BASELINE = Path("scripts/baselines/display_profiles.txt")
IMAGE = "microkernel-full-gui"


def features(cargo_toml):
    """Each Cargo feature and the names it enables."""
    found = re.finditer(r'^([a-z0-9-]+) = \[(.*?)\]', cargo_toml, re.S | re.M)
    return {m.group(1): re.findall(r'"([^"]+)"', m.group(2)) for m in found}


def enabled(table, name):
    """Every feature `name` turns on, itself included."""
    on, todo = set(), [name]
    while todo:
        n = todo.pop()
        if n not in on:
            on.add(n)
            todo.extend(table.get(n, []))
    return on


def sites(root, cargo=None):
    on = enabled(features(cargo or (root / "Cargo.toml").read_text()), IMAGE)
    src = (root / "src/services/registry/displays.rs").read_text()
    names = re.findall(r'^\s*"(driver\.[a-z0-9_]+)",', src.split("DISPLAYS")[1].split("];")[0], re.M)
    capsules = json.loads((root / "tools/nix/capsules.json").read_text())
    feature = {c.get("handle"): c.get("feature") for c in capsules if isinstance(c, dict)}
    return [f"{n} {feature[n]}" for n in names if n in feature and feature[n] not in on]


def self_test(root):
    """The scan reports a display driver taken out of the image."""
    cargo = (root / "Cargo.toml").read_text().replace('  "nonos-capsule-driver-virtio-gpu",\n', "")
    return any(s.startswith("driver.virtio_gpu0 ") for s in sites(root, cargo))


def main():
    args = gate.parser(__doc__).parse_args()
    if args.self_test:
        ok = self_test(args.root)
        print("display-profiles self-test:", "bites" if ok else "MISSED a driver left out")
        return int(not ok)
    return gate.run("display-profiles", "not in " + IMAGE + ":", sites, BASELINE, args)


if __name__ == "__main__":
    sys.exit(main())
