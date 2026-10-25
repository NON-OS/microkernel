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
"""Hold every driver's kernel wait to the kernel's own (D5).

The kernel gives a request SLOW_BUDGET_MS (src/services/lifecycle/
reply_wait.rs) before it stops waiting. A driver that answers inside a
mirror of it, KERNEL_WAIT_MS (capsule_driver_nvme), must use the same
number, or it answers after the kernel gave up. Every KERNEL_WAIT_MS under
userland/ is held to the kernel's value; an unreadable budget fails.
"""

import argparse
import re
import sys
from pathlib import Path

BUDGET = "src/services/lifecycle/reply_wait.rs"
CONST = r"\bconst\s+{}\s*:\s*u\d+\s*=\s*([0-9_]+)\s*;"


def value(text: str, name: str):
    found = re.search(CONST.format(name), text)
    return int(found.group(1).replace("_", "")) if found else None


def check(root: Path) -> int:
    budget_file = root / BUDGET
    budget = value(budget_file.read_text(), "SLOW_BUDGET_MS") if budget_file.exists() else None
    if budget is None:
        print(f"kernel-wait: no SLOW_BUDGET_MS in {BUDGET}")
        return 1
    mirrors, problems = 0, []
    for path in sorted((root / "userland").rglob("*.rs")):
        if "target" in path.parts:
            continue
        got = value(path.read_text(errors="replace"), "KERNEL_WAIT_MS")
        if got is None:
            continue
        mirrors += 1
        if got != budget:
            rel = path.relative_to(root)
            problems.append(f"{rel}: KERNEL_WAIT_MS {got}, the kernel waits {budget}")
    for p in problems:
        print(f"kernel-wait: {p}")
    print(f"kernel-wait: SLOW_BUDGET_MS {budget}, {mirrors} mirrors, {len(problems)} problems")
    return 1 if problems else 0


def main() -> int:
    ap = argparse.ArgumentParser(description=__doc__)
    ap.add_argument("--root", type=Path, default=Path("."))
    ap.add_argument("--self-test", action="store_true")
    args = ap.parse_args()
    if args.self_test:
        sys.path.insert(0, str(Path(__file__).parent))
        from kernel_wait_fixture import self_test  # noqa: E402
        return self_test(check, BUDGET)
    return check(args.root)


if __name__ == "__main__":
    sys.exit(main())
