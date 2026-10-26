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
"""The default kernel rule waits for, and keeps, a profile build (B-02).
Under -j, `make nonos-mk-desktop-gui-prod nonos-mk-esp` built the bare core
kernel over the desktop's ELF in a fresh tree and packed it (mk/20-build.mk).
"""

import argparse
import re
import sys
from pathlib import Path

RULE = "$(TARGET_DIR)/x86_64-nonos/release/nonos-kernel: |"
FILTER = "$(filter $(KERNEL_BUILD_GOALS),$(MAKECMDGOALS))"


def builders(text: str) -> set:
    found, target = set(), None
    for line in text.splitlines():
        m = re.match(r"^([A-Za-z0-9_.%-]+):", line)
        target = m.group(1) if m else target
        if "call nonos_kernel_build" in line and target:
            found.add(target)
    return found


def problems(text: str) -> list:
    m = re.search(r"^KERNEL_BUILD_GOALS := ((?:.*\\\n)*.*)$", text, re.M)
    listed = set(m.group(1).replace("\\\n", " ").split()) if m else set()
    out = [f"{t} builds the kernel but is not in KERNEL_BUILD_GOALS" for t in sorted(builders(text) - listed)]
    rule = text[text.find(RULE) : text.find("\n\n", text.find(RULE))] if RULE in text else ""
    if FILTER not in rule.split("\n", 1)[0]:
        out.append("the default kernel rule does not wait for a kernel goal on the command line")
    guard = r"if \[ -e \$@ \]; then \\\n(?:.*\\\n)*?\s*else \$\(MAKE\) --no-print-directory nonos-mk-kernel-default; fi"
    if not re.search(guard, rule.partition("\n")[2]):
        out.append("the default kernel rule does not keep a built ELF in the shell that builds one")
    return out


CASES = [("as built", lambda t: t, 0), ("a goal unlisted", lambda t: t.replace(
         "nonos-mk-desktop-gui-prod nonos-mk-driver", "nonos-mk-driver", 1), 1),
         ("filter dropped", lambda t: t.replace(" " + FILTER, "", 1), 1),
         ("guard in its own shell", lambda t: t.replace("else $(MAKE) --no-print-directory", "exit 0; fi\n\t@$(MAKE) --no-print-directory", 1), 1)]


def main() -> int:
    ap = argparse.ArgumentParser(description=__doc__.split("\n")[0])
    ap.add_argument("--self-test", action="store_true")
    ap.add_argument("--mk", default="mk/20-build.mk")
    a = ap.parse_args()
    text = Path(a.mk).read_text()
    if a.self_test:
        bad = [w for w, f, n in CASES if len(problems(f(text))) != n]
        print(f"kernel-goals self-test: {len(CASES)} cases, {len(bad)} wrong {bad if bad else ''}")
        return 1 if bad else 0
    found = problems(text)
    for p in found:
        print(f"kernel-goals: {p}")
    print(f"kernel-goals: {len(found)} problems")
    return 1 if found else 0


if __name__ == "__main__":
    sys.exit(main())
