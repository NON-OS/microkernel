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
"""check_mirror_caps: a capsule's README quoting another mask than its manifest."""

import re

from mirror_caps_bits import manifest


def stale_readmes(root):
    out = []
    for mk in sorted((root / "userland").glob("*/Capsule.mk")):
        required = manifest(mk.read_text(), "CAPSULE_REQUIRED_CAPS")
        readme = mk.parent / "README.md"
        if required is None or not readme.exists():
            continue
        for n, line in enumerate(readme.read_text().splitlines(), 1):
            for quoted in re.findall(r"CAPSULE_REQUIRED_CAPS\s*:?=\s*`?(0x[0-9a-fA-F]+)", line):
                if int(quoted, 16) != int(required, 16):
                    out.append(f"{readme.relative_to(root)}:{n}: quotes {quoted}, the manifest says {required}")
    return out
