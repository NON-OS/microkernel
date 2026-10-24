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
"""One document per listing, from an index the generator wrote: each an
index of one entry, so `marketplace-index sign-release` and `encode` take
it as it is, and a review reads one app at a time.

    python3 -m nonos_market_catalogue.documents INDEX.json OUTDIR

run from tools/. A document is unsigned: the publisher signs it."""

import json
import sys
from pathlib import Path


def split(index: dict) -> dict:
    """listing_id -> a one-entry index with the same header."""
    head = {k: v for k, v in index.items() if k != "entries"}
    return {e["listing_id"]: dict(head, entries=[e]) for e in index["entries"]}


def write(index: dict, out: Path) -> list:
    """Each document as <listing_id>.json in `out`; the paths written."""
    out.mkdir(parents=True, exist_ok=True)
    done = []
    for listing, doc in sorted(split(index).items()):
        path = out / f"{listing}.json"
        path.write_text(json.dumps(doc, indent=2) + "\n")
        done.append(path)
    return done


if __name__ == "__main__":
    if len(sys.argv) != 3:
        sys.exit(__doc__)
    for p in write(json.loads(Path(sys.argv[1]).read_text()), Path(sys.argv[2])):
        print(p)
