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
"""Hold the kernel's reserved service names to the image's spawn sites (K18).

A capsule loaded at run time may not take a name a baked spawn site owns
(load/baked_names.rs, baked_apps.rs). A spawn site whose name the tables miss
is a name a store capsule could take; an entry with no spawn site left is a
reservation for nothing; a namespace that is not the one the capsule's
Capsule.mk signs it under would refuse the real capsule or admit another.
"""

import argparse
import sys
from pathlib import Path

sys.path.insert(0, str(Path(__file__).parent))
from baked_names_scan import families, namespaces, spawn_names, table  # noqa: E402


def check(root: Path) -> int:
    names, fams = spawn_names(root), families(root)
    rows, owned = table(root), namespaces(root)
    covered = lambda n: any(n == r or n.startswith(r + ".") for r in list(rows) + fams)
    problems = [f"spawn site name {n} is not reserved" for n in sorted(names) if not covered(n)]
    problems += [f"reserved {r} has no spawn site" for r in sorted(rows)
                 if not any(n == r or n.startswith(r + ".") for n in names)]
    problems += [f"reserved {r} under namespace {ns!r}, its Capsule.mk says {owned.get(r, '')!r}"
                 for r, ns in sorted(rows.items()) if ns != owned.get(r, "")]
    for p in problems:
        print(f"baked-names: {p}")
    print(f"baked-names: {len(names)} spawn names, {len(rows)} reserved, {len(problems)} problems")
    return 1 if problems else 0


def main() -> int:
    ap = argparse.ArgumentParser(description=__doc__)
    ap.add_argument("--root", type=Path, default=Path("."))
    ap.add_argument("--self-test", action="store_true")
    args = ap.parse_args()
    if args.self_test:
        from baked_names_fixture import self_test  # noqa: E402
        return self_test(check)
    return check(args.root)


if __name__ == "__main__":
    sys.exit(main())
