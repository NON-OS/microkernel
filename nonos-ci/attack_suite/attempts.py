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


"""The attempts one boot's log shows: each [ATTACK] line with the kernel's
line beside it, and the verdict on the attacker's fault, which only the
kernel's lines can give."""

import collections
import re

from .fault import fault_lines
from .kernel_lines import attacker_pid, kernel_line

LINE = re.compile(r"\[ATTACK\] ([a-z0-9-]+) (refused|ESCAPED): ?([^\n]*)")
NOTE = re.compile(r"\[ATTACK-NOTE\][^\n]*")
# An escape whatever its shape: no colon, or a name in another case. The
# word is the printers' own, in capitals; a guest's summary says "0 escaped"
# when it held and BROKEN when it did not (linux_guests/src/report.rs).
ESCAPE_WORD = re.compile(r"ESCAPED|\[GUEST\] \S+ BROKEN")
# An [ATTACK] line that is no verdict: split, cut or garbled on its way, so
# what it said cannot be read.
ATTACK_TAG = "[ATTACK] "
# The checker writes this attack's refusal from the kernel's lines; a log
# line that claims one is not evidence. A line claiming its escape still is.
CHECKER_ONLY = "fault-containment"

Attempt = collections.namedtuple("Attempt", "name verdict how kernel")


def attempts(log):
    lines = log.splitlines()
    pid = attacker_pid(lines)
    out = []
    for i, line in enumerate(lines):
        m = LINE.search(line)
        if m and not (m.group(1) == CHECKER_ONLY and m.group(2) == "refused"):
            name, verdict, how = m.groups()
            out.append(Attempt(name, verdict, how.strip(), kernel_line(lines, i, name, pid)))
    for line, trap in fault_lines(lines):
        name, verdict, how = LINE.search(line).groups()
        out.append(Attempt(name, verdict, how.strip(), trap))
    return out


def stray_escapes(log):
    """Lines that say ESCAPED but are no [ATTACK] verdict, and [ATTACK] lines
    that are no verdict at all."""
    return [l.strip() for l in log.splitlines()
            if (ESCAPE_WORD.search(l) or ATTACK_TAG in l) and not LINE.search(l)]


def notes(log):
    return NOTE.findall(log)
