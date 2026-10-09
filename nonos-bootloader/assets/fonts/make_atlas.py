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
"""Pre-render the loader's type and mark into nonos.nxf (format in nxf.py),
which src/display/ink/atlas.rs embeds: Geist for text, JetBrains Mono for
labels, hashes and the marks Geist lacks, the brand's Ø (marks.py)."""

import argparse
import freetype  # freetype-py

from marks import mark_face
from nxf import write

EXTRA = {0xD8: "Ø", 0xB7: "·", 0x18: "↑", 0x19: "↓", 0x10: "→",
         0x07: "•", 0x01: "✓", 0x02: "×"}
CODES = [(c, chr(c)) for c in range(0x20, 0x7F)] + sorted(EXTRA.items())
# Per screen class (src/display/ink/style.rs): mono, caption, body, label,
# heading, display; then the mark. (font, weight, px) for the text faces.
SIZES = [(11, 12, 14, 16, 22, 30, 64), (13, 14, 17, 20, 28, 40, 96),
         (17, 19, 23, 27, 38, 54, 132), (24, 26, 32, 38, 54, 80, 190)]
STYLES = [("mono", 400), ("geist", 400), ("geist", 400), ("geist", 500), ("geist", 500),
          ("geist", 600)]


def render(path, wght, px, fallback):
    face, back = freetype.Face(path), freetype.Face(fallback)
    if wght:
        face.set_var_design_coords((wght,))
    face.set_pixel_sizes(0, px)
    back.set_pixel_sizes(0, px)
    ascent, line = face.size.ascender >> 6, face.size.height >> 6
    glyphs = []
    for code, ch in CODES:
        f = face if face.get_char_index(ch) else back
        f.load_char(ch, freetype.FT_LOAD_RENDER | freetype.FT_LOAD_TARGET_LIGHT)
        g, bm = f.glyph, f.glyph.bitmap
        cov = [bm.buffer[r * bm.pitch + c] >> 4 for r in range(bm.rows) for c in range(bm.width)]
        top = ascent - g.bitmap_top
        glyphs.append((code, g.bitmap_left, top, bm.width, bm.rows, g.advance.x, cov))
    return ascent, line, glyphs


def main():
    ap = argparse.ArgumentParser(description=__doc__.split("\n")[0])
    ap.add_argument("--geist", default="GeistVariable.woff2")
    ap.add_argument("--mono", default="JetBrainsMono-Regular.ttf")
    ap.add_argument("--out", default="nonos.nxf")
    a = ap.parse_args()
    faces = []
    for row in SIZES:
        for (font, wght), px in zip(STYLES, row):
            wght = wght if font == "geist" else 0
            faces.append((px, wght // 100, *render(getattr(a, font), wght, px, a.mono)))
        faces.append((row[-1], 0, *mark_face(row[-1])))
    write(a.out, faces)

if __name__ == "__main__":
    main()
