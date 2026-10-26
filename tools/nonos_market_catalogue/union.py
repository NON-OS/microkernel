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

"""A catalogue with another's entries added: what a development image lists,
the release catalogue and the development test listings beside it, all
under the copy's throwaway operator key. A release seal never adds one."""

import json

# The test catalogue names no key: each copy makes its own and signs with it.
PLACEHOLDER = "OPERATOR"


def union(release, extra, operator_hex):
    """`release` with every entry of `extra` added after its own, each with
    the operator key in place of the placeholder and no logo. An id in both
    is refused: one listing id must mean one thing to the market."""
    have = {e["listing_id"] for e in release["entries"]}
    for e in extra["entries"]:
        if e["listing_id"] in have:
            raise SystemExit(f"catalogue: {e['listing_id']} is a release listing and an added one")
        added = dict(e, logo_hash=e.get("logo_hash", ""))
        if added.get("publisher_pubkey") == PLACEHOLDER:
            added["publisher_pubkey"] = operator_hex
        release["entries"].append(added)
        have.add(added["listing_id"])
    return release


def add(path, extra_path, operator_hex):
    """The catalogue at `path` rewritten with `extra_path`'s entries added."""
    with open(path) as f:
        release = json.load(f)
    with open(extra_path) as f:
        extra = json.load(f)
    with open(path, "w") as f:
        json.dump(union(release, extra, operator_hex), f, indent=2)
