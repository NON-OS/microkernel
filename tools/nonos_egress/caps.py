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

"""Which capsules hold Network, and every crate each one builds from."""

import re
from pathlib import Path

NETWORK = 1 << 2
CAPS = re.compile(r"^\s*CAPSULE_(?:REQUIRED|OPTIONAL)_CAPS\s*:?=\s*(0x[0-9a-fA-F]+|\d+)\s*$")
PATH_DEP = re.compile(r'path\s*=\s*"([^"]+)"')


def mask(capsule_mk):
    """The capabilities a Capsule.mk asks for, required and optional."""
    out = 0
    for line in capsule_mk.read_text().splitlines():
        m = CAPS.match(line)
        if m:
            out |= int(m.group(1), 0)
    return out


def networked(root):
    """Each capsule whose Capsule.mk asks for Network, by directory name."""
    return sorted(p.parent.name for p in (root / "userland").glob("*/Capsule.mk")
                  if mask(p) & NETWORK)


def crates(root, names):
    """The capsules named and every userland crate they depend on by path.
    Proof crates and vendored code are left out: neither ships a dial."""
    userland = (root / "userland").resolve()
    seen, todo = set(), [userland / n for n in names]
    while todo:
        crate = todo.pop()
        if crate in seen:
            continue
        seen.add(crate)
        manifest = crate / "Cargo.toml"
        if not manifest.exists():
            continue
        for rel in PATH_DEP.findall(manifest.read_text()):
            dep = (crate / rel).resolve()
            inside = dep.is_relative_to(userland) and dep.is_dir()
            if inside and not dep.name.endswith("_proofs"):
                todo.append(dep)
    return sorted(seen)
