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
"""The refinement modules a proof is looked for in, comments stripped."""

import re

PROOF_MODULES = ('CapsComplete.lean', 'Closure.lean')
PROOF_DIRS = (
    'verification/extraction/lean/NonosExtraction',
    'verification/extraction/lean/Policy',
)


def strip_comments(src):
    """Lean block and line comments, so a header mention is not a proof."""
    src = re.sub(r'/-.*?-/', ' ', src, flags=re.S)
    src = re.sub(r'--[^\n]*', ' ', src)
    return src


def proof_files(root):
    """Each refinement module's text with comments stripped."""
    out = []
    for d in PROOF_DIRS:
        for path in sorted((root / d).glob('*.lean')):
            if 'Refinement' in path.name or path.name in PROOF_MODULES:
                out.append(strip_comments(path.read_text(errors='replace')))
    return out


def imports_module(text, lean):
    """Whether a proof file imports the generated module of this crate."""
    if lean == 'Policy':
        return re.search(r'^import Policy(\.\w+)*$', text, re.M) is not None
    return re.search(r'^import NonosExtraction\.%s$' % re.escape(lean), text, re.M) is not None
