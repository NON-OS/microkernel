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
"""check_kernel_wait.py on fixtures: a mirror that differs, and a budget
that cannot be read, both fail; a mirror that agrees passes."""

import tempfile
from pathlib import Path


def self_test(check, BUDGET: str) -> int:
    cases = [("35_000", "35_000", 0), ("35_000", "30_000", 1), (None, "35_000", 1)]
    for budget, mirror, want in cases:
        with tempfile.TemporaryDirectory() as d:
            root = Path(d)
            (root / BUDGET).parent.mkdir(parents=True)
            text = f"const SLOW_BUDGET_MS: u64 = {budget};\n" if budget else "// none\n"
            (root / BUDGET).write_text(text)
            drv = root / "userland/capsule_driver_x/src/limits.rs"
            drv.parent.mkdir(parents=True)
            drv.write_text(f"pub const KERNEL_WAIT_MS: u64 = {mirror};\n")
            if check(root) != want:
                print(f"kernel-wait: self-test failed on {budget} against {mirror}")
                return 1
    print("kernel-wait: self-test passed: a differing mirror and a lost budget fail")
    return 0
