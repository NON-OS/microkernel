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
"""The local test chain, 31337, belongs to the development catalogue alone.

The test catalogue prices every listing on it, so a development image's
market reads reach the devnet node and nothing real. A catalogue under
nonos-data/market is the one a release seals, and naming 31337 there would
point a release's market at a chain anyone can run on their own machine."""

import json
import subprocess

DEVNET = 31337
RELEASE = "nonos-data/market/"


def chains(doc):
    return [(e.get("listing_id", "?"), (e.get("token") or {}).get("chain_id")) for e in doc.get("entries", [])]


def dev_chains(doc):
    return [f"{lid}: priced on chain {c}, not the local test chain {DEVNET}" for lid, c in chains(doc) if c != DEVNET]


def release_chains(path, doc):
    return [f"{path}: {lid} is priced on the local test chain {DEVNET}" for lid, c in chains(doc) if c == DEVNET]


def release_catalogues(root):
    listed = subprocess.run(["git", "-C", str(root), "ls-files", "-z", "--", RELEASE],
                            capture_output=True, check=True).stdout.decode().split("\0")
    out = []
    for path in sorted(p for p in listed if p.endswith(".json")):
        doc = json.loads((root / path).read_text())
        if isinstance(doc, dict) and "entries" in doc:
            out += release_chains(path, doc)
    return out


def cases(doc, release):
    """(passed, should pass) pairs: the tree's two catalogues as they are,
    then a release listing moved onto 31337 and a dev listing moved off it."""
    e = release["entries"][0]
    moved = dict(release, entries=[dict(e, token=dict(e.get("token") or {}, chain_id=DEVNET))])
    d = doc["entries"][0]
    away = dict(doc, entries=[dict(d, token=dict(d["token"], chain_id=1))])
    return [(not dev_chains(doc), True), (not release_chains(RELEASE, release), True),
            (not release_chains(RELEASE, moved), False), (not dev_chains(away), False)]
