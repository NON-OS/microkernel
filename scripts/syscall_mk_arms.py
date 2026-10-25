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
#
# You should have received a copy of the GNU Affero General Public License
# along with this program. If not, see <https://www.gnu.org/licenses/>.
"""Where check_syscall_abi finds the microkernel's dispatch arms.

`route` asks the families under `microkernel/dispatch/` first and, for a
number none of them took, the ACPI capsule's `acpi::handle` last. That
capsule's arms count only while the fallback is still in `route`: take the
fallback out and its arms stop counting with it.

Before any family, `route` asks `retired::handle`, which answers EINVAL for a
number retired under the stable-ABI rule. A retired number is published and
reaches that gate rather than an arm, and counts as reaching it only while the
gate is still in `route`.
"""

import re
from pathlib import Path

MK_ACPI_DISPATCH = Path("src/syscall/microkernel/acpi/dispatch.rs")
# How `route` hands a number no family took to the ACPI capsule's arms.
MK_ACPI_FALLBACK = "microkernel::acpi::handle("
MK_RETIRED = Path("src/syscall/microkernel/retired.rs")
# How `route` answers a retired number before any family sees it.
MK_RETIRED_GATE = "microkernel::retired::handle("
RETIRED_RE = re.compile(r"\bRETIRED\s*:[^=]*=\s*\[(.*?)\];", re.S)
TAG_RE = re.compile(r'\*b"(\w{4})"')


def mk_arms(root, dispatch, arm_re):
    """Every `SYS_*` name an arm matches on, under `dispatch` and, while
    `route` falls back to it, in the ACPI capsule's dispatch."""
    text = "\n".join(p.read_text() for p in sorted((root / dispatch).rglob("*.rs")))
    acpi = root / MK_ACPI_DISPATCH
    if MK_ACPI_FALLBACK in text and acpi.is_file():
        text += "\n" + acpi.read_text()
    return {arm.strip() for group in arm_re.findall(text) for arm in group.split("|")}


def mk_retired(root, dispatch):
    """Every tag `retired.rs` lists in RETIRED, while `route` under `dispatch`
    still asks `retired::handle` first; none once the gate is gone."""
    text = "\n".join(p.read_text() for p in sorted((root / dispatch).rglob("*.rs")))
    retired = root / MK_RETIRED
    if MK_RETIRED_GATE not in text or not retired.is_file():
        return set()
    table = RETIRED_RE.search(retired.read_text())
    return set(TAG_RE.findall(table.group(1))) if table else set()
