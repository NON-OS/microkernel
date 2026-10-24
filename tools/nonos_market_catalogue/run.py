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
"""`nonos-market-catalogue --catalogue`: recipes loaded, both signed indexes
checked, each package fetched to its index size and listed. A fault that
leaves nothing true to list stops the run; a package that fails alone is
left out, said, and its cached copy dropped so the next run fetches anew."""

import sys
from pathlib import Path

from .alpine_index import records, signed_index
from .catalogue import catalogue_entries
from .fetch import fetch
from .names import cache_safe
from .recipes_file import load

BRANCHES = ("main", "community")
INDEX_CAP = 32 << 20


def table(cache: Path, release_url: str, arch: str) -> dict:
    """Both branches' records, each index's signature checked first."""
    out = {}
    for branch in BRANCHES:
        base, at = f"{release_url}/{branch}/{arch}", cache / f"{branch}-APKINDEX.tar.gz"
        raw = fetch(f"{base}/APKINDEX.tar.gz", at, INDEX_CAP)
        try:
            recs = records(signed_index(raw))
        except ValueError:
            at.unlink(missing_ok=True)
            raise
        for rec in recs.values():
            rec["base"] = base
        out.update(recs)
    return out


def apk_of(cache: Path, rec: dict) -> bytes:
    raw = fetch(f"{rec['base']}/{rec['P']}-{rec['V']}.apk", cache / f"{rec['P']}-{rec['V']}.apk", int(rec["S"]))
    if len(raw) != int(rec["S"]):
        raise ValueError(f"{len(raw)} bytes where the signed index says {rec['S']}")
    return raw


def catalogue_run(path: Path, cache: Path, release_url: str, arch: str, key: str, when_ms: int) -> list:
    """The catalogue's entries, or SystemExit with what stopped them."""
    try:
        recipes = load(path, Path("."))
        known = table(cache, release_url, arch)
        found = catalogue_entries(recipes, known, lambda rec: apk_of(cache, rec), key, when_ms)
    except (ValueError, TypeError, OSError) as why:
        sys.exit(f"catalogue: {why}")
    listed = {e["listing_id"] for e in found}
    for r in recipes:
        rec = known.get(r["package"])
        if rec and f"linux.{r['package']}" not in listed and cache_safe(rec):
            (cache / f"{rec['P']}-{rec['V']}.apk").unlink(missing_ok=True)
    print(f"catalogue: {len(found)} of {len(recipes)} apps", file=sys.stderr)
    return found
