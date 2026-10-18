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
"""foot's Wayland globals, held to what the server offers: every global
foot looks for (userland/capsule_linux/abi/wayland-wanted.txt) is advertised
by the registry (wayland/registry.rs), except the ones held back on purpose,
each with its reason. A held one that becomes offered must leave HELD, so the
list stays true. Needs nothing built."""

import re
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
REGISTRY = ROOT / "userland/capsule_linux/src/linux/wayland/registry.rs"
WANTED = ROOT / "userland/capsule_linux/abi/wayland-wanted.txt"
HELD = {
    # Its feedback reports when a frame was shown, which the server does not know.
    "wp_presentation",
    # An input method this system does not have; held until foot's run shows a need.
    "zwp_text_input_manager_v3",
    # Would repeat wl_output's scale 1; held until foot's run shows a need.
    "wp_fractional_scale_manager_v1",
}


def main() -> int:
    offered = set(re.findall(r'interface: b"(\w+)"', REGISTRY.read_text()))
    wanted = {l.split()[1] for l in WANTED.read_text().splitlines()
              if l.startswith("foot ")}
    missing = wanted - offered - HELD
    stale = HELD & offered
    for name in sorted(missing):
        print(f"wayland: foot looks for {name}, which is neither offered nor held", file=sys.stderr)
    for name in sorted(stale):
        print(f"wayland: {name} is offered now; take it out of HELD", file=sys.stderr)
    if missing or stale or not wanted:
        return 1
    print(f"wayland: foot's {len(wanted)} globals: {len(wanted & offered)} offered, {len(HELD)} held")
    return 0


if __name__ == "__main__":
    sys.exit(main())
