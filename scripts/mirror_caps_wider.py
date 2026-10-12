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
"""check_mirror_caps: every spawn mirror's grant held to its manifest."""

import re

from mirror_caps_bits import capability_bits, manifest, value


def wider(root):
    bits = capability_bits(root)
    spawns = {p: p.read_text() for p in (root / "src").rglob("spawn.rs")}
    out, checked = [], 0
    for mk in sorted((root / "userland").glob("*/Capsule.mk")):
        text = mk.read_text()
        name = (manifest(text, "CAPSULE_SERVICE_ENDPOINT") or "").split(":")[-1]
        if not name:
            continue
        allowed = int(manifest(text, "CAPSULE_REQUIRED_CAPS") or "0", 16)
        allowed |= int(manifest(text, "CAPSULE_OPTIONAL_CAPS") or "0", 16)
        for path, src in spawns.items():
            if f'SERVICE_NAME: &str = "{name}"' not in src:
                continue
            consts = {m.group(1): value(m.group(2), bits, {})
                      for m in re.finditer(r"const (\w+): u64 =([^;]+);", src, re.S)}
            for m in re.finditer(r"requested_caps:\s*([^,]+?),\n", src, re.S):
                checked += 1
                grant = value(m.group(1), bits, consts)
                if grant & ~allowed:
                    out.append(f"{path.relative_to(root)}: grants {grant:#x} to {name}, whose manifest "
                               f"allows {allowed:#x} ({mk.relative_to(root)})")
    return out, checked
