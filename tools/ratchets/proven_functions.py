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
"""Extracted functions that carry a proof, as a number that may only rise.

EVIDENCE.json lists what Charon and Aeneas lower out of the kernel. That is a
different set from the functions a theorem actually reasons about, and for a
while it was quietly the larger of the two: six functions were extracted, diffed
by CI and counted, with nothing proven about them. A manifest that counts those
beside the rest is inflating its own headline.

This walks the refinement modules with comments stripped, so a function named
only in a file header does not count as proven, and fails when the proven count
falls below the floor or when the unproven gap grows past its ceiling.
"""

import argparse
import json
import re
import sys
from pathlib import Path

# Raise these when the numbers improve. They may never be lowered.
FLOOR = 90
GAP_CEILING = 2

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


def proof_text(root):
    out = []
    for d in PROOF_DIRS:
        for path in sorted((root / d).glob('*.lean')):
            if 'Refinement' in path.name or path.name in PROOF_MODULES:
                out.append(strip_comments(path.read_text(errors='replace')))
    return '\n'.join(out)


def classify(root):
    manifest = json.loads(
        (root / 'verification/evidence/EVIDENCE.json').read_text())
    names = manifest['proof_systems']['lean_extraction']['extracted_functions']
    code = proof_text(root)
    proven, bare = [], []
    for name in names:
        leaf = name.split('::')[-1]
        target = proven if re.search(r'\b%s\b' % re.escape(leaf), code) else bare
        target.append(name)
    return names, proven, bare


def main():
    ap = argparse.ArgumentParser(description=__doc__.split('\n', 1)[0])
    ap.add_argument('--root', default='.', help='repository root')
    args = ap.parse_args()
    root = Path(args.root)

    names, proven, bare = classify(root)
    print('extracted and CI-diffed  %4d' % len(names))
    print('carrying a proof         %4d   (floor %d)' % (len(proven), FLOOR))
    print('extracted, no proof      %4d   (ceiling %d)' % (len(bare), GAP_CEILING))
    for name in bare:
        print('   no proof:', name)

    failed = False
    if len(proven) < FLOOR:
        print('\nproven count fell to %d, floor is %d' % (len(proven), FLOOR),
              file=sys.stderr)
        failed = True
    if len(bare) > GAP_CEILING:
        print('\n%d extracted functions carry no proof, ceiling is %d'
              % (len(bare), GAP_CEILING), file=sys.stderr)
        failed = True
    if len(proven) > FLOOR:
        print('\nproven count is %d; raise FLOOR in this file' % len(proven))
    return 1 if failed else 0


if __name__ == '__main__':
    sys.exit(main())
