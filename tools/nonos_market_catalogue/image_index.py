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

"""The app images' listings added to the catalogue a seal signs: one entry
for each image entry under nonos-data/market/images/ (the pack tool's
`<pin>.json`), its release pinned to the image's measurement and its policy
naming the box's program (image_listing.py), published by the operator.
The seal's own release-signing step then signs them as it signs every
operator release; nothing here holds a key. A listing id already in the
catalogue is refused: one id means one thing to the market."""

import json
from pathlib import Path

from .image_listing import listing, recipe_for

RECIPES = ("userland/capsule_market/linux-catalogue.json", "userland/capsule_market/linux-images.json")


def entries(images: Path, operator_hex: str, when_ms: int, root: Path) -> list:
    """The listing of each image entry in `images`, in name order; ValueError
    naming the entry that does not make one."""
    out = []
    for path in sorted(images.glob("*.json")):
        try:
            image = json.loads(path.read_text())
            out.append(listing(recipe_for(image["name"], [root / r for r in RECIPES]), image, operator_hex, when_ms))
        except (ValueError, KeyError, OSError) as why:
            raise ValueError(f"{path.name}: {why}") from None
    return out


def add(catalogue: Path, images: Path, operator_hex: str, when_ms: int, root: Path) -> int:
    """The catalogue at `catalogue` rewritten with the images' listings; how many."""
    doc = json.loads(catalogue.read_text())
    have = {e["listing_id"] for e in doc["entries"]}
    added = entries(images, operator_hex, when_ms, root)
    for e in added:
        if e["listing_id"] in have:
            raise ValueError(f"{e['listing_id']} is listed twice")
        have.add(e["listing_id"])
    doc["entries"].extend(added)
    catalogue.write_text(json.dumps(doc, indent=2))
    return len(added)
