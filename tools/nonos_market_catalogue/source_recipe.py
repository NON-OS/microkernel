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
"""A recipe of kind `source`: an app no distribution packages, built from
its upstream at one commit (source_run.py lists it). It names the upstream
(`source`, an https git URL, and `commit`, the full 40-hex commit), the
nix recipe that builds it offline (`build`, under tools/nix/sources/), and
the program its listing sells (`program`, required). The rest is checked as
an Alpine recipe's is (recipes.py). Its listing is `linux.nonos-<package>`,
the namespace the device installs from the NONOS package mirror
(install/tools.rs), so the generator adds the prefix and a recipe may not."""

import re
from pathlib import Path

from .recipes import checked as common

KIND = "source"
OWN = ("kind", "source", "commit", "build")
KEYS = frozenset(OWN + ("package", "licence", "mask", "dests", "calls", "name", "text", "logo", "program"))
NEEDED = frozenset(OWN + ("package", "licence", "mask", "name", "text", "program"))
# install/tools.rs `tool`: lower case, digits, `-` and `.`; and `nonos-` and
# the name together fit a listing id (names.py LISTING).
NAME = re.compile(r"[a-z0-9][a-z0-9.-]{0,57}")
# A host by name, then a path: no user, port, query or fragment.
URL = re.compile(r"https://[a-z0-9][a-z0-9-]*(\.[a-z0-9][a-z0-9-]*)+(/[A-Za-z0-9._~-]+)+")
COMMIT = re.compile(r"[0-9a-f]{40}")
BUILD = re.compile(r"[a-z0-9][a-z0-9.-]{0,63}")


def is_source(item) -> bool:
    return isinstance(item, dict) and item.get("kind") == KIND


def checked(item: dict, root: Path) -> dict:
    """The recipe, checked for its upstream and build and then as
    recipes.checked checks one, or ValueError."""
    if not all(isinstance(v, str) for v in item.values()):
        raise ValueError("a recipe is an object of strings")
    name = item.get("package", "?")
    extra = set(item) - KEYS
    if extra or not NEEDED <= set(item):
        raise ValueError(f"{name}: keys {sorted(extra) or sorted(NEEDED - set(item))} for a source recipe")
    if not NAME.fullmatch(name):
        raise ValueError(f"{name}: not a name a NONOS mirror package may have")
    if not URL.fullmatch(item["source"]) or "/." in item["source"]:
        raise ValueError(f"{name}: its source is not an https git URL")
    if not COMMIT.fullmatch(item["commit"]):
        raise ValueError(f"{name}: its commit is not a full 40-hex commit")
    if not BUILD.fullmatch(item["build"]):
        raise ValueError(f"{name}: its build is not a nix recipe name")
    recipe = common({k: v for k, v in item.items() if k not in OWN}, root)
    return dict(recipe, **{k: item[k] for k in OWN})
