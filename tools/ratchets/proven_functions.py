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
FLOOR = 410
GAP_CEILING = 2
# Functions carrying a property beyond "this wrapper is its method". That
# wrapper theorem is real, and it is what ties a manifest entry to the method a
# theorem talks about, but on its own it says nothing about behaviour. Counting
# the two together would be the inflation this file exists to stop.
SUBSTANTIVE_FLOOR = 141

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
        m = re.search(r'#\[path = "([^"]+)"\]', lib.read_text())
        if m:
            out.setdefault(m.group(1).split('../')[-1], []).append(c['name'])
    return {k: v for k, v in out.items() if len(v) > 1}


def classify(root):
    manifest = json.loads(
        (root / 'verification/evidence/EVIDENCE.json').read_text())
    names = manifest['proof_systems']['lean_extraction']['extracted_functions']
    code = proof_text(root)
    # A wrapper theorem names the function only on its own line; a substantive
    # theorem names it somewhere else. Strip the generated wrapper block and see
    # what still mentions it.
    # Only continuation lines, never blank ones: `\s+` matches a newline, so a
    # greedy version of this ate everything after the first wrapper theorem and
    # reported every crate as wrapper-only.
    without_wrappers = re.sub(
        r'theorem the_\w+_wrapper_is_its_method[^\n]*\n(?:[ \t]+[^\n]*\n)*', '', code)

    proven, bare, substantive = [], [], []
    for name in names:
        leaf = name.split('::')[-1]
        pat = r'\b%s\b' % re.escape(leaf)
        if re.search(pat, code):
            proven.append(name)
            if re.search(pat, without_wrappers):
                substantive.append(name)
        else:
            bare.append(name)
    return names, proven, bare, substantive


def main():
    ap = argparse.ArgumentParser(description=__doc__.split('\n', 1)[0])
    ap.add_argument('--root', default='.', help='repository root')
    args = ap.parse_args()
    root = Path(args.root)

    dups = mirrored_sources(root)
    names, proven, bare, substantive = classify(root)
    print('extracted and CI-diffed  %4d' % len(names))
    print('carrying a proof         %4d   (floor %d)' % (len(proven), FLOOR))
    print('extracted, no proof      %4d   (ceiling %d)' % (len(bare), GAP_CEILING))
    print('of those, with a property beyond the wrapper  %4d   (floor %d)'
          % (len(substantive), SUBSTANTIVE_FLOOR))
    for name in bare:
        print('   no proof:', name)

    failed = False
    if dups:
        print('\nkernel files mirrored by more than one crate, so their functions '
              'are counted twice:', file=sys.stderr)
        for path, crates in dups.items():
            print('  %s  <- %s' % (path, ', '.join(crates)), file=sys.stderr)
        failed = True
    if len(proven) < FLOOR:
        print('\nproven count fell to %d, floor is %d' % (len(proven), FLOOR),
              file=sys.stderr)
        failed = True
    if len(bare) > GAP_CEILING:
        print('\n%d extracted functions carry no proof, ceiling is %d'
              % (len(bare), GAP_CEILING), file=sys.stderr)
        failed = True
    if len(substantive) < SUBSTANTIVE_FLOOR:
        print('\nsubstantive count fell to %d, floor is %d'
              % (len(substantive), SUBSTANTIVE_FLOOR), file=sys.stderr)
        failed = True
    if len(proven) > FLOOR:
        print('\nproven count is %d; raise FLOOR in this file' % len(proven))
    if len(substantive) > SUBSTANTIVE_FLOOR:
        print('substantive count is %d; raise SUBSTANTIVE_FLOOR' % len(substantive))
    return 1 if failed else 0


if __name__ == '__main__':
    sys.exit(main())
