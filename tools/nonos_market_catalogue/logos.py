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
"""A listing's logo (HANDOFF/catalogue/logos-design.md): the upstream file,
vendored byte for byte where its recipe's `logo` names it, and the raster
the device gets, made once by a maintainer and committed at
nonos-data/market/logos/linux.<package>.rgba. A recipe may name a logo only
where the record allows it. The lock pins G, the BLAKE3 of the source, and
R, the BLAKE3 of the raster, "" for neither; the listing signs R as its
logo_hash. Nothing here fetches, and the build never remakes a raster.

    python3 -m nonos_market_catalogue.logos --rasterise

run from the tree's root with PYTHONPATH=tools, is the maintainer's step:
it needs Pillow and rsvg-convert, writes every raster, and is followed by
--update-lock and review."""

import io
import subprocess
import sys
from pathlib import Path

from .digest import blake3

RECORD = "HANDOFF/catalogue/logos-licences.md"
RASTERS = "nonos-data/market/logos"
# The owner's ruling on the UNCLEAR rows, as the record writes it in the row.
RULING = 'owner 2026-10-25: "Use them anyway"'
# The store's tile, and the largest raster drawn at its own pixels.
TILE, OWN_MAX = 36, 48


def verdicts(text: str) -> dict:
    """{app: its row's Verdict cell}, from the record's first table."""
    rows = {}
    for line in text.split("## Table", 1)[-1].split("\n## ", 1)[0].splitlines():
        cells = [c.strip() for c in line.strip().strip("|").split("|")]
        if line.startswith("| ") and len(cells) == 5 and cells[0] not in ("App", "---"):
            rows[cells[0]] = cells[4]
    return rows


def may_show(verdict: str) -> bool:
    """CLEAR, or UNCLEAR with the owner's ruling written beside it."""
    return verdict.startswith("CLEAR") or (verdict.startswith("UNCLEAR") and RULING in verdict)


def allowed(root: Path, package: str) -> bool:
    try:
        rows = verdicts((root / RECORD).read_text())
    except OSError:
        return False
    return package in rows and may_show(rows[package])


def raster_of(root: Path, package: str) -> Path:
    return root / RASTERS / f"linux.{package}.rgba"


def digests(recipe: dict, root: Path) -> tuple:
    """(G, R) for a recipe, or ValueError. A recipe without a logo has no
    raster, so no listing carries bytes nobody vetted."""
    at = raster_of(root, recipe["package"])
    if not recipe.get("logo"):
        if at.exists():
            raise ValueError(f"{recipe['package']}: names no logo, yet {at} is there")
        return "", ""
    try:
        source, raster = (root / recipe["logo"]).read_bytes(), at.read_bytes()
    except OSError as why:
        raise ValueError(f"{recipe['package']}: its logo: {why}; run --rasterise") from None
    return blake3(source), blake3(raster)


def held(marks: dict, pins: dict) -> None:
    """ValueError naming every logo whose source or raster is not the one locked."""
    moved = [f"{n}: the logo source is {g or 'none'}, locked {pins[n]['G'] or 'none'}"
             for n, (g, _) in marks.items() if n in pins and pins[n]["G"] != g]
    moved += [f"{n}: the logo raster is {r or 'none'}, locked {pins[n]['R'] or 'none'}"
              for n, (_, r) in marks.items() if n in pins and pins[n]["R"] != r]
    if moved:
        raise ValueError("the lock does not hold; review, then --update-lock: " + "; ".join(moved))


def rasterise(source: Path) -> bytes:
    """Square RGBA: a small square image at its own pixels, anything else
    fitted into the tile and centred on a clear square."""
    from PIL import Image  # the maintainer's step only

    if source.suffix == ".svg":
        png = subprocess.run(["rsvg-convert", "-a", "-w", "288", "-h", "288", "-b", "none", str(source)],
                             stdout=subprocess.PIPE, check=True).stdout
        image = Image.open(io.BytesIO(png))
    else:
        image = Image.open(source)
    image = image.convert("RGBA")
    if image.width == image.height <= OWN_MAX:
        return image.tobytes()
    scale = TILE / max(image.size)
    size = (max(1, round(image.width * scale)), max(1, round(image.height * scale)))
    fitted = image.convert("RGBa").resize(size, Image.LANCZOS).convert("RGBA")
    out = Image.new("RGBA", (TILE, TILE), (0, 0, 0, 0))
    out.paste(fitted, ((TILE - size[0]) // 2, (TILE - size[1]) // 2))
    return out.tobytes()


def main(argv) -> int:
    from .recipes_file import load

    if argv != ["--rasterise"]:
        print(__doc__, file=sys.stderr)
        return 2
    root = Path(".")
    (root / RASTERS).mkdir(parents=True, exist_ok=True)
    for recipe in load(root / "userland/capsule_market/linux-catalogue.json", root):
        if recipe.get("logo"):
            raster_of(root, recipe["package"]).write_bytes(rasterise(root / recipe["logo"]))
            print(raster_of(root, recipe["package"]))
    return 0


if __name__ == "__main__":
    sys.exit(main(sys.argv[1:]))
