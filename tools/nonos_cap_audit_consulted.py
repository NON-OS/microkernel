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
"""nonos-cap-audit: which declared capabilities something consults."""

import re
from pathlib import Path

from nonos_cap_audit_alias import alias, used_elsewhere
from nonos_cap_audit_read import called_anywhere, code_lines, read


ANY = r"Capability::(\w+)"
# Under src/userspace and in a spawn.rs, a `.bit()` builds the mask a capsule
# is given or asks for, and asks no holder: the attest refusal probe's
# `with_io` once counted IO as consulted. A gate there names no `.bit()`.
GRANT = r"Capability::(\w+)\b(?!\.bit\(\))"


def consulted(root: Path, listed: set) -> set:
    """Which of `listed` some reachable code actually asks about.

    A capability named only inside a checker that nothing calls is not
    consulted, however it looks to a grep. That distinction is the
    point: `Capability::FileSystem` is written into manifests and read
    by `can_read`, `can_write` and six more in `syscall/caps/checks`,
    and at the time of writing not one of those is called from
    anywhere. A manifest granting it says something the kernel does not
    enforce.

    Filtered against `listed` because two other enums in this kernel
    are also called `Capability`, one for zkIDs and one for the policy
    engine. A regex cannot tell them apart and the compiler does not
    have to, so anything not on the list is somebody else's type.
    """
    seen, by_checker, aliases = set(), {}, {}
    for path in (root / "src").rglob("*.rs"):
        if "capabilities/types/" in str(path):
            continue
        inside = "syscall/caps/checks" in str(path)
        grants = "src/userspace/" in str(path) or path.name == "spawn.rs"
        fn = None
        for line in code_lines(path):
            found = re.search(r"\bfn\s+(\w+)", line)
            if found:
                fn = found.group(1)
            if defined := alias(line):
                aliases[defined[0]] = defined[1]
                continue
            for name in re.findall(GRANT if grants else ANY, line):
                if inside and fn:
                    by_checker.setdefault(name, set()).add(fn)
                else:
                    seen.add(name)
    seen |= {cap for const, cap in aliases.items() if used_elsewhere(root, const)}
    for name, fns in by_checker.items():
        if any(called_anywhere(root, fn) for fn in fns):
            seen.add(name)
    return seen & listed
