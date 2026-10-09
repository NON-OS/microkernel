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
"""The NØNOS mark as atlas faces: the official Ø (assets/brand/nonos-icon-cyan.svg)
as coverage at each screen class's size (code 0xD8), and its glow, a blur of
the same shape with room around it (code 0x04). Needs rsvg-convert and Pillow."""

import io
import random
import subprocess

from PIL import Image, ImageFilter

SVG = "../brand/nonos-icon-cyan.svg"


def raster(px):
    png = subprocess.run(["rsvg-convert", "-h", str(px), SVG], check=True, capture_output=True).stdout
    return Image.open(io.BytesIO(png)).getchannel("A")


def cov(img, dither=False):
    """4-bit coverage; the glow is dithered so its fall-off shows no bands."""
    noise = random.Random(7)
    return [min(15, (v + (noise.randrange(16) if dither else 8)) >> 4) for v in img.tobytes()]


def mark_face(px):
    """A face whose glyphs are the mark and its glow, in atlas glyph tuples."""
    a = raster(px)
    pad = px // 2
    big = Image.new("L", (a.width + 2 * pad, a.height + 2 * pad))
    big.paste(a, (pad, pad))
    glow = big.filter(ImageFilter.GaussianBlur(px / 7))
    glow = glow.point(lambda v: min(255, v * 2))
    mark = (0xD8, 0, 0, a.width, a.height, a.width * 64, cov(a))
    halo = (0x04, -pad, -pad, glow.width, glow.height, 0, cov(glow, True))
    return 0, a.height, [mark, halo]
