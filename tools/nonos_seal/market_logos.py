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

"""The logos phase 1 keeps beside the index: the rasters a developer
committed, which the market capsule embeds and the index signs by BLAKE3."""

import json
import os
import subprocess

from .market_inputs import LOGOS, committed

INDEX_JSON = "nonos-data/market/index.json"


def agree():
    """SystemExit unless every logo_hash the signed index names is the BLAKE3
    of the raster committed for that listing: the market serves only bytes the
    index signs, so a mismatch would show no logo. A raster the index does
    not name yet (an index sealed before its logo) is served as none."""
    if not os.path.isfile(INDEX_JSON):
        return
    with open(INDEX_JSON) as f:
        named = {e["listing_id"]: e.get("logo_hash", "") for e in json.load(f).get("entries", [])}
    wrong = []
    for lid, want in named.items():
        at = os.path.join(LOGOS, lid + ".rgba")
        got = subprocess.run(["b3sum", "--no-names", at], capture_output=True, text=True).stdout.strip()
        if want and got != want:
            wrong.append(lid)
    if wrong:
        raise SystemExit(f"  the signed index and {LOGOS} disagree on: {' '.join(sorted(wrong))}")


def kept(*paths):
    """Whether `paths` and the logos are as committed at HEAD, the logos
    agreeing with the index (SystemExit when they do not)."""
    if not committed(*paths, LOGOS):
        return False
    agree()
    return True
