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
"""Where the catalogue's packages come from, as the device gets them:
both branches' indexes fetched fresh and their signatures checked, each
listed package found by its own name (Index::listed), and fetched from the
first branch that serves it (install/get/package.rs), to its signed size."""

from pathlib import Path

from .alpine_index import signed_index
from .apkindex import listed, records, text
from .fetch import fetch

# As capsule_linux's install/run.rs: RELEASE, ARCH, BRANCHES (test_run pins them).
RELEASE, ARCH, BRANCHES = "v3.20", "x86_64", ("main", "community")
INDEX_CAP = 32 << 20


def table(mirror: str, cache: Path, names: list) -> dict:
    """Each name's listed record, from both signed indexes joined, main first."""
    joined, bases = b"", [f"{mirror}/{RELEASE}/{b}/{ARCH}" for b in BRANCHES]
    for branch, base in zip(BRANCHES, bases):
        raw = fetch(f"{base}/APKINDEX.tar.gz", cache / f"{branch}-APKINDEX.tar.gz", INDEX_CAP, fresh=True)
        joined += text(signed_index(raw))
    recs = records(joined)
    found = {n: listed(recs, n) for n in names}
    return {n: dict(r, base=bases[0], bases=bases) for n, r in found.items() if r}


def cached(cache: Path, branch: str, rec: dict) -> Path:
    """Kept under the branch that served it, so a warm cache never says
    another branch served it."""
    return cache / branch / f"{rec['P']}-{rec['V']}.apk"


def apk_of(cache: Path, rec: dict) -> bytes:
    """The package from the first branch that serves it, as get/package.rs."""
    size = int(rec["S"])
    for branch, base in zip(BRANCHES, rec["bases"]):
        at = cached(cache, branch, rec)
        try:
            raw = fetch(f"{base}/{rec['P']}-{rec['V']}.apk", at, size)
        except ValueError:
            continue
        if len(raw) != size:
            at.unlink(missing_ok=True)
            raise ValueError(f"{len(raw)} bytes where the signed index says {size}")
        rec["base"] = base
        return raw
    raise ValueError("no branch serves it")
