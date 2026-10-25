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
"""The keymap a Wayland guest is given, held to its pinned sha256. Needs no
xkbcomp: regenerating is tools/nonos-linux-keymap, a deliberate step, after
which the new sha256 is pinned here."""

import hashlib
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
KEYMAP = ROOT / "userland/linux_userland/keymap-us.xkb"
NOTICE = ROOT / "userland/linux_userland/keymap-us.NOTICE"
PINNED = "628e2826eeaf192e1d0fb32cc1356f8a1166db4f7b5c8b1509f0cefa10ae14da"


def main() -> int:
    if not NOTICE.is_file():
        print("keymap: keymap-us.NOTICE, its licence, is missing", file=sys.stderr)
        return 1
    got = hashlib.sha256(KEYMAP.read_bytes()).hexdigest()
    if got != PINNED:
        print(f"keymap: keymap-us.xkb is {got}, pinned {PINNED}", file=sys.stderr)
        return 1
    print(f"keymap: keymap-us.xkb is the pinned {PINNED[:16]}")
    return 0


if __name__ == "__main__":
    sys.exit(main())
