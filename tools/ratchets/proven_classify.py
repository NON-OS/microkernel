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
"""Each extracted function: proven, bare, and substantive or not."""

import json
import re

from proven_files import imports_module, proof_files
from proven_statements import aliases, statements

def classify(root):
    # Read the crate manifest, not EVIDENCE.json. The evidence script calls this
    # to fill its own counts field, so reading its output here would make the
    # numbers a function of the last run rather than of the tree.
    manifest = json.loads(
        (root / 'verification/extraction/crates.json').read_text())
    names = sorted({s for c in manifest['crates'] for s in c['starts']})
    files = proof_files(root)
    # A name counts only in files that import its own crate's module. Leaf
    # names repeat across crates (`new`, `is_present`, `leaf`), and matching
    # them anywhere once counted functions no theorem mentions.
    own = {}
    for c in manifest['crates']:
        mine = [t for t in files if imports_module(t, c['lean'])]
        for st in c['starts']:
            prev = own.setdefault(st, [])
            prev.extend(mine)

    proven, bare, substantive = [], [], []
    for name in names:
        leaf = name.split('::')[-1]
        mine = own.get(name, [])
        if re.search(r'\b%s\b' % re.escape(leaf), '\n'.join(mine)):
            proven.append(name)
            pat = re.compile(r'\b(%s)\b' % '|'.join(
                map(re.escape, sorted(aliases(mine, leaf)))))
            if any(pat.search(sig) for t in mine for sig in statements(t)):
                substantive.append(name)
        else:
            bare.append(name)
    return names, proven, bare, substantive
