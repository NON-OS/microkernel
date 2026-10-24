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
"""The catalogue's recipes: one an app, in userland/capsule_market/
linux-catalogue.json. A recipe names the Alpine package, the licence the
index must still give it, the services its box grants, optionally a dests
file (one plan/33 item a line) and a measured set, and the store's text."""

import json
from pathlib import Path

from nonos_linux_pack.dests import dests_list
from nonos_linux_pack.policy import market_ready, mask_of

from .licences import redistributable

# No set measured yet: every Linux call may complete, and bucket 512, the
# numbers past 511 that no Linux call has, stays clear so it is market ready.
UNMEASURED = b"\xff" * 64 + b"\x00"
KEYS = frozenset(("package", "licence", "mask", "dests", "calls", "name", "text"))


def calls_of(item: dict) -> bytes:
    raw = item.get("calls", "")
    calls = bytes.fromhex(raw.removeprefix("calls=")) if raw else UNMEASURED
    if len(calls) != 65:
        raise ValueError(f"{item['package']}: calls is not 130 hex digits")
    return calls


def checked(item: dict, root: Path) -> dict:
    """The recipe, with its mask, dests list and calls decoded, or ValueError."""
    extra = set(item) - KEYS
    if extra or not {"package", "licence", "mask", "name", "text"} <= set(item):
        raise ValueError(f"{item.get('package', '?')}: keys {sorted(extra) or 'missing'}")
    if not redistributable(item["licence"]):
        raise ValueError(f"{item['package']}: {item['licence']} is not on the allowlist")
    mask = mask_of(item["mask"])
    if mask & mask_of("FILES_USER"):
        raise ValueError(f"{item['package']}: FILES_USER is reserved (plan/32)")
    dests = None
    if item.get("dests"):
        lines = (root / item["dests"]).read_text().splitlines()
        dests = dests_list([x.strip() for x in lines if x.strip()])
        if not mask & 2:
            raise ValueError(f"{item['package']}: a dests list needs NET_ANON")
    calls = calls_of(item)
    if calls[-1] & 0xFE or not market_ready(calls, mask):
        raise ValueError(f"{item['package']}: its set is not market ready (plan/30)")
    return dict(item, mask=mask, dests=dests, calls=calls)


def load(path: Path, root: Path) -> list:
    """Every recipe, checked, each package once."""
    items = [checked(i, root) for i in json.loads(path.read_text())]
    names = [i["package"] for i in items]
    if len(set(names)) != len(names):
        raise ValueError("a package has two recipes")
    return items
