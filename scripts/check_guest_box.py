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
"""The test guests' box names only services nonos_linux_policy's build takes (B-03).

GuestPolicies.mk gave every guest FILES_USER, which the build refuses as
reserved (build/line.rs), so no NONOS_LINUX_GUESTS=1 image built. Every name
in LINUX_GUEST_ALL is one the build knows (src/mask.rs NAMES) and none is
one it refuses.
"""

import argparse
import re
import sys
from pathlib import Path

MK = "userland/linux_guests/GuestPolicies.mk"
NAMES = "userland/nonos_linux_policy/src/mask.rs"
LINE = "userland/nonos_linux_policy/build/line.rs"


def problems(mk: str, names: str, line: str) -> list:
    m = re.search(r"^LINUX_GUEST_ALL := (.*)$", mk, re.M)
    box = m.group(1).split() if m else []
    known = set(re.findall(r'\("([A-Z_]+)",', names))
    reserved = set(re.findall(r"if mask & ([A-Z_]+) != 0 \{\s*return Err\([^)]*reserved", line))
    out = [] if box else ["GuestPolicies.mk names no LINUX_GUEST_ALL"]
    out += [f"{n}: the policy build names no such service" for n in box if n not in known]
    out += [f"{n}: the policy build refuses it as reserved" for n in box if n in reserved]
    if not reserved:
        out.append("line.rs names no reserved service: the scan no longer reads it")
    return out


def main() -> int:
    ap = argparse.ArgumentParser(description=__doc__.split("\n")[0])
    ap.add_argument("--self-test", action="store_true")
    a = ap.parse_args()
    mk, names, line = (Path(p).read_text() for p in (MK, NAMES, LINE))
    if a.self_test:
        add = lambda w: mk.replace("LINUX_GUEST_ALL := ", f"LINUX_GUEST_ALL := {w} ", 1)
        cases = [("as built", (mk, names, line), 0), ("FILES_USER back", (add("FILES_USER"), names, line), 1),
                 ("an unknown name", (add("CAMERA"), names, line), 1),
                 ("refusal unread", (mk, names, line.replace("reserved", "kept back")), 1)]
        bad = [w for w, args, n in cases if len(problems(*args)) != n]
        print(f"guest-box self-test: {len(cases)} cases, {len(bad)} wrong {bad if bad else ''}")
        return 1 if bad else 0
    found = problems(mk, names, line)
    for p in found:
        print(f"guest-box: {p}")
    print(f"guest-box: {len(found)} problems")
    return 1 if found else 0


if __name__ == "__main__":
    sys.exit(main())
