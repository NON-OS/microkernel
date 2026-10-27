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

"""The type system, drawn on the finished frame so it stays crisp.

- A mono slug top left (`LIVE · QEMU · 0.9.3`), typed on as a shot opens.
- The UTC time of what is on screen, top right.
- One big statement in Geist SemiBold, centred, its letters rising in.
- A small boxed caption low, its box drawing out before the words."""

import numpy as np
from PIL import Image, ImageDraw, ImageFilter

import brand
from cards import tracked, tracked_width
from ease import out_cubic, in_cubic, window
from raster import font

SLUG_SIZE = 19
STATEMENT_SIZE = 104
CAPTION_SIZE = 33


def _overlay(img, layer, xy=(0, 0)):
    base = img.convert("RGBA")
    base.alpha_composite(layer, xy)
    return base.convert("RGB")


def slug(img, text, t, colour=brand.TEXT_2, dot=True):
    """The slug, typed on over 0.4 s from the shot's start."""
    if not text or t < 0:
        return img
    f = font(brand.MONO, SLUG_SIZE)
    n = len(text)
    shown = text[:max(0, min(n, int(t / 0.4 * n) + 1))]
    layer = Image.new("RGBA", (brand.W // 2, 60), (0, 0, 0, 0))
    d = ImageDraw.Draw(layer)
    x = 0
    if dot:
        a = int(255 * (0.55 + 0.45 * (0.5 + 0.5 * np.cos(t * 2.4))))
        d.ellipse((0, 9, 9, 18), fill=brand.CYAN + (a,))
        x = 22
    tracked(d, (x, 2), shown, f, colour + (235,), 2.2)
    return _overlay(img, layer, (brand.MARGIN, brand.MARGIN - 8))


def timecode(img, text, t, colour=brand.TEXT_2):
    if not text or t < 0:
        return img
    f = font(brand.MONO, SLUG_SIZE)
    a = int(235 * window(t, 0.05, 0.35, out_cubic))
    w = tracked_width(text, f, 2.2)
    layer = Image.new("RGBA", (int(w) + 4, 40), (0, 0, 0, 0))
    tracked(ImageDraw.Draw(layer), (0, 2), text, f, colour + (a,), 2.2)
    return _overlay(img, layer, (int(brand.W - brand.MARGIN - w), brand.MARGIN - 8))


def statement(img, text, t, t_in, t_out, y=None, size=STATEMENT_SIZE):
    """The line rises in letter by letter (22 ms apart, 0.55 s each) and
    lifts away as it fades."""
    if t < t_in or t > t_out:
        return img
    f = font(brand.GEIST_SEMIBOLD, size)
    total = f.getlength(text)
    cx = brand.W / 2
    cy = brand.H / 2 if y is None else y
    asc, desc = f.getmetrics()
    x0 = cx - total / 2
    top = cy - (asc + desc) / 2
    out_k = window(t, t_out - 0.4, t_out, in_cubic)
    pad = 80
    layer = Image.new("RGBA", (int(total) + 2 * pad, asc + desc + 2 * pad), (0, 0, 0, 0))
    shadow = Image.new("L", layer.size, 0)
    d, ds = ImageDraw.Draw(layer), ImageDraw.Draw(shadow)
    for i, ch in enumerate(text):
        k = window(t, t_in + i * 0.022, t_in + i * 0.022 + 0.55, out_cubic)
        if k <= 0:
            continue
        a = k * (1 - out_k)
        dy = (1 - k) * 22 - out_k * 10
        x = f.getlength(text[:i])
        d.text((pad + x, pad + dy), ch, font=f, fill=brand.TEXT + (int(255 * a),))
        ds.text((pad + x, pad + dy + 4), ch, font=f, fill=int(200 * a))
    shadow = shadow.filter(ImageFilter.GaussianBlur(22))
    dark = Image.new("RGBA", layer.size, (0, 0, 0, 0))
    dark.putalpha(shadow)
    dark.alpha_composite(layer)
    return _overlay(img, dark, (int(x0 - pad), int(top - pad)))


def tag(img, text, t, t_in, t_out, y, colour=brand.TEXT_3, size=24, tracking=12):
    """A short mono line, centred and widely tracked (the version on the
    end card), fading in and out."""
    if t < t_in or t > t_out:
        return img
    f = font(brand.MONO, size)
    a = window(t, t_in, t_in + 0.6, out_cubic) * (1 - window(t, t_out - 0.3, t_out, in_cubic))
    w = tracked_width(text, f, tracking)
    layer = Image.new("RGBA", (int(w) + 8, size * 2), (0, 0, 0, 0))
    tracked(ImageDraw.Draw(layer), (0, 4), text, f, colour + (int(255 * a),), tracking)
    return _overlay(img, layer, (int(brand.W / 2 - w / 2), int(y)))


def caption(img, text, t, t_in, t_out, y=None):
    """A small boxed caption, low and centred: the box draws out from its
    left edge, then the words come up; it all fades at the end."""
    if t < t_in or t > t_out:
        return img
    f = font(brand.GEIST_MEDIUM, CAPTION_SIZE)
    tw = f.getlength(text)
    asc, desc = f.getmetrics()
    px, py = 26, 16
    bw, bh = int(tw + 2 * px + 10), int(asc + desc + 2 * py)
    k = window(t, t_in, t_in + 0.45, out_cubic)
    tk = window(t, t_in + 0.18, t_in + 0.55, out_cubic)
    fade = 1 - window(t, t_out - 0.3, t_out, in_cubic)
    layer = Image.new("RGBA", (bw + 4, bh + 4), (0, 0, 0, 0))
    d = ImageDraw.Draw(layer)
    w_now = max(4, int(bw * k))
    d.rectangle((0, 0, w_now, bh), fill=brand.INK + (int(200 * fade),))
    d.rectangle((0, 0, w_now, bh), outline=brand.TEAL + (int(230 * fade),), width=1)
    d.rectangle((0, 0, 3, bh), fill=brand.CYAN + (int(255 * fade),))
    d.text((px + 10, py - 2 + (1 - tk) * 8), text, font=f, fill=brand.TEXT + (int(255 * tk * fade),))
    if w_now < bw:
        # The words never show past the box while it is still drawing out.
        clip = Image.new("RGBA", layer.size, (0, 0, 0, 0))
        clip.paste(layer.crop((0, 0, w_now + 1, layer.height)), (0, 0))
        layer = clip
    yy = int(brand.H - 150 if y is None else y)
    return _overlay(img, layer, (int(brand.W / 2 - bw / 2), yy))
