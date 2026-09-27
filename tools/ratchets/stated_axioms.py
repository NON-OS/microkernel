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
"""Every axiom the extraction tier depends on has to be stated in ASSUMPTIONS.md.

An opaque model is a fine thing to depend on and a silent one is not. This reads
the axiom profile out of a Lean build log and fails when something appears that
the register does not mention, so the register cannot fall behind the corpus.

A `sorryAx` is never acceptable and is reported separately from an unstated
axiom, because the two mean different things: one is a hole in a proof, the other
is a dependency nobody wrote down.

Run it over the output of `lake build` for the extraction project:

    lake build > /tmp/build.txt 2>&1
    python3 tools/ratchets/stated_axioms.py --log /tmp/build.txt
"""

import argparse
import re
import sys
from pathlib import Path

STANDARD = {'propext', 'Classical.choice', 'Quot.sound'}

# bv_decide names its axiom after the theorem that used it, so the register
# states the shape rather than every instance.
BV_DECIDE = re.compile(r'^.*\._native\.bv_decide\.ax_\d+_\d+$')
BV_DECIDE_ENTRY = '<theorem>._native.bv_decide.ax_N'


def axioms_in(log_text):
    """Every non-standard axiom named in a build log, with a use count."""
    flat = re.sub(r'\n\s+', ' ', log_text)
    found = {}
    for group in re.findall(r'depends on axioms: \[([^\]]*)\]', flat):
        for name in (x.strip() for x in group.split(',')):
            if not name or name in STANDARD:
                continue
            key = BV_DECIDE_ENTRY if BV_DECIDE.match(name) else name
            found[key] = found.get(key, 0) + 1
    return found


def stated_in(register_text):
    """Names the register mentions, as a set.

    Matching is by mention rather than by a table, because the register is prose
    and should stay readable. A name in a code span counts as stated.
    """
    return set(re.findall(r'`([^`]+)`', register_text))


def covered(name, stated):
    if name in stated:
        return True
    # `core.sync.atomic.*` covers the family the register describes as a group.
    for entry in stated:
        if entry.endswith('*') and name.startswith(entry[:-1]):
            return True
    return False


def main():
    ap = argparse.ArgumentParser(description=__doc__.split('\n', 1)[0])
    ap.add_argument('--log', required=True, help='output of lake build')
    ap.add_argument('--register',
                    default='verification/extraction/ASSUMPTIONS.md')
    ap.add_argument('--root', default='.')
    args = ap.parse_args()

    root = Path(args.root)
    log = Path(args.log).read_text(errors='replace')
    register = (root / args.register).read_text()

    if 'sorryAx' in log:
        print('a sorry is present in the extraction corpus', file=sys.stderr)
        return 1

    found = axioms_in(log)
    stated = stated_in(register)
    unstated = sorted(n for n in found if not covered(n, stated))

    print('non-standard axioms in the build  %3d' % len(found))
    print('unstated in the register          %3d' % len(unstated))
    for name in sorted(found):
        mark = ' ' if covered(name, stated) else '!'
        print('  %s %-58s %d' % (mark, name, found[name]))

    if unstated:
        print('\nthese axioms are not mentioned in %s:' % args.register,
              file=sys.stderr)
        for name in unstated:
            print('  %s' % name, file=sys.stderr)
        print('\nAdd each one with what it models and why it is not a proof '
              'axiom. Do not widen the gate to make this pass.', file=sys.stderr)
        return 1
    return 0


if __name__ == '__main__':
    sys.exit(main())
