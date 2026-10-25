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
"""nonos-cap-audit's two checks: capabilities consulted, syscalls gated."""

import re
import sys
from pathlib import Path

from nonos_cap_audit_consulted import consulted
from nonos_cap_audit_read import declared, read


# Capabilities known to enforce nothing, recorded so the check is a
# ratchet rather than noise: these do not fail the audit, and any other
# will. `IO` is read by `can_read` and `can_write`, `Hardware` by
# `can_hardware`, and nothing calls any of them; neither is requested by a
# capsule. `Entropy` is only defined as `CAP_ENTROPY`, which nothing reads,
# since its clients went in K9; manifests still grant it, and dropping the
# bit changes signed manifests, the owner's call. Wire one or retire it and
# take it off this list; do not add to it to make the audit quiet.
UNENFORCED = {"IO", "Hardware", "Entropy"}


def capabilities(root: Path) -> int:
    """Every declared capability must be consulted by something."""
    listed = declared(root)
    used = consulted(root, listed)

    bad = 0
    for name in sorted(listed - used - UNENFORCED):
        print(f"declared but nothing consults it: {name}", file=sys.stderr)
        bad += 1
    for name in sorted(UNENFORCED - (listed - used)):
        print(f"on the unenforced list but now enforced, remove it: {name}", file=sys.stderr)
        bad += 1
    print(f"capabilities: {len(listed)} declared, {len(used)} consulted, "
          f"{len(UNENFORCED)} known unenforced")
    return bad


def syscalls(root: Path) -> int:
    """Every Mk number must be gated, named, routed and dispatched."""
    defs = {
        n for n, _ in re.findall(
            r"(\w+) = tag4\(b\"(\w{4})\"\)", read(root, "src/syscall/numbers/defs.rs")
        )
    }
    mk = {d for d in defs if d.startswith("Mk")}
    reg = set(re.findall(r"SyscallNumber::(\w+),", read(root, "src/syscall/abi/registry/mk.rs")))
    table = "".join(p.read_text() for p in sorted((root / "src/syscall/contract/cap_table").glob("*.rs")))
    gated = set(re.findall(r"SyscallNumber::(Mk\w+)", table))

    bad = 0
    for name in sorted(mk - reg):
        print(f"syscall not in the ABI registry: {name}", file=sys.stderr)
        bad += 1
    for name in sorted(mk - gated):
        print(f"syscall with no capability gate: {name}", file=sys.stderr)
        bad += 1
    print(f"syscalls: {len(mk)} Mk numbers, all gated and named" if not bad else "")
    return bad
