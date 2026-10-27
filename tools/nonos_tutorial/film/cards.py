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

"""Evidence cards: real output typeset on the dark ground.

A card is a dark glass plate: a cyan heading (`EVIDENCE · SHARED MAP ·
0d0cff02d4`), the UTC time to its right, where the run was, a rule, the
lines exactly as the machine printed them, and the evidence's path below.
Lines type on with a cursor; a refusal flashes the plate's edge back. The
texture is drawn once; each frame only reveals and lights it."""

import json
from pathlib import Path

import numpy as np
from PIL import Image, ImageDraw

import brand
from raster import font, svg_mask, svg_paths, svg_subpaths

EVIDENCE = Path(__file__).resolve().parent / "evidence.json"


def load_evidence(path=EVIDENCE):
    data = json.loads(Path(path).read_text())
    return {k: v for k, v in data.items() if not k.startswith("_")}


def drawn_strings(ev, lines="all"):
    """Every string a card of this evidence draws (for CREDITS checks)."""
    picked = ev["lines"] if lines == "all" else [ev["lines"][i] for i in lines]
    out = [s for s in (ev.get("heading"), ev.get("utc"), ev.get("where"), ev.get("path")) if s]
    for ln in picked:
        out.append(ln["text"])
        if "value" in ln:
            out.append(ln["value"])
    return out


def tracked(draw, xy, text, fnt, fill, tracking):
    """Text with extra letter spacing; returns the width drawn."""
    x, y = xy
    for ch in text:
        draw.text((x, y), ch, font=fnt, fill=fill)
        x += fnt.getlength(ch) + tracking
    return x - xy[0] - tracking


def tracked_width(text, fnt, tracking):
    return sum(fnt.getlength(c) for c in text) + tracking * max(0, len(text) - 1)


class CardTexture:
    """The card's layers at texture scale, and where each line sits."""

    def __init__(self, ev, lines="all", width=1600, scale=1.0, compact=False):
        s = scale
        self.ev = ev
        self.picked = ev["lines"] if lines == "all" else [ev["lines"][i] for i in lines]
        pad = int(60 * s)
        f_head = font(brand.MONO, 30 * s)
        f_sub = font(brand.MONO, 21 * s)
        f_line = font(brand.MONO, (30 if not compact else 34) * s)
        f_foot = font(brand.MONO, 19 * s)
        lh = int((54 if not compact else 58) * s)
        self.char_w = f_line.getlength("M")
        longest = max(f_line.getlength(ln["text"] + ("   " + ln["value"] if "value" in ln else ""))
                      for ln in self.picked)
        width = int(max(width * s, longest + 2 * pad + 40 * s))
        head_h = int(150 * s)
        foot_h = int(78 * s)
        height = head_h + lh * len(self.picked) + foot_h + int(16 * s)
        self.size = (width, height)
        self.pad, self.lh, self.head_h = pad, lh, head_h

        base = Image.new("RGBA", self.size, (0, 0, 0, 0))
        d = ImageDraw.Draw(base)
        r = int(10 * s)
        d.rounded_rectangle((0, 0, width - 1, height - 1), radius=r, fill=brand.CARD + (238,))
        # A faint lift at the top, as glass catches the light.
        lift = np.linspace(14, 0, int(height * 0.5)).astype(np.uint8)
        lift_img = Image.new("L", (width, height), 0)
        lift_img.paste(Image.fromarray(np.repeat(lift[:, None], width, 1)), (0, 0))
        glow = Image.new("RGBA", self.size, brand.CYAN + (0,))
        glow.putalpha(lift_img)
        base = Image.alpha_composite(base, Image.composite(glow, Image.new("RGBA", self.size), base.split()[3]))
        d = ImageDraw.Draw(base)
        y = int(48 * s)
        tracked(d, (pad, y), ev["heading"], f_head, brand.CYAN, 3 * s)
        if ev.get("utc"):
            uw = tracked_width(ev["utc"], f_sub, 2 * s)
            tracked(d, (width - pad - uw, y + int(6 * s)), ev["utc"], f_sub, brand.TEXT_3, 2 * s)
        if ev.get("where"):
            tracked(d, (pad, y + int(46 * s)), ev["where"], f_sub, brand.TEXT_3, 2 * s)
        ry = head_h - int(18 * s)
        d.line((pad, ry, width - pad, ry), fill=brand.RULE + (255,), width=max(1, int(2 * s)))
        d.line((pad, ry, pad + int(140 * s), ry), fill=brand.CYAN + (255,), width=max(1, int(2 * s)))
        fy = height - foot_h + int(22 * s)
        d.line((pad, fy - int(14 * s), width - pad, fy - int(14 * s)), fill=brand.RULE + (255,), width=1)
        if ev.get("path"):
            tracked(d, (pad, fy), ev["path"], f_foot, brand.TEXT_3, 1 * s)
        self.base = base

        lines_img = Image.new("RGBA", self.size, (0, 0, 0, 0))
        d = ImageDraw.Draw(lines_img)
        self.line_boxes = []
        for i, ln in enumerate(self.picked):
            ly = head_h + int(8 * s) + i * lh
            x = pad + int(22 * s)
            text = ln["text"]
            if text.startswith("P "):
                # The probe's marker in cyan; mono, so the rest sits where it would.
                d.text((x, ly), "P", font=f_line, fill=brand.CYAN + (255,))
                d.text((x + f_line.getlength("P"), ly), text[1:], font=f_line, fill=brand.TEXT + (255,))
            else:
                d.text((x, ly), text, font=f_line, fill=brand.TEXT + (255,))
            if "value" in ln:
                vw = f_line.getlength(ln["value"])
                d.text((width - pad - vw, ly), ln["value"], font=f_line, fill=brand.CYAN + (255,))
            self.line_boxes.append((x, ly, len(text), ln))
        self.lines = lines_img
        self.radius = r
        self.scale = s

    def frame(self, t, start, step, cps, flash_len=0.45):
        """The card at time t (seconds after the shot began): lines typed on
        from `start`, one every `step` seconds, `cps` characters a second.
        Returns (RGBA texture, edge flash 0..1)."""
        s = self.scale
        w, h = self.size
        mask = Image.new("L", self.size, 0)
        md = ImageDraw.Draw(mask)
        overlay = Image.new("RGBA", self.size, (0, 0, 0, 0))
        od = ImageDraw.Draw(overlay)
        flash = 0.0
        for i, (x, ly, n, ln) in enumerate(self.line_boxes):
            t0 = start + i * step
            if t < t0:
                break
            shown = min(n, int((t - t0) * cps) + 1)
            right = x + shown * self.char_w
            full = shown >= n
            md.rectangle((0, ly - int(4 * s), right if not full else w, ly + self.lh - int(6 * s)), fill=255)
            if not full:
                od.rectangle((right + 2 * s, ly + 3 * s, right + self.char_w * 0.9, ly + self.lh * 0.66),
                             fill=brand.CYAN + (230,))
            done = t0 + n / cps
            if ln.get("refused") and t >= done:
                k = max(0.0, 1.0 - (t - done) / flash_len)
                flash = max(flash, k)
                level = int(255 * (0.45 + 0.55 * k))
                od.rectangle((self.pad - 2 * s, ly + 2 * s, self.pad + 4 * s, ly + self.lh * 0.7),
                             fill=brand.CYAN + (level,))
            if ln.get("kept") and t >= done:
                od.rectangle((self.pad - 2 * s, ly + 2 * s, self.pad + 4 * s, ly + self.lh * 0.7),
                             fill=brand.TEAL + (255,))
        lines = self.lines.copy()
        lines.putalpha(Image.fromarray(np.minimum(np.asarray(lines.split()[3]), np.asarray(mask))))
        out = Image.alpha_composite(self.base, lines)
        out = Image.alpha_composite(out, overlay)
        if flash > 0:
            edge = Image.new("RGBA", self.size, (0, 0, 0, 0))
            ImageDraw.Draw(edge).rounded_rectangle((1, 1, w - 2, h - 2), radius=self.radius,
                                                  outline=brand.CYAN + (int(255 * flash),),
                                                  width=max(2, int(3 * s)))
            out = Image.alpha_composite(out, edge)
        else:
            edge = Image.new("RGBA", self.size, (0, 0, 0, 0))
            ImageDraw.Draw(edge).rounded_rectangle((1, 1, w - 2, h - 2), radius=self.radius,
                                                  outline=brand.RULE + (255,), width=max(1, int(2 * s)))
            out = Image.alpha_composite(out, edge)
        return out, flash


def label_texture(text, scale=2.0):
    """The tab an isolation box carries: its policy, in mono, on Ink.
    Returns the tab with its text, the empty tab, and the x where each
    character ends (so the text can type on)."""
    f = font(brand.MONO, 22 * scale)
    pad_x, pad_y = int(16 * scale), int(9 * scale)
    tw = tracked_width(text, f, 1.5 * scale)
    w, h = int(tw + 2 * pad_x), int(22 * scale * 1.3 + 2 * pad_y)
    panel = Image.new("RGBA", (w, h), brand.INK + (236,))
    ImageDraw.Draw(panel).rectangle((0, 0, w - 1, h - 1), outline=brand.CYAN + (255,),
                                    width=max(1, int(1.5 * scale)))
    full = panel.copy()
    tracked(ImageDraw.Draw(full), (pad_x, pad_y), text, f, brand.CYAN, 1.5 * scale)
    ends = [pad_x]
    for ch in text:
        ends.append(ends[-1] + f.getlength(ch) + 1.5 * scale)
    return full, panel, ends


def wordmark_texture(height=420, sub="0.9.3", tile=False):
    """The NØNOS wordmark from the brand's SVG (nonos-logo-teal.svg: white
    letters with the Ø, in the brand's own colours), without the tile
    unless asked, and the version under it, on a transparent plate."""
    (vw, vh), paths = svg_paths(brand.LOGO)
    keep = []
    for fill, rule, d in paths:
        xs = [x for s in svg_subpaths(d) for x, _ in s]
        if tile or min(xs) > 460:
            keep.append((fill, rule, d, min(xs), max(xs)))
    x_lo = min(k[3] for k in keep) - 30
    x_hi = max(k[4] for k in keep) + 30
    pad_top = 40.0
    scale = height / (vh - 2 * 130 + 2 * pad_top) if not tile else height / vh
    y_off = -(130 - pad_top) * scale if not tile else 0
    w = int((x_hi - x_lo) * scale)
    f = font(brand.MONO, int(height * 0.10))
    sub_h = int(height * 0.45) if sub else 0
    img = Image.new("RGBA", (w, height + sub_h), (0, 0, 0, 0))
    for fill, rule, d, lo, hi in keep:
        m = svg_mask(d, scale, (w, height), offset=(-x_lo * scale, y_off), ss=3, rule=rule)
        colour = brand.CYAN if fill.lower() == "#66ffff" else brand.TEXT
        layer = Image.new("RGBA", img.size, colour + (0,))
        full = Image.new("L", img.size, 0)
        full.paste(m, (0, 0))
        layer.putalpha(full)
        img = Image.alpha_composite(img, layer)
    if sub:
        d = ImageDraw.Draw(img)
        sw = tracked_width(sub, f, f.size * 0.7)
        tracked(d, ((w - sw) / 2, height + sub_h * 0.30), sub, f, brand.TEXT_3, f.size * 0.7)
    return img
