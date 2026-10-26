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
"""nonos-mk-esp says what kernel it packs, and warns when it starts nothing.

A bare `make nonos-mk-esp` packed the default microkernel-core, which embeds
no capsule, so no spawn succeeded (B-02). Every kernel build records its
profile in $(KERNEL_PROFILE_RECORD); nonos-mk-esp names it and warns on it.
"""

import argparse
import re
import sys
from pathlib import Path

WRITE = "> $(KERNEL_PROFILE_RECORD)"
WARN = "WARNING: this kernel starts no capsules; for a desktop: make nonos-mk-desktop-gui-prod nonos-mk-esp"


def block(text: str, start: str) -> str:
    i = text.find(start)
    if i < 0:
        return ""
    m = re.search(r"\n(?:\s*\n|endef\b)", text[i:])
    return text[i : i + m.start()] if m else text[i:]


def problems(text: str) -> list:
    out = []
    if WRITE not in block(text, "define nonos_kernel_build"):
        out.append("nonos_kernel_build does not record the profile it built")
    default = block(text, "nonos-mk-kernel-default: |")
    if not re.search(r"'microkernel-core' " + re.escape(WRITE), default):
        out.append("the default kernel rule does not record microkernel-core")
    esp = text[text.find("\nnonos-mk-esp:") :]
    esp = esp[: esp.find('@echo "Packaging EFI System Partition..."')]
    if "$(KERNEL_PROFILE_RECORD)" not in esp or WARN not in esp or '"microkernel-core"' not in esp:
        out.append("nonos-mk-esp does not name the record and warn on the bare default")
    return out


CASES = [("as built", lambda t: t, 0), ("macro write dropped", lambda t: t.replace("'$(2)$(NONOS_SMP_FEATURE)' " + WRITE, "''"), 1),
         ("warning dropped", lambda t: t.replace(WARN, "packed"), 1), ("default not recorded",
         lambda t: t.replace("'microkernel-core' " + WRITE, "''"), 1)]


def main() -> int:
    ap = argparse.ArgumentParser(description=__doc__.split("\n")[0])
    ap.add_argument("--self-test", action="store_true")
    ap.add_argument("--mk", default="mk/20-build.mk")
    a = ap.parse_args()
    text = Path(a.mk).read_text()
    if a.self_test:
        bad = [w for w, f, n in CASES if len(problems(f(text))) != n]
        print(f"esp-profile self-test: {len(CASES)} cases, {len(bad)} wrong {bad if bad else ''}")
        return 1 if bad else 0
    found = problems(text)
    for p in found:
        print(f"esp-profile: {p}")
    print(f"esp-profile: {len(found)} problems")
    return 1 if found else 0


if __name__ == "__main__":
    sys.exit(main())
