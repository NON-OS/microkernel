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
"""Listings for the catalogue's recipes: Alpine packages fetched, hashed,
sold free, each carrying the guest_policy its box is."""

import sys

from nonos_linux_pack.dests import dests_hash
from nonos_linux_pack.policy import encode, listing_id, release_id

from . import alpine_auth, apk
from .digest import blake3
from .names import index_record
from .records import entry, release

# I-C5: the install role installs it only in the box this release names.
NOTE = "fetched and hashed at {} bytes; licence {}; installs only in this release's box"


def guest_policy(recipe: dict, program: bytes, publisher: bytes, app: str, rel: str) -> bytes:
    """The 256 bytes, ids by marketplace_abi's rule under `publisher`."""
    lid = listing_id(publisher, app)
    hashed = dests_hash(recipe["dests"]) if recipe["dests"] else bytes(32)
    return encode(bytes.fromhex(blake3(program)), recipe["mask"], lid, release_id(lid, rel), hashed,
                  recipe["calls"])


def one(recipe: dict, rec: dict, raw: bytes, key: str, when_ms: int) -> dict:
    name = rec["P"]
    if rec.get("L", "") != recipe.get("index_licence", recipe["licence"]):
        raise ValueError(f"the index now says {rec.get('L')!r}, the recipe {recipe['licence']!r}")
    files = alpine_auth.package_files(raw, alpine_auth.checksum(rec.get("C")))
    found = apk.program(files, name, recipe.get("program", ""))
    if found is None:
        raise ValueError("the package has no program under usr/bin")
    app, rel_id = f"linux.{name}", f"{name}@{rec['V']}"
    policy = guest_policy(recipe, found[1], bytes.fromhex(key), app, rel_id)
    digest = blake3(raw)
    rel = release(rel_id, digest, digest, f"{rec['base']}/{name}-{rec['V']}.apk", ["x86_64-linux"],
                  ["ForeignExec"], NOTE.format(len(raw), recipe["licence"]),
                  "nonos.operator.linux", when_ms)
    rel["ext"] = {"guest_policy": policy.hex()}
    return entry(app, digest, recipe["name"], "Linux", key, recipe["text"], [rel], recipe.get("logo_hash", ""))


def catalogue_entries(recipes: list, table: dict, get, key: str, when_ms: int) -> list:
    """An entry per recipe whose package the index has; `get(rec)` returns its
    bytes. One missing or failing a check is left out, said, never unready."""
    if len(bytes.fromhex(key)) != 32:  # ValueError when not hex either
        raise ValueError("the publisher key is not 32 bytes of hex")
    out = []
    for recipe in recipes:
        rec = table.get(recipe["package"])
        if rec is None:
            print(f"  skip {recipe['package']}: not in the index", file=sys.stderr)
            continue
        try:
            rec = index_record(rec)
            out.append(one(recipe, rec, get(rec), key, when_ms))
        except (ValueError, TypeError, OSError) as why:
            print(f"  skip {recipe['package']}: {why}", file=sys.stderr)
    return out
