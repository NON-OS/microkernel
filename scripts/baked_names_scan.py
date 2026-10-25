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
"""What check_baked_names.py reads: spawn site names, the kernel's tables,
and each capsule's signed namespace from its Capsule.mk."""

import re
from pathlib import Path

LOAD = "src/kernel_core/process_spawn/capsule_spawn/from_vfs/load"
LITERAL = r'\bname: "([^"]+)"|const SERVICE_NAME: &str = "([^"]+)"|pub const JOB: &str = "([^"]+)"'


def spawn_names(root: Path) -> set:
    """Every name a baked spawn site declares: its spec, its Linux roles,
    its tool table."""
    names = set()
    for p in (root / "src").rglob("*.rs"):
        rel, text = str(p.relative_to(root)), p.read_text(errors="ignore")
        if rel.startswith(LOAD):
            continue
        tools = "tool_capsules" in rel
        if not ("CapsuleSpecVerified" in text or tools or ("capsule_linux" in rel and "roles" in rel)):
            continue
        names |= {next(g for g in m.groups() if g) for m in re.finditer(LITERAL, text)}
        if tools:
            names |= set(re.findall(r'b?"(tool\.[a-z0-9._-]+)"', text))
    return names


def table(root: Path) -> dict:
    text = "".join((root / LOAD / f).read_text() for f in ("baked_names.rs", "baked_apps.rs"))
    return dict(re.findall(r'\("([^"]+)", "([^"]*)"\)', text))


def families(root: Path) -> list:
    m = re.search(r"FAMILIES: &\[&str\] = &\[([^\]]*)\]", (root / LOAD / "baked_names.rs").read_text())
    return re.findall(r'"([^"]+)"', m.group(1)) if m else []


def namespaces(root: Path) -> dict:
    owned = {}
    for mk in (root / "userland").glob("*/Capsule.mk"):
        t = mk.read_text()
        ns = re.search(r"CAPSULE_NAMESPACE\s*:=\s*(\S+)", t)
        svc = re.search(r"CAPSULE_SERVICE_ENDPOINT\s*:=\s*service:\d+:(\S+)", t)
        if ns and svc:
            owned[svc.group(1)] = ns.group(1)
    return owned
