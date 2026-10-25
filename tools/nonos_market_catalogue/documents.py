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

from .names import listing


def split(index: dict) -> dict:
    """listing_id -> a one-entry index with the same header."""
    if not isinstance(index, dict) or index.get("schema_version") != 3:
        raise ValueError("not a schema-3 index")
    if not isinstance(index.get("entries"), list):
        raise TypeError("the index has no entries list")
    head = {k: v for k, v in index.items() if k != "entries"}
    docs = {}
    for e in index["entries"]:
        lid = listing(e.get("listing_id") if isinstance(e, dict) else None)
        if lid in docs:
            raise ValueError(f"{lid} is listed twice")
        docs[lid] = dict(head, entries=[e])
    return docs


def write(index: dict, out: Path) -> list:
    """Each document as <listing_id>.json in `out`; the paths written."""
    out.mkdir(parents=True, exist_ok=True)
    done = []
    for lid, doc in sorted(split(index).items()):
        path = out / f"{lid}.json"
        path.write_text(json.dumps(doc, indent=2) + "\n")
        done.append(path)
    return done


def main(argv) -> int:
    if len(argv) != 2 or argv[0] in ("-h", "--help"):
        print(__doc__, file=sys.stderr)
        return 0 if argv[:1] in (["-h"], ["--help"]) else 2
    try:
        index = json.loads(Path(argv[0]).read_text())
        for p in write(index, Path(argv[1])):
            print(p)
    except (OSError, ValueError, TypeError) as why:
        print(f"documents: {why}", file=sys.stderr)
        return 1
    return 0


if __name__ == "__main__":
    sys.exit(main(sys.argv[1:]))
