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
"""The counts against their floors, and what fails."""

import sys

from proven_floors import FLOOR, GAP_CEILING, SUBSTANTIVE_FLOOR

def report(names, proven, bare, substantive, dups):
    """Print the counts and every failure; the exit code."""
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
