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
"""nonos_system_map's printing: paced lines, rules, headings, numbers, names."""

import sys
import time

import system_map_consts as pace
from system_map_consts import BOLD, CAPABILITIES, CYAN, DIM, OFF

def out(text="", delay=None):
    step = pace.DELAY if delay is None else delay
    if step <= 0:
        sys.stdout.write(text + "\n")
        sys.stdout.flush()
        return
    for char in text:
        sys.stdout.write(char)
        sys.stdout.flush()
        time.sleep(step)
    sys.stdout.write("\n")
    sys.stdout.flush()


def rule():
    out(DIM + "-" * 74 + OFF, delay=0)


def heading(title):
    time.sleep(pace.PAUSE)
    rule()
    out(BOLD + CYAN + "  " + title + OFF)
    rule()


def commas(n):
    return "{:,}".format(n)


def mib(n):
    return "{:.2f} MiB".format(n / 1048576.0)


def decode(mask):
    """Capability names for a mask, with any unknown bits kept visible."""
    names = []
    seen = 0
    for bit, name in CAPABILITIES:
        if mask & bit:
            names.append(name)
            seen |= bit
    leftover = mask & ~seen
    if leftover:
        names.append("bit:0x{:x}".format(leftover))
    return names
