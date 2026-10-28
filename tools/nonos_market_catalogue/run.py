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
fetched as the device does (sources.py), and listed, each at the version
its lock pins (lock.py). Any fault stops the run, so a seal lists every
recipe or none: an index past the lock, or a package that fails, is said
by name, and a failed package's cached copy is dropped so the next run
fetches it anew."""

import subprocess
import sys
from pathlib import Path

from . import listed_with, lock, logos, source_lock
from .catalogue import catalogue_entries
from .names import cache_safe
from .recipes_file import load
from .source_recipe import is_source
from .source_run import source_entries
from .sources import ARCH, BRANCHES, RELEASE, apk_of, cached, table


def catalogue_run(path: Path, cache: Path, mirror: str, release: str, arch: str, key: str, when_ms: int,
                  update_lock: bool = False, site: tuple = ("", None, None)) -> list:
    """The catalogue's entries, or SystemExit with what stopped them. `update_lock`
    writes the locks from this run instead of holding to them; `site` is source_run.py's."""
    if (release, arch) != (RELEASE, ARCH):
        sys.exit(f"catalogue: the device installs from Alpine {RELEASE} {ARCH}, not {release} {arch}")
    cache, pinned = cache / "catalogue" / RELEASE / ARCH, path.with_name(path.stem + ".lock.json")
    try:
        recipes = load(path, Path("."))
        marks = {r["package"]: logos.digests(r, Path(".")) for r in recipes}
        # The listing signs the raster's BLAKE3 as its logo_hash.
        recipes = [dict(r, logo_hash=marks[r["package"]][1]) for r in recipes]
        built, recipes = [r for r in recipes if is_source(r)], [r for r in recipes if not is_source(r)]
        names, extras = [r["package"] for r in recipes], listed_with.extras(recipes)
        known = listed_with.in_index(table(mirror, cache, names + extras), extras)
        pins = None if update_lock else lock.read(pinned)
        if pins is not None:
            lock.held(known, pins, names, extras)
            logos.held(marks, pins)
        found = catalogue_entries(recipes, known, lambda rec: apk_of(cache, rec), key, when_ms)
        if pins is not None:
            lock.bytes_held(pins, found)
    except (ValueError, TypeError, OSError, subprocess.CalledProcessError) as why:
        sys.exit(f"catalogue: {why}")
    listed_ids = {e["listing_id"] for e in found}
    for r in recipes:
        rec = known.get(r["package"])
        if rec and f"linux.{r['package']}" not in listed_ids and cache_safe(rec):
            for branch in BRANCHES:
                cached(cache, branch, rec).unlink(missing_ok=True)
    print(f"catalogue: {len(found)} of {len(recipes)} apps", file=sys.stderr)
    if len(found) != len(recipes):
        sys.exit("catalogue: a recipe was not listed; a seal lists every recipe or none")
    try:
        built = source_entries(built, site, source_lock.path_of(path), marks, key, when_ms, update_lock)
    except (ValueError, TypeError, OSError) as why:
        sys.exit(f"catalogue: {why}")
    if update_lock:
        lock.write(pinned, known, names, found, marks, extras)
    return found + built
