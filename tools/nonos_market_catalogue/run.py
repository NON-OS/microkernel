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
"""`nonos-market-catalogue --catalogue`: recipes loaded, packages found and
fetched as the device does (sources.py), and listed. A fault that leaves
nothing true to list stops the run; a package that fails alone is left
out, said, and its cached copy dropped so the next run fetches anew."""

import subprocess
import sys
from pathlib import Path

from .catalogue import catalogue_entries
from .names import cache_safe
from .recipes_file import load
from .sources import ARCH, BRANCHES, RELEASE, apk_of, cached, table


def catalogue_run(path: Path, cache: Path, mirror: str, release: str, arch: str, key: str, when_ms: int) -> list:
    """The catalogue's entries, or SystemExit with what stopped them."""
    if (release, arch) != (RELEASE, ARCH):
        sys.exit(f"catalogue: the device installs from Alpine {RELEASE} {ARCH}, not {release} {arch}")
    cache = cache / "catalogue" / RELEASE / ARCH
    try:
        recipes = load(path, Path("."))
        known = table(mirror, cache, [r["package"] for r in recipes])
        found = catalogue_entries(recipes, known, lambda rec: apk_of(cache, rec), key, when_ms)
    except (ValueError, TypeError, OSError, subprocess.CalledProcessError) as why:
        sys.exit(f"catalogue: {why}")
    listed_ids = {e["listing_id"] for e in found}
    for r in recipes:
        rec = known.get(r["package"])
        if rec and f"linux.{r['package']}" not in listed_ids and cache_safe(rec):
            for branch in BRANCHES:
                cached(cache, branch, rec).unlink(missing_ok=True)
    print(f"catalogue: {len(found)} of {len(recipes)} apps", file=sys.stderr)
    return found
