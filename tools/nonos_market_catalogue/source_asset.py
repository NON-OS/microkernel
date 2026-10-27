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
"""How a source listing is named and what the sources lock pins of it, for
both variants (asset_recipe.py). A built one is named by its commit. An
asset's release is named by the release directory its URL is in, and its
lock row pins the asset by URL, sha256 and BLAKE3 and the checksum file
that gave the sha256. Its program must be those very bytes, and its
content's Debian pins must hold every package the recipe needs."""

import hashlib

from . import asset_needs, tarwalk
from .asset_recipe import is_asset
from .digest import blake3


def naming(recipe: dict, name: str) -> tuple:
    """(listing id, release id, where its program came from)."""
    if not is_asset(recipe):
        return f"linux.{name}", f"{name}@{recipe['commit']}", f"built from {recipe['source']} at {recipe['commit']}"
    space, tag = "kali." if "family" in recipe else "", recipe["asset"].split("/")[-2]
    said = f"upstream's release binary {recipe['asset']}, sha256 {recipe['sha256']} as {recipe['sums']} gives it"
    return f"linux.{space}{name}", f"{name}@{tag}", said


def row(recipe: dict, body: bytes, program: bytes) -> dict:
    """What the sources lock pins of one content, or ValueError when an
    asset's program is not the published binary or its pins do not hold."""
    pinned = {"B": blake3(body), "S": str(len(body))}
    if not is_asset(recipe):
        return dict(pinned, commit=recipe["commit"])
    got = hashlib.sha256(program).hexdigest()
    if got != recipe["sha256"]:
        raise ValueError(f"its program has sha256 {got}, not the published asset's {recipe['sha256']}")
    pins = asset_needs.carried(tarwalk.files(body), recipe["package"])
    want = set(asset_needs.names(recipe["needs"])) if "needs" in recipe else set()
    if (pins is None) != (not want) or not want <= {p[0] for p in pins or ()}:
        raise ValueError(f"its content's Debian pins do not hold its needs {sorted(want)}")
    return dict(pinned, asset=recipe["asset"], sha256=got, A=blake3(program), sums=recipe["sums"])
