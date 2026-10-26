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

"""The kernel's own line beside each attempt: what the kernel said when it
refused, so a report shows the refusal from both sides.

The attack capsule names its pid first ([ATTACK-START]); each of its lines is
paired with the kernel's last [CAP-DENY] for that pid since the attempt
before it. The attestation-refusal lines are the kernel's own, printed by
its spawn gate. A Linux guest's line is paired with the last kernel or
personality line before it. An attempt the kernel refused with an errno
alone, logging nothing, says so rather than borrowing another line."""

import re

ATTEMPT = re.compile(r"\[ATTACK(?:-[A-Z]+)?\]")
START = re.compile(r"\[ATTACK-START\] [a-z-]+ pid (\d+)")
CAP_DENY = re.compile(r"\[CAP-DENY\] pid=(\d+)")
# The tags the kernel and the Linux personality print on the serial line.
KERNEL = re.compile(r"\[(?:CAP-DENY|LINUX[A-Z-]*|PERSONALITY|IOMMU|SPAWN[A-Z-]*|ATTEST[A-Z-]*|TRAP [A-Z]+)\]")
# Attacks whose line the kernel prints itself.
KERNEL_PRINTED = ("trailer-",)
CAPSULE = ("cap-escape", "foreign-memory", "service-squat", "held-escape", "deputy-escape", "reply-twice")


def kernel_line(lines, i, name, attacker):
    """The kernel's line for the attempt reported at lines[i], or None."""
    if name.startswith(KERNEL_PRINTED):
        return "(printed by the kernel's spawn gate)"
    j = i - 1
    while j >= 0 and not ATTEMPT.search(lines[j]):
        j -= 1
    window = lines[j + 1:i]
    if name in CAPSULE:
        hits = [l for l in window if (m := CAP_DENY.search(l))
                and (attacker is None or int(m.group(1)) == attacker)]
    else:
        hits = [l for l in window if KERNEL.search(l)]
    return hits[-1].strip() if hits else None


def attacker_pid(lines):
    """The pid the attack capsule says it runs as, or None."""
    for l in lines:
        if m := START.search(l):
            return int(m.group(1))
    return None
