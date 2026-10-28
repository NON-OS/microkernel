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
"""Listings for the source recipes (source_recipe.py): each content bundled
(source_content.py), held to the sources lock, written to the package-out
directory as <blake3>.tar, listed as `linux.nonos-<package>` (`linux.kali.` in
Debian's family, source_asset.py), fetched from the NONOS mirror at /linux/<blake3>.tar
(install/tools_install.rs) and bound to its program by its guest_policy, as an
Alpine listing is. Any fault stops the run by name. With no mirror, none is listed."""

import sys
from pathlib import Path

from . import apk, pack_size, source_asset, source_content, source_lock, tarwalk
from .catalogue import guest_policy
from .digest import blake3
from .records import entry, release

NOTE = "{}; content of {} bytes pinned by BLAKE3; licence {}; installs only in this release's box"
PREFIX = "nonos-"


def one(recipe: dict, body: bytes, mirror: str, key: str, when_ms: int) -> tuple:
    """The listing of one bundled content and its lock row, or ValueError."""
    pack_size.check(body)
    # The device records the program by choose.rs's rule under the install's
    # own name, nonos-<package>, as for an Alpine market listing.
    name = PREFIX + recipe["package"]
    _, program = apk.chosen_by_bytes(tarwalk.files(body), name, recipe["program"])
    (app, rel_id, said), digest = source_asset.naming(recipe, name), blake3(body)
    rel = release(rel_id, digest, digest, f"http://{mirror}/linux/{digest}.tar", ["x86_64-linux"],
                  ["ForeignExec"], NOTE.format(said, len(body), recipe["licence"]), "nonos.operator.linux", when_ms)
    rel["ext"] = {"guest_policy": guest_policy(recipe, program, bytes.fromhex(key), app, rel_id).hex()}
    listed = entry(app, digest, recipe["name"], "Linux", key, recipe["text"], [rel], recipe.get("logo_hash", ""))
    return listed, source_asset.row(recipe, body, program)


def source_entries(recipes: list, site: tuple, pinned: Path, marks: dict, key: str, when_ms: int,
                   update_lock: bool) -> list:
    """`site` is (mirror name:port, package-out directory, --sources
    directory). The entries, or ValueError naming the recipe that failed."""
    mirror, out, where = site
    if not mirror:
        for r in recipes:
            print(f"  skip {PREFIX}{r['package']}: no NONOS package mirror to fetch it from", file=sys.stderr)
        return []
    bodies, found, built = {}, [], {}
    for r in recipes:
        try:
            body = source_content.bundled(where, r["package"])
            listed, built[r["package"]] = one(r, body, mirror, key, when_ms)
            found.append(listed)
        except (ValueError, OSError) as why:
            raise ValueError(f"{PREFIX}{r['package']}: {why}") from None
        bodies[found[-1]["capsule_id"]] = body  # the content's BLAKE3, as `one` names it
    if not update_lock:
        source_lock.held(source_lock.read(pinned), built, marks)
    elif built or pinned.exists():
        source_lock.write(pinned, built, marks)
    for digest, body in bodies.items():
        out.mkdir(parents=True, exist_ok=True)
        (out / f"{digest}.tar").write_bytes(body)
    return found
