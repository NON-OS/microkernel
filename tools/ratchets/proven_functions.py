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
falls below the floor or when the unproven gap grows past its ceiling. A name
counts only in the modules that import its own crate's generated code, and a
function counts as substantive only when a theorem statement names it or a name
the file gives it.
"""

import argparse
import sys
from pathlib import Path

from proven_classify import classify
from proven_mirrors import mirrored_sources
from proven_report import report


def main():
    ap = argparse.ArgumentParser(description=__doc__.split('\n', 1)[0])
    ap.add_argument('--root', default='.', help='repository root')
    ap.add_argument('--json', action='store_true',
                    help='emit the counts as JSON for the evidence manifest')
    args = ap.parse_args()
    root = Path(args.root)

    dups = mirrored_sources(root)
    names, proven, bare, substantive = classify(root)
    if args.json:
        import json as _json
        print(_json.dumps({
            'extracted': len(names),
            'proven': len(proven),
            'substantive': len(substantive),
            'trivial': len(proven) - len(substantive),
            'unproven': len(bare),
        }))
        return 0
    return report(names, proven, bare, substantive, dups)


if __name__ == '__main__':
    sys.exit(main())
