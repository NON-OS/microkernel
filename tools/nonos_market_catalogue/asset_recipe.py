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
"""A source recipe whose program is upstream's own release binary, not a
build. In place of `commit` and `build` it names `asset`, the https URL of
that one file; `sha256`, its digest; and `sums`, upstream's checksum file
in the same release, which gives that digest (tools/nonos-source-asset
checks it there). With `family: debian` and `needs`, the Debian packages
that give its shared libraries, space separated, it is listed in the
Debian family as `linux.kali.nonos-<package>`, and its content pins those
packages (asset_needs.py). source_asset.py lists it."""

import re

from . import asset_needs

OWN = ("kind", "source", "asset", "sha256", "sums")
FAMILY = ("family", "needs")
BUILT = frozenset(("commit", "build"))
SHA256 = re.compile(r"[0-9a-f]{64}")
# `kali.nonos-` and the name fit a listing id (names.py LISTING).
LONGEST = 53


def is_asset(item: dict) -> bool:
    return "asset" in item


def shape(item: dict, own: tuple, keys: frozenset, needed: frozenset) -> tuple:
    """(its own keys, the keys it may have, those it must), from a built
    recipe's: an asset's in place of a commit and a build."""
    if not is_asset(item):
        return own, keys, needed
    mine = OWN + tuple(k for k in FAMILY if k in item)
    return mine, keys - BUILT | set(OWN + FAMILY), needed - BUILT | set(OWN)


def checked(item: dict, name: str, url) -> None:
    """ValueError unless the asset's fields hold; `url` is source_recipe's URL."""
    for key in ("asset", "sums"):
        if not url.fullmatch(item[key]) or "/." in item[key]:
            raise ValueError(f"{name}: its {key} is not an https URL")
    if item["asset"].rsplit("/", 1)[0] != item["sums"].rsplit("/", 1)[0] or item["asset"] == item["sums"]:
        raise ValueError(f"{name}: its sums are not another file of the asset's own release")
    if not SHA256.fullmatch(item["sha256"]):
        raise ValueError(f"{name}: its sha256 is not 64 lower-case hex digits")
    if ("family" in item) != ("needs" in item):
        raise ValueError(f"{name}: family and needs go together")
    if "family" in item and (item["family"] != "debian" or len(name) > LONGEST):
        raise ValueError(f"{name}: its family is not debian, or its name is past {LONGEST} for one")
    if "needs" in item:
        asset_needs.names(item["needs"])
