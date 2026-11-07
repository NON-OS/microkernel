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
"""One boot's serial log, read for the kernel's boot census and for the
order the ACPI capsule brought the platform up in.

The census lines are the kernel's (src/hardware/boot_census/rule.rs):

    [CENSUS] <class> <up|absent|missing|held> expected=<n> up=<n>[ why=<reason>]
    [CENSUS] summary up=<n> absent=<n> held=<n> missing=<n> verdict=<green|red>

A boot may say its census more than once (a row changed after it was first
taken); the last one said is the one that counts.
"""

import re
from dataclasses import dataclass, field

ROW = re.compile(r"\[CENSUS\] ([a-z]+) (up|absent|missing|held) expected=(\d+) up=(\d+)(?: why=(.*))?$")
SUMMARY = re.compile(
    r"\[CENSUS\] summary up=(\d+) absent=(\d+) held=(\d+) missing=(\d+) verdict=(green|red)$")
FATAL = ("[FATAL]", "[PANIC]", "[TRAP GP]", "[TRAP UD]", "[INIT] capsule spawn incomplete")
EC_OFF = "[ACPI] EC turned off"
ACPI_MODE = ("[ACPI] entered ACPI mode", "[ACPI] already in ACPI mode")
REG = "[ACPI] _REG failed in "
# A terminal's colour codes, which some serial captures keep.
ESCAPE = re.compile(r"\x1b\[[0-9;]*[A-Za-z]")


@dataclass
class Boot:
    name: str
    rows: dict = field(default_factory=dict)
    summary: tuple = None
    fatal: list = field(default_factory=list)
    ec_off: int = 0
    mode_line: int = None
    first_reg_line: int = None
    battery_ms: int = None


def read(name, text):
    """The last census, the faults and the bring-up order of one log."""
    b, pending = Boot(name), {}
    for n, raw in enumerate(text.splitlines()):
        line = ESCAPE.sub("", raw.rstrip("\r"))
        at = line.find("[")
        line = line[at:] if at > 0 else line
        m = ROW.match(line)
        if m:
            cls, state, exp, up, why = m.groups()
            pending[cls] = (state, int(exp), int(up), why or "")
            continue
        m = SUMMARY.match(line)
        if m:
            up, absent, held, missing, verdict = m.groups()
            b.rows, pending = pending, {}
            b.summary = (int(up), int(absent), int(held), int(missing), verdict)
            continue
        if line.startswith(FATAL):
            b.fatal.append(line)
        if line.startswith(EC_OFF):
            b.ec_off += 1
        if line.startswith(ACPI_MODE) and b.mode_line is None:
            b.mode_line = n
        if line.startswith(REG) and b.first_reg_line is None:
            b.first_reg_line = n
        m = re.match(r"\[BRINGUP\] t=(\d+) dev=BATTERY ev=published", line)
        if m and b.battery_ms is None:
            b.battery_ms = int(m.group(1))
    return b
