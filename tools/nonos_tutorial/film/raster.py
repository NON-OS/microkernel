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

"""Drawing primitives Pillow lacks: anti-aliased quads and lines (drawn
supersampled inside their own box), a texture warped onto a projected quad,
fast wide blurs, and the brand's SVG marks filled even-odd."""

import re
from functools import lru_cache

import numpy as np
from PIL import Image, ImageChops, ImageDraw, ImageFilter, ImageFont

from geom import bbox, perspective_coeffs


@lru_cache(maxsize=64)
def font(path, size):
    return ImageFont.truetype(str(path), int(size))


def aa_polygon(size, pts, ss=4, value=255):
    """An L mask of `size` with the polygon filled, edges anti-aliased."""
    w, h = size
    out = Image.new("L", size, 0)
    box = bbox(pts, w, h, pad=2)
    if box is None:
        return out
    x0, y0, x1, y1 = box
    big = Image.new("L", ((x1 - x0) * ss, (y1 - y0) * ss), 0)
    ImageDraw.Draw(big).polygon([((x - x0) * ss, (y - y0) * ss) for x, y in pts], fill=value)
    out.paste(big.reduce(ss), (x0, y0))
    return out


def aa_polygon_box(pts, box, ss=4, value=255):
    """The polygon's anti-aliased L mask over `box` (x0, y0, x1, y1) only."""
    x0, y0, x1, y1 = box
    big = Image.new("L", ((x1 - x0) * ss, (y1 - y0) * ss), 0)
    ImageDraw.Draw(big).polygon([((x - x0) * ss, (y - y0) * ss) for x, y in pts], fill=value)
    return big.reduce(ss)


def aa_segments(size, segments, width, ss=3):
    """An L mask of line segments [((x0, y0), (x1, y1), level 0..1), ...],
    `width` pixels wide, anti-aliased."""
    w, h = size
    out = Image.new("L", size, 0)
    pts = [p for s in segments for p in s[:2]]
    if not pts:
        return out
    box = bbox(pts, w, h, pad=int(width) + 3)
    if box is None:
        return out
    x0, y0, x1, y1 = box
    big = Image.new("L", ((x1 - x0) * ss, (y1 - y0) * ss), 0)
    d = ImageDraw.Draw(big)
    for a, b, level in segments:
        d.line([((a[0] - x0) * ss, (a[1] - y0) * ss), ((b[0] - x0) * ss, (b[1] - y0) * ss)],
               fill=int(255 * max(0.0, min(1.0, level))), width=max(1, int(round(width * ss))))
    out.paste(big.reduce(ss), (x0, y0))
    return out


def edge_segments(quad, levels, steps=24):
    """The closed outline of `quad` cut into short segments, each with a
    level from `levels(x, y)`, so a rim light can fade along an edge."""
    segs = []
    n = len(quad)
    for i in range(n):
        a, b = np.asarray(quad[i]), np.asarray(quad[(i + 1) % n])
        for k in range(steps):
            p, q = a + (b - a) * (k / steps), a + (b - a) * ((k + 1) / steps)
            m = (p + q) / 2
            segs.append(((float(p[0]), float(p[1])), (float(q[0]), float(q[1])), levels(*m)))
    return segs


def warp(texture, quad, size, resample=Image.BICUBIC):
    """`texture` put on the projected `quad` (TL, TR, BR, BL) of an image of
    `size`; returns (image of the quad's box, (x0, y0)) or (None, None)."""
    w, h = size
    box = bbox(quad, w, h, pad=2)
    if box is None:
        return None, None
    x0, y0, x1, y1 = box
    tw, th = texture.size
    out_q = [(x - x0, y - y0) for x, y in quad]
    coeffs = perspective_coeffs(out_q, [(0, 0), (tw, 0), (tw, th), (0, th)])
    img = texture.transform((x1 - x0, y1 - y0), Image.PERSPECTIVE, coeffs, resample)
    return img, (x0, y0)


def blur(img, radius, down=1):
    """A Gaussian blur, done on a `down`-times smaller copy when wide."""
    if radius <= 0:
        return img
    if down <= 1:
        return img.filter(ImageFilter.GaussianBlur(radius))
    w, h = img.size
    small = img.resize((max(1, w // down), max(1, h // down)), Image.BILINEAR)
    small = small.filter(ImageFilter.GaussianBlur(radius / down))
    return small.resize((w, h), Image.BILINEAR)


def solid(size, colour, mode="RGB"):
    return Image.new(mode, size, tuple(colour))


def tint(mask, colour, gain=1.0):
    """An RGB light of `colour` shaped by an L mask (for adding)."""
    m = np.asarray(mask, dtype=np.float32) * (gain / 255.0)
    c = np.asarray(colour, dtype=np.float32)
    return Image.fromarray(np.clip(m[..., None] * c, 0, 255).astype(np.uint8), "RGB")


def add_at(base, layer, xy=(0, 0)):
    """base += layer at xy, clipped (light adds)."""
    x, y = xy
    w, h = layer.size
    region = base.crop((x, y, x + w, y + h))
    base.paste(ImageChops.add(region, layer.convert(base.mode)), (x, y))


def over_at(base, layer, mask, xy=(0, 0)):
    """base = layer over base through mask, at xy."""
    base.paste(layer, xy, mask)


def scale_mask(mask, k):
    if k >= 0.999:
        return mask
    return mask.point(lambda p: int(p * max(0.0, k)))


# The brand's marks, from their SVGs: absolute M, L, H, V, C, Z only.

_TOKEN = re.compile(r"[MLHVCZ]|-?\d*\.?\d+(?:e-?\d+)?", re.I)


def svg_subpaths(d, steps=14):
    """A path's subpaths as lists of (x, y), curves flattened."""
    toks = _TOKEN.findall(d)
    i, cmd = 0, None
    paths, cur, pos = [], [], (0.0, 0.0)

    def num():
        nonlocal i
        v = float(toks[i])
        i += 1
        return v

    while i < len(toks):
        if toks[i].isalpha():
            cmd = toks[i]
            i += 1
            if cmd in "Zz":
                if cur:
                    paths.append(cur)
                cur = []
                continue
        if cmd == "M":
            if cur:
                paths.append(cur)
            pos = (num(), num())
            cur = [pos]
            cmd = "L"
        elif cmd == "L":
            pos = (num(), num())
            cur.append(pos)
        elif cmd == "H":
            pos = (num(), pos[1])
            cur.append(pos)
        elif cmd == "V":
            pos = (pos[0], num())
            cur.append(pos)
        elif cmd == "C":
            p1, p2, p3 = (num(), num()), (num(), num()), (num(), num())
            p0 = pos
            for k in range(1, steps + 1):
                t = k / steps
                a, b, c, e = (1 - t) ** 3, 3 * (1 - t) ** 2 * t, 3 * (1 - t) * t * t, t ** 3
                cur.append((a * p0[0] + b * p1[0] + c * p2[0] + e * p3[0],
                            a * p0[1] + b * p1[1] + c * p2[1] + e * p3[1]))
            pos = p3
        else:
            raise ValueError(f"path command {cmd!r} is not one the brand SVGs use")
    if cur:
        paths.append(cur)
    return paths


def svg_paths(path):
    """The viewBox size and [(fill, rule, d), ...] of an SVG's filled
    <path> elements; a path with no fill inherits the root's (the brand's
    files set fill="none" there), and a stroke alone is skipped."""
    with open(path, encoding="utf-8") as fh:
        text = fh.read()
    view = re.search(r'viewBox="([\d.\s-]+)"', text).group(1).split()
    root = re.search(r"<svg\b[^>]*>", text).group(0)
    root_fill = re.search(r'\sfill="([^"]*)"', root)
    out = []
    for m in re.finditer(r"<path\b([^>]*)/?>", text):
        attrs = m.group(1)
        d = re.search(r'\sd="([^"]*)"', attrs).group(1)
        fill = re.search(r'\sfill="([^"]*)"', attrs)
        fill = fill.group(1) if fill else (root_fill.group(1) if root_fill else "#000000")
        if fill.lower() == "none":
            continue
        rule = re.search(r'fill-rule="([^"]*)"', attrs)
        out.append((fill, rule.group(1) if rule else "nonzero", d))
    return (float(view[2]), float(view[3])), out


def _signed_area(pts):
    return sum(pts[i][0] * pts[(i + 1) % len(pts)][1] - pts[(i + 1) % len(pts)][0] * pts[i][1]
               for i in range(len(pts))) / 2


def svg_mask(d, scale, size, offset=(0, 0), ss=4, rule="nonzero"):
    """An L mask of one path at `scale` pixels per SVG unit. Nonzero adds
    each subpath's winding (its signed area's sign); even-odd counts."""
    w, h = size
    acc = np.zeros((h * ss, w * ss), np.int16)
    for sub in svg_subpaths(d):
        if len(sub) < 3:
            continue
        layer = Image.new("1", (w * ss, h * ss), 0)
        ImageDraw.Draw(layer).polygon(
            [((x * scale + offset[0]) * ss, (y * scale + offset[1]) * ss) for x, y in sub], fill=1)
        inside = np.asarray(layer, dtype=bool)
        acc += inside.astype(np.int16) * (1 if rule == "evenodd" or _signed_area(sub) >= 0 else -1)
    filled = (acc % 2 == 1) if rule == "evenodd" else (acc != 0)
    big = Image.fromarray((filled * 255).astype(np.uint8), "L")
    return big.reduce(ss) if ss > 1 else big
