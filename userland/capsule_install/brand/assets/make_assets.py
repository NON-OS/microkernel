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
"""The installer's brand assets, from the files the loader carries
(nonos-bootloader/assets): Geist cut to static Regular and Medium
TrueType (the toolkit's rasteriser reads static faces), JetBrains Mono, and
the official Ø (nonos-icon-cyan.svg) as 8-bit coverage with its glow, each
"NXM1", width and height as u16 LE, then the bytes. Needs fontTools, brotli,
Pillow and rsvg-convert; the outputs are committed."""

import argparse
import io
import shutil
import struct
import subprocess

from fontTools.ttLib import TTFont
from fontTools.varLib.instancer import instantiateVariableFont
from PIL import Image, ImageFilter

LOADER = "../../../../nonos-bootloader/assets"


def geist(src, weight, out):
    font = TTFont(src)
    instantiateVariableFont(font, {"wght": weight}, inplace=True)
    font.flavor = None
    font.save(out)


def mark(svg, px, out):
    png = subprocess.run(["rsvg-convert", "-h", str(px), svg], check=True, capture_output=True).stdout
    a = Image.open(io.BytesIO(png)).getchannel("A")
    pad = px // 2
    big = Image.new("L", (a.width + 2 * pad, a.height + 2 * pad))
    big.paste(a, (pad, pad))
    glow = big.filter(ImageFilter.GaussianBlur(px / 7)).point(lambda v: min(255, v * 2))
    for img, name in ((a, out), (glow, out.replace(".nxm", "-glow.nxm"))):
        with open(name, "wb") as f:
            f.write(b"NXM1" + struct.pack("<HH", img.width, img.height) + img.tobytes())


def main():
    ap = argparse.ArgumentParser(description=__doc__.split("\n")[0])
    ap.add_argument("--loader", default=LOADER)
    a = ap.parse_args()
    for weight, name in ((400, "Regular"), (500, "Medium")):
        geist(f"{a.loader}/fonts/GeistVariable.woff2", weight, f"Geist-{name}.ttf")
    shutil.copy(f"{a.loader}/fonts/JetBrainsMono-Regular.ttf", "JetBrainsMono-Regular.ttf")
    for px in (56, 96, 150):
        mark(f"{a.loader}/brand/nonos-icon-cyan.svg", px, f"mark-{px}.nxm")


if __name__ == "__main__":
    main()
