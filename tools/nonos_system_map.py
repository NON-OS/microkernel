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
"""What authority every capsule in this image actually holds.

Reads the Capsule.mk that declared each capsule and the signed artifacts the
build produced from it, so the answer is the one the spawn gate enforces
rather than a description of it. Nothing here is asserted: every number is
counted from a file on disk.

    python3 tools/nonos_system_map.py           the whole map
    python3 tools/nonos_system_map.py --fast    no typing delay
    python3 tools/nonos_system_map.py --tsv     one capsule per line
    python3 tools/nonos_system_map.py --rare    only the scarce authority
"""

import sys
import time

import system_map_consts as pace
from system_map_capsule import load
from system_map_consts import BOLD, OFF, REPO
from system_map_out import out, rule
from system_map_report import report_roots, report_table
from system_map_report2 import report_distribution, report_scarce, report_tsv


def main():
    args = sys.argv[1:]
    if "--fast" in args or "--tsv" in args:
        pace.DELAY = 0.0
        pace.PAUSE = 0.0

    capsules = load()
    if not capsules:
        print("no Capsule.mk found under {}/userland".format(REPO))
        return 1

    if "--tsv" in args:
        report_tsv(capsules)
        return 0

    out()
    out(BOLD + "  NONOS system map" + OFF)
    if "--rare" in args:
        report_scarce(capsules)
    else:
        report_roots(capsules)
        report_table(capsules)
        report_distribution(capsules)
        report_scarce(capsules)
    time.sleep(pace.PAUSE)
    rule()
    return 0


if __name__ == "__main__":
    sys.exit(main())
