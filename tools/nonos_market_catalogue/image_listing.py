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

"""An app image's listing (capsule_linux install/image/): the release a
market document carries for `linux.nonos-image-<package>`, from the entry
`nonos-linux-pack image` wrote and the app's recipe. The release pins the
image's measurement, fetched as its chunk list from the NONOS mirror, and
its guest policy names the program the entry's box names, by its BLAKE3,
in the recipe's box: the same policy rule as any listing. Unsigned: the
publisher signs it as any release (`marketplace-index sign-release`).

    python3 -m nonos_market_catalogue.image_listing --entry ENTRY.json \\
        --recipes userland/capsule_market/linux-catalogue.json \\
        --operator-pubkey HEX --serial N --out DOC.json

A recipe is looked up by the entry's package in each recipes file given,
the Alpine catalogue first, then userland/capsule_market/linux-images.json."""

import argparse
import json
import sys
from pathlib import Path

from nonos_linux_pack.dests import dests_hash
from nonos_linux_pack.policy import encode, listing_id, mask_of, release_id

from .recipes import checked
from .records import entry as record, release

ROOT = Path(__file__).resolve().parents[2]
NOTE = "an app image of {} packages, {} bytes, pinned by its chunk list's BLAKE3; licence {}; installs only in this release's box"
FORMAT = "nonos.app-image-entry.v1"


def policy(recipe: dict, image: dict, publisher: bytes) -> bytes:
    """The 256-byte policy and the listed arguments, for the box's program."""
    lid = listing_id(publisher, image["listing_id"])
    hashed = dests_hash(recipe["dests"]) if recipe["dests"] else bytes(32)
    program = bytes.fromhex(image["box"]["program_blake3"])
    return encode(program, recipe["mask"], lid, release_id(lid, image["release_id"]), hashed,
                  recipe["calls"], None) + recipe.get("args", b"")


def listing(item: dict, image: dict, key: str, when_ms: int) -> dict:
    """The entry for the image, or ValueError naming what does not hold."""
    if image.get("format") != FORMAT or "box" not in image:
        raise ValueError("not an image entry with a box (nonos-linux-pack image --program)")
    if item.get("with"):
        raise ValueError("an image carries its extras already; its recipe may not name any")
    recipe = checked(item, ROOT)
    if mask_of(" ".join(image["box"]["mask"])) != recipe["mask"]:
        raise ValueError(f"the image's box {image['box']['mask']} is not the recipe's mask {item['mask']!r}")
    rel = release(image["release_id"], image["measurement"], image["measurement"], image["package_url"],
                  ["x86_64-linux"], ["ForeignExec"],
                  NOTE.format(len(image["packages"]), image["image_size"], item["licence"]),
                  "nonos.operator.linux", when_ms)
    rel["ext"] = {"guest_policy": policy(recipe, image, bytes.fromhex(key)).hex()}
    return record(image["listing_id"], image["measurement"], recipe["name"], "Linux", key, recipe["text"], [rel],
                  item.get("logo_hash", ""))


def recipe_for(package: str, files: list) -> dict:
    for f in files:
        data = json.loads(Path(f).read_text())
        for item in data.get("apps", []):
            if item.get("package") == package and item.get("kind") is None:
                return item
    raise ValueError(f"no recipe for {package}")


def main(argv) -> int:
    ap = argparse.ArgumentParser(description=__doc__.splitlines()[0])
    ap.add_argument("--entry", required=True, type=Path)
    ap.add_argument("--recipes", action="append", default=[], type=Path)
    ap.add_argument("--operator-pubkey", required=True)
    ap.add_argument("--serial", required=True, type=int)
    ap.add_argument("--out", required=True, type=Path)
    a = ap.parse_args(argv)
    files = a.recipes or [ROOT / "userland/capsule_market/linux-catalogue.json",
                          ROOT / "userland/capsule_market/linux-images.json"]
    try:
        image = json.loads(a.entry.read_text())
        doc = {"schema_version": 4, "operator_id": "nonos.marketplace.v1", "published_at_ms": a.serial * 1000,
               "serial": a.serial,
               "entries": [listing(recipe_for(image["name"], files), image, a.operator_pubkey, a.serial * 1000)]}
    except (ValueError, KeyError, OSError) as why:
        print(f"image_listing: {why}", file=sys.stderr)
        return 1
    a.out.write_text(json.dumps(doc, indent=2) + "\n")
    return 0


if __name__ == "__main__":
    sys.exit(main(sys.argv[1:]))
