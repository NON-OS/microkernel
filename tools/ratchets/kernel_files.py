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
"""The kernel sources a ratchet counts, from the newest library dep-info."""

import sys
from pathlib import Path

from ring0 import depinfo_sources, newest_depinfo

DEPS = Path("target/x86_64-nonos/release/deps")


def kernel_files(root, depinfo=None, target_deps=DEPS):
    """(dep-info, sources), or (None, []) with the reason printed."""
    d = depinfo or newest_depinfo(root / target_deps)
    if d is None or not d.is_file():
        print("no kernel dep-info; build the kernel first", file=sys.stderr)
        return None, []
    files = depinfo_sources(d, root)
    if not files:
        print(f"{d} lists no kernel sources", file=sys.stderr)
    return d, files
