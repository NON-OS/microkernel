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

"""The film's cards, drawn from nothing: the title, one for each chapter,
and the end. Each is the brand's near-black under a vignette, a small label
in the mono face and the accent, a title in Geist SemiBold, a line under it
in Geist and the dimmed text colour, and an accent rule that draws itself
out. The words fade up; the card itself fades in and out at the joins."""

from . import brand
from .effects import q, text

# label, title, line: (size, y) on the 1080-line card.
TITLE_CARD = {"label": (34, 360), "title": (168, 410), "line": (46, 640)}
CHAPTER_CARD = {"label": (30, 400), "title": (96, 450), "line": (42, 600)}


def rule(y, width=360, grow=0.9):
    """The accent rule, centred, growing to `width` over `grow` seconds."""
    w = f"{width}*min(1,t/{grow})"
    return f"drawbox=x={q(f'(iw-{w})/2')}:y={y}:w={q(w)}:h=4:color={brand.ACCENT}:t=fill"


def words(path, font, size, colour, y, delay):
    """One centred line, fading up after `delay` seconds."""
    return text(path, font, size, colour, "(w-tw)/2", y,
                f":alpha={q(f'clip((t-{delay:g})/0.6,0,1)')}")


def chain(texts, layout):
    """The card's filters, on a colour source: vignette, rule, then each of
    label, title and line that the card has, in turn."""
    filters = ["format=yuv420p", "vignette=PI/4", rule(layout["title"][1] + layout["title"][0] + 40)]
    faces = {"label": (brand.MONO, brand.ACCENT), "title": (brand.TITLE, brand.TEXT),
             "line": (brand.BODY, brand.DIM)}
    for n, part in enumerate(("label", "title", "line")):
        if part in texts:
            font, colour = faces[part]
            size, y = layout[part]
            filters.append(words(texts[part], font, size, colour, y, 0.2 + 0.25 * n))
    return ",".join(filters)


def command(texts, seconds, out, layout=CHAPTER_CARD, encode=()):
    """ffmpeg drawing a card of `seconds` to `out`. `texts` maps label,
    title and line to the files holding their words."""
    source = f"color=c={brand.CARD}:s={brand.WIDTH}x{brand.HEIGHT}:r={brand.FPS}:d={seconds:g}"
    return ["ffmpeg", "-hide_banner", "-loglevel", "error", "-y", "-f", "lavfi", "-i", source,
            "-vf", chain(texts, layout), *encode, str(out)]
