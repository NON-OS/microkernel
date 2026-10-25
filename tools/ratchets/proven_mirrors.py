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
"""Kernel files mirrored by more than one extraction crate."""

import json
import re

def mirrored_sources(root):
    """Kernel file each crate mirrors, so two crates cannot cover one file.

    The sweep re-extracted two files that already had hand-built crates, which
    counted twenty-eight functions twice. Nothing caught it because both crates
    were real and both extracted cleanly; the manifest simply listed the same
    kernel code under two names.
    """
    out = {}
    manifest = json.loads(
        (root / 'verification/extraction/crates.json').read_text())
    for c in manifest['crates']:
        lib = root / c['dir'] / 'src/lib.rs'
        if not lib.is_file():
            continue
        text = lib.read_text()
        m = (re.search(r'#\[path = "([^"]+)"\]', text)
             or re.search(r'mirroring `([^`]+)`', text))
        if m:
            out.setdefault(m.group(1).split('../')[-1], []).append(c['name'])
    return {k: v for k, v in out.items() if len(v) > 1}
