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

"""The hardware coverage map, written from docs/hardware/coverage-parts.tsv.

    python3 tools/hardware/coverage.py           write docs/hardware/coverage-*.md
    python3 tools/hardware/coverage.py --check   exit 1 if a page is out of date

Two columns come from the tree, never from the parts file: how many of a
part's ids its driver's own table names, and whether the Standard image
carries the driver's feature. So when a table or the image changes, the map
changes with it, and --check says so.
"""

import sys

from coverage_facts import ROOT, image_features
from coverage_parts import facts, rows
from coverage_render import page

def main(argv):
    features = image_features()
    every = [facts(r, features) for r in rows()]
    stale = []
    for area in dict.fromkeys(r["area"] for r in every):
        path = ROOT / f"docs/hardware/coverage-{area}.md"
        text = page(area, [r for r in every if r["area"] == area])
        if "--check" in argv:
            if not path.is_file() or path.read_text() != text:
                stale.append(str(path.relative_to(ROOT)))
        else:
            path.write_text(text)
    if stale:
        print("coverage: out of date, run tools/hardware/coverage.py:", *stale)
        return 1
    return 0


if __name__ == "__main__":
    sys.exit(main(sys.argv[1:]))
