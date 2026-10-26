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
"""Every screen that shows the release reads it from VERSION: each surface
file still embeds the file, and no app source spells a release number."""

import re
import sys
import gate
from release_string import SURFACES, problems, sources


def mutants(root):
    """One changed file each, the check's name for it, and its new text."""
    files = sources(root)
    first = SURFACES[0]
    version = (root / "VERSION").read_text().strip()
    literal = f'pub const SHOWN: &str = "release {version}";\n'
    spelled = "const SHOWN: &str = \"NONOS 1.4.0\";\n"
    return [
        ("a surface spells the release", first, re.sub(
            r'include_(str|bytes)!\("[./]*VERSION"\)', '"0.0.0"', files[first])),
        ("an app writes VERSION's text", "userland/capsule_about/src/x.rs", literal),
        ("an app writes another release", "userland/desktop_shell/src/x.rs", spelled),
    ]


def self_test(root):
    """Every mutant adds a problem the tree lacks."""
    files = sources(root)
    base, missed = set(problems(root, files)), []
    for name, path, text in mutants(root):
        if not set(problems(root, {**files, path: text})) - base:
            missed.append(name)
    return missed


def main():
    args = gate.parser(__doc__).parse_args()
    if args.self_test:
        missed = self_test(args.root)
        print("release-string self-test:", "bites" if not missed else "MISSED " + ", ".join(missed))
        return int(bool(missed))
    found = problems(args.root, sources(args.root))
    for p in found:
        print("release-string:", p)
    print("release-string:", "ok" if not found else f"{len(found)} places off VERSION")
    return int(bool(found))


if __name__ == "__main__":
    sys.exit(main())
