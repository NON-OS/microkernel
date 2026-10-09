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
"""The scan behind check_unenforced.py: controls that exist and do not run.

Three shapes, each one a past defect here: a gate-shaped function nothing
calls, a policy value in a security tree nothing reads, and a check that
logs what it would refuse and then admits.
"""

import re

from unreachable_scan import COMMENT, DEFINITION, IMPORT, WORD, sources, unreachable

GATE = re.compile(
    r"^(check|verify|validate|require|ensure|enforce|authori[sz]e|permit|allow|deny|refuse"
    r"|reject|guard|gate|admit|is_allowed|may_|can_)"
)
POLICY_TREES = ("src/security", "src/capabilities", "src/syscall/contract", "src/drivers/pci/security")
POLICY = re.compile(r"^\s*pub(?:\([a-z]+\))?\s+(?:const|static)\s+(?:mut\s+)?([A-Z][A-Z0-9_]*)\s*:")
LOGS = re.compile(
    r"not enforced|would (have )?(refus|reject|den)|enforce_[a-z_]+\s*:\s*false"
    r"|permissive mode|(log|audit)[-_ ]only",
    re.I,
)


def uncalled_gates(root):
    return [s for s in unreachable(root) if GATE.match(s.split(" ", 1)[1])]


def unread_policy(root):
    defs, seen = [], set()
    for rel, lines in sources(root):
        policy = str(rel).startswith(POLICY_TREES)
        for n, line in enumerate(lines, 1):
            m = POLICY.match(line)
            if m and policy:
                defs.append((m.group(1), f"{rel}:{n}"))
            if m or IMPORT.match(line) or COMMENT.match(line) or DEFINITION.match(line):
                # A definition names itself; its initialiser may still read others.
                seen.update(WORD.findall(line.split("=", 1)[1]) if m and "=" in line else [])
                continue
            seen.update(re.findall(r"\b[A-Z][A-Z0-9_]*\b", line))
    return [f"{site} {name}" for name, site in defs if name not in seen]


def logs_not_refuses(root):
    out = []
    for rel, lines in sources(root):
        for n, line in enumerate(lines, 1):
            if not COMMENT.match(line) and LOGS.search(line):
                out.append(f"{rel}:{n} logs")
    return out


def unenforced(root):
    return sorted(uncalled_gates(root) + unread_policy(root) + logs_not_refuses(root))
