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
"""One recipe, one app (recipes_file.py reads the file): the Alpine
package, the licence the index must still give it, the services its box
grants, optionally a dests file (one plan/33 item a line) and a measured
set, and the store's text. Every field is checked before any is used."""

import re
from pathlib import Path

from nonos_linux_pack.dests import dests_list
from nonos_linux_pack.policy import market_ready, mask_of

from .licences import redistributable
from .names import package

# No set measured yet: every Linux call may complete, and bucket 512, the
# numbers past 511 that no Linux call has, stays clear so it is market ready.
UNMEASURED = b"\xff" * 64 + b"\x00"
# As nonos-market-catalogue's NAMESPACES: linux.kali.* is read as Debian,
# linux.blackarch.* as pacman, qwen- and nonos- are the image's own tiers and tools.
NAMESPACES = ("kali.", "blackarch.", "qwen-", "nonos-")
KEYS = frozenset(("package", "licence", "mask", "dests", "calls", "name", "text",
                  "index_licence", "licence_source", "program"))
# A recipe's `program`: the one program of a package of many its listing
# sells, by its path in the package (install/market/choose.rs).
PROGRAM = re.compile(r"usr/bin/[a-z0-9][a-z0-9._+-]{0,63}")


def calls_of(item: dict) -> bytes:
    raw = item.get("calls", "")
    calls = bytes.fromhex(raw.removeprefix("calls=")) if raw else UNMEASURED  # ValueError if not hex
    if len(calls) != 65:
        raise ValueError(f"{item['package']}: calls is not 130 hex digits")
    return calls


def checked(item: dict, root: Path) -> dict:
    """The recipe, with its mask, dests list and calls decoded, or ValueError."""
    if not isinstance(item, dict) or not all(isinstance(v, str) for v in item.values()):
        raise ValueError("a recipe is an object of strings")
    extra = set(item) - KEYS
    if extra or not {"package", "licence", "mask", "name", "text"} <= set(item):
        raise ValueError(f"{item.get('package', '?')}: keys {sorted(extra) or 'missing'}")
    if package(item["package"]).startswith(NAMESPACES):
        raise ValueError(f"{item['package']}: inside a namespace another family owns")
    if not redistributable(item["licence"]):
        raise ValueError(f"{item['package']}: {item['licence']} is not on the allowlist")
    # The listing names the licence of what it ships. Where the index's field
    # says otherwise, the recipe says what the index says and cites upstream.
    said, source = item.get("index_licence"), item.get("licence_source")
    if (said or source) and not (said and source and source.startswith("https://") and said != item["licence"]):
        raise ValueError(f"{item['package']}: index_licence and licence_source go together, cite https, and differ")
    if "program" in item and not PROGRAM.fullmatch(item["program"]):
        raise ValueError(f"{item['package']}: its program is not a path under usr/bin")
    mask = mask_of(item["mask"])
    dests = None
    if item.get("dests"):
        at = (root / item["dests"]).resolve()
        if not at.is_relative_to(root.resolve()):
            raise ValueError(f"{item['package']}: its dests file is outside the tree")
        try:
            lines = at.read_text().splitlines()
        except OSError as why:  # UnicodeDecodeError is already a ValueError
            raise ValueError(f"{item['package']}: its dests file: {why}") from None
        dests = dests_list([x.strip() for x in lines if x.strip()])
        if not mask & 2:
            raise ValueError(f"{item['package']}: a dests list needs NET_ANON")
    calls = calls_of(item)
    # market_ready also refuses FILES_USER, which is reserved (plan/32).
    if calls[-1] & 0xFE or not market_ready(calls, mask):
        raise ValueError(f"{item['package']}: its set or services are not market ready (plan/30)")
    return dict(item, mask=mask, dests=dests, calls=calls)

