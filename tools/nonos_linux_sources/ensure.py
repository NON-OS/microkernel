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

"""Every source recipe's content present at OUT and held to its pin, or the
run stopped naming each package that is not (__init__.py says the order)."""

import shutil
import sys
import tempfile
from pathlib import Path

from . import asset, cache, store
from .pins import ROOT, holds, pins, recipes

BUILT = ("docs/using/darkfi.md, \"The builds\": tools/nix/sources/darkfi-build.sh, then "
         "darkfi-wallet-box.sh, darkirc-box.sh or darkfi-app-*.sh, on a machine that builds them")


class Absent(ValueError):
    """No cache, content branch or producer here gives a recipe's content."""


def say(line: str) -> None:
    print(f"  linux-sources: {line}", file=sys.stderr)


def one(recipe: dict, pin: tuple, out: Path, kept: Path, repo: Path, fetched: list) -> str:
    """Where `recipe`'s content came from, or ValueError naming why none."""
    package = recipe["package"]
    tar = out / f"{package}.tar"
    if holds(tar, package, pin):
        if not (kept / f"{pin[0]}.tar").is_file():
            cache.keep(kept, tar, pin)
        return "present"
    if (out / package).exists():
        raise ValueError(f"{package}: {out / package} is a tree, not the pinned tar; remove it")
    tar.unlink(missing_ok=True)
    if cache.take(kept, package, pin, tar):
        return "the cache"
    if not fetched:
        fetched.append(store.fetched(repo))
    if fetched[0] and store.take(repo, package, tar):
        if holds(tar, package, pin):
            cache.keep(kept, tar, pin)
            return f"{store.BRANCH}"
        tar.unlink()
        say(f"{package}: {store.BRANCH}'s tar is not the pinned content; not taken")
    if asset.is_asset(recipe):
        lack = asset.missing()
        if lack:
            raise Absent(f"{package}: {', '.join(lack)} not found here, so upstream's release "
                         "cannot be held to its pin; not made")
        with tempfile.TemporaryDirectory() as d:
            if asset.make(recipe, Path(d)) and holds(Path(d) / f"{package}.tar", package, pin):
                shutil.copyfile(Path(d) / f"{package}.tar", tar)
                cache.keep(kept, tar, pin)
                return "upstream's release"
        raise Absent(f"{package}: upstream's release did not give the pinned content")
    raise Absent(f"{package}: no cache, no {store.BRANCH} and no producer here gives the pinned "
                     f"content {pin[0]}; it is built by {BUILT}")


def ensure(out: Path, kept: Path = None, repo: Path = ROOT, absent_ok: bool = False) -> list:
    """(package, where from) for each source recipe; ValueError naming every
    package that could not be had, after trying them all. `absent_ok`, for a
    development build only: a recipe nothing here gives is said in one line
    and left absent, so the catalogue leaves it out; a lock with no pin for a
    recipe, or a tree in place of its tar, still stops the run."""
    out.mkdir(parents=True, exist_ok=True)
    kept = kept or cache.where()
    pinned, done, failed, fetched = pins(), [], [], []
    for r in recipes():
        package = r["package"]
        if package not in pinned:
            failed.append(f"{package}: the sources lock pins no content for it")
            continue
        try:
            done.append((package, one(r, pinned[package], out, kept, repo, fetched)))
        except Absent as why:
            if not absent_ok:
                failed.append(str(why))
                continue
            say(f"{why}; left out of this development build's catalogue")
        except ValueError as why:
            failed.append(str(why))
    if failed:
        raise ValueError("; ".join(failed))
    return done
