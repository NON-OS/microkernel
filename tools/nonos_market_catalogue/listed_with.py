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
"""A recipe's `with`: Alpine packages its install adds to the closure, in
nonos_linux_policy::with's block, which rides after the listed args (or the
policy) in the release's guest_policy and so under its signature. 1 to 8
names, each by the package rule (names.py), none twice and not the recipe's
own. The device resolves each through the signed index like any need; the
lock pins each one's version, checksum and size (lock.py)."""

from .listed_args import MAX_LEN
from .listed_args import listed as args_of
from .names import package

MAGIC = b"NLXWITH1"
MAX_NAMES = 8


def block(names: list, own: str) -> bytes:
    """The canonical block for `names`, or ValueError naming the rule."""
    if not isinstance(names, list) or not all(isinstance(n, str) for n in names):
        raise ValueError("with is a list of strings")
    if not 1 <= len(names) <= MAX_NAMES:
        raise ValueError(f"{len(names)} extras: a recipe adds 1 to {MAX_NAMES}")
    raw = [package(n).encode() for n in names]
    if len(set(names)) != len(names) or own in names:
        raise ValueError("an extra is named twice, or is the recipe's own package")
    return MAGIC + bytes([len(raw)]) + b"".join(bytes([len(n)]) + n for n in raw)


def listed(item: dict) -> bytes:
    """The recipe's block, or b"" when it adds none; ValueError when it and
    the args together pass the bound on what follows the policy."""
    out = block(item["with"], item["package"]) if "with" in item else b""
    if len(args_of(item)) + len(out) > MAX_LEN:
        raise ValueError(f"{item['package']}: its args and with pass {MAX_LEN} bytes")
    return out


def extras(recipes: list) -> list:
    """Every extra the recipes add that is no recipe's own package, once."""
    own = {r["package"] for r in recipes}
    return sorted({n for r in recipes for n in r.get("with", ()) if n not in own})


def in_index(known: dict, names: list) -> dict:
    """`known`, or ValueError naming each extra the signed index lacks."""
    gone = [n for n in names if n not in known]
    if gone:
        raise ValueError("the index lacks the extras " + ", ".join(gone))
    return known
