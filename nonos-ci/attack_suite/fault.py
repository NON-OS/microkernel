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

"""The attacker's fault, which the attacker cannot report: its line is
judged here from the kernel's trap line and what the log did after it."""

import re

FAULT = re.compile(r"\[ATTACK-FAULT\] pid (\d+) writes address (0x[0-9a-f]+)")
# At the start of its line, as the kernel's trap dump prints it. A capsule
# holding Debug writes raw bytes and could still print one there
# (docs/security/self-test.md says so); a trap quoted inside another line is
# not one.
TRAP = re.compile(r"^\[TRAP PF\] cpl=3 .*?pid=(0x[0-9a-f]+).*?cr2=(0x[0-9a-f]+)")
# The kernel's own death: a Rust panic, or a fault with no recovery path
# (arch/x86_64/diag/fatal_notice.rs). A kernel-mode trap line alone is not
# one: the handler resolves demand faults on user copies.
PANIC = re.compile(r"KERNEL PANIC|\[PANIC [A-Z]+\] fatal kernel fault")


def fault_lines(lines):
    """The attack lines the attacker's own fault earns, each with the
    kernel line it rests on: the kernel's user-mode trap for that pid at that
    address, then the log going on with no kernel fault."""
    out = []
    for i, line in enumerate(lines):
        m = FAULT.search(line)
        if not m:
            continue
        pid, addr = int(m.group(1)), int(m.group(2), 16)
        rest = lines[i + 1:]
        trap = next((j for j, l in enumerate(rest) if (t := TRAP.match(l))
                     and int(t.group(1), 16) == pid and int(t.group(2), 16) == addr), None)
        if trap is None:
            out.append((f"[ATTACK] fault-containment ESCAPED: no user fault for pid {pid} at {hex(addr)}",
                        None))
            continue
        after = [l for l in rest[trap + 1:] if l.strip() and not l.startswith("[TRAP")]
        if any(PANIC.search(l) for l in rest):
            line = "[ATTACK] fault-containment ESCAPED: the kernel faulted after the attacker did"
        elif not after:
            line = "[ATTACK] fault-containment ESCAPED: the log ended at the attacker's fault"
        else:
            line = (f"[ATTACK] fault-containment refused: the kernel ended pid {pid} at its fault "
                    f"on {hex(addr)}, and {len(after)} lines followed")
        out.append((line, rest[trap].strip()))
    return out
