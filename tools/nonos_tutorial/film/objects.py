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

"""What stands on the stage: the real screen as a lit plate, evidence
cards, and the wordmark.

A screen is a thin slab: a dark bezel, sides that catch the key light, a
cyan rim light along its edge, a bloom where its light spills into the
air, and a rough reflection on the floor. Its pixels are the recording's,
untouched but for brightness while it powers on and a dimming outside an
isolation box. Overlays ride on the screen's plane, a little in front of
it: an isolation box drawn round a window as its policy types on, and a
marker on a line."""

import math

import numpy as np
from PIL import Image, ImageChops, ImageEnhance

import brand
from cards import CardTexture, label_texture, wordmark_texture
from ease import Track, out_cubic, window
from geom import Plane, bbox, inverse_depth_plane
from raster import aa_polygon_box, aa_segments, blur, warp


def _track(spec, default):
    if spec is None:
        return lambda t: default
    if isinstance(spec, (int, float)):
        return lambda t: float(spec)
    return Track([(k[0], float(k[1])) for k in spec], "inOutCubic")


def add_np(buf, img, xy, gain=1.0, colour=None):
    """buf (float32 RGB) += img at xy; an L image is lit with `colour`."""
    x0, y0 = xy
    a = np.asarray(img, np.float32)
    if a.ndim == 2:
        a = a[..., None] / 255.0 * np.asarray(colour, np.float32)
    h, w = a.shape[:2]
    H, W = buf.shape[:2]
    if x0 >= W or y0 >= H or x0 + w <= 0 or y0 + h <= 0:
        return
    sx0, sy0 = max(0, -x0), max(0, -y0)
    sx1, sy1 = min(w, W - x0), min(h, H - y0)
    buf[y0 + sy0:y0 + sy1, x0 + sx0:x0 + sx1] += a[sy0:sy1, sx0:sx1, :3] * gain


def add_light(img, mask, xy, colour, gain=1.0):
    """img (PIL RGB) += colour light shaped by an L mask, at xy."""
    m = np.asarray(mask, np.float32)[..., None] * (gain / 255.0)
    light = np.clip(m * np.asarray(colour, np.float32), 0, 255).astype(np.uint8)
    x, y = xy
    w, h = mask.size
    region = img.crop((x, y, x + w, y + h))
    img.paste(ImageChops.add(region, Image.fromarray(light, "RGB")), (x, y))


def rim_segments(plane, cam, grow, lift, level, steps=18):
    """The outline of the plane grown by `grow`, as projected segments with
    a level from level(lu, lv, s): lu, lv in -1..1 across the plate, s the
    fraction of the way round from the top left corner, clockwise."""
    corners = [(0, 0), (1, 0), (1, 1), (0, 1)]
    gu, gv = grow / plane.width, grow / plane.height
    segs = []
    for i in range(4):
        (ua, va), (ub, vb) = corners[i], corners[(i + 1) % 4]
        for k in range(steps):
            fa, fb = k / steps, (k + 1) / steps
            pa = (ua + (ub - ua) * fa, va + (vb - va) * fa)
            pb = (ua + (ub - ua) * fb, va + (vb - va) * fb)
            mu, mv = (pa[0] + pb[0]) / 2, (pa[1] + pb[1]) / 2
            s = (i + (k + 0.5) / steps) / 4
            lv_ = level(mu * 2 - 1, 1 - mv * 2, s)
            if lv_ <= 0.004:
                continue
            w = [plane.point(-gu + p[0] * (1 + 2 * gu), -gv + p[1] * (1 + 2 * gv), lift) for p in (pa, pb)]
            q, z = cam.project(w)
            if (z < 0.5).any():
                continue
            segs.append(((q[0][0], q[0][1]), (q[1][0], q[1][1]), lv_))
    return segs


def _coc(cam, depth, focus_inv, aperture):
    return float(np.max(aperture * cam.f * np.abs(1.0 / max(float(depth), 1e-3) - np.asarray(focus_inv))))


def sequence_index(t, count, fps, start=0):
    """Which frame of a sequence shows at shot time t: `fps` source frames a
    second from `start`, held on the last."""
    return max(0, min(count - 1, start + int(t * fps + 1e-6)))


class ScreenSource:
    """A screen's pixels: one still ("b434:000719.png"), or a folder of
    frames played as a sequence ({"frames": "b434:", "from": "002279.png",
    "to": "002399.png", "fps": 2}). Textures load as they are needed."""

    def __init__(self, src, footage):
        self.footage = footage
        if isinstance(src, str):
            self.refs, self.fps = [src], 0.0
        else:
            alias = src["frames"]
            files = footage.frames(alias)
            names = [f.name for f in files]
            lo = names.index(src["from"]) if src.get("from") else 0
            hi = names.index(src["to"]) + 1 if src.get("to") else len(names)
            prefix = alias if alias.endswith(":") or alias.endswith("/") else alias + "/"
            self.refs = [prefix + n for n in names[lo:hi]]
            self.fps = float(src.get("fps", 30))
        self._cache = {}

    def ref_at(self, t):
        return self.refs[sequence_index(t, len(self.refs), self.fps)]

    def at(self, t):
        ref = self.ref_at(t)
        if ref not in self._cache:
            if len(self._cache) > 6:
                self._cache.pop(next(iter(self._cache)))
            tex = Image.open(self.footage.path(ref)).convert("RGB")
            self._cache[ref] = (tex, tex.resize((tex.width // 2, tex.height // 2), Image.LANCZOS),
                                tex.resize((tex.width // 4, tex.height // 4), Image.BILINEAR))
        return self._cache[ref]


class Screen:
    kind = "screen"

    def __init__(self, spec, footage, evidence):
        self.spec = spec
        self.id = spec.get("id", "screen")
        self.footage = ScreenSource(spec["src"], footage)
        self.src_ref = self.footage.refs[0]
        tex, self.half, self.quarter = self.footage.at(0.0)
        self.tex = tex
        mean = np.asarray(self.quarter, np.float32).reshape(-1, 3).mean(0)
        hue = mean / max(1.0, mean.max())
        self.light_rgb = tuple(0.55 * np.asarray(brand.CYAN, np.float32) + 0.45 * hue * 255)
        self.plane = Plane(spec.get("centre", [0, 6, 0]), spec.get("width", 16), tex.width / tex.height,
                           spec.get("yaw", 0), spec.get("pitch", 0), spec.get("roll", 0))
        self.power = _track(spec.get("power"), 1.0)
        rim = spec.get("rim", {})
        self.rim_level = float(rim.get("level", 1.0))
        self.rim_trace = rim.get("trace")
        k = rim.get("key", [-0.6, 0.8])
        n = math.hypot(*k)
        self.key = (k[0] / n, k[1] / n)
        self.bezel = float(spec.get("bezel", 0.16))
        self.depth_u = float(spec.get("depth", 0.22))
        self.glow = float(spec.get("glow", 0.45))
        self.reflect = float(spec.get("reflect", 0.30))
        self.overlays = []
        for o in spec.get("overlays", []):
            if o["kind"] == "box":
                self.overlays.append(Box(o, self, evidence))
            elif o["kind"] == "highlight":
                self.overlays.append(Highlight(o, self))
            else:
                raise ValueError(f"screen {self.id}: unknown overlay {o['kind']!r}")
        self.dim_box = next((o for o in self.overlays if isinstance(o, Box) and o.dim > 0), None)
        self._dimmed = {}

    def _dimmed_at(self, t):
        """The frame dimmed outside its isolation box, built once a frame."""
        ref = self.footage.ref_at(t)
        if ref not in self._dimmed:
            tex = self.footage.at(t)[0]
            d = ImageEnhance.Brightness(tex).enhance(1.0 - self.dim_box.dim)
            x0, y0, x1, y1 = (int(v) for v in self.dim_box.rect)
            d.paste(tex.crop((x0, y0, x1, y1)), (x0, y0))
            self._dimmed = {ref: d}
        return self._dimmed[ref]

    def centre_depth(self, cam):
        return cam.to_view([self.plane.centre])[0][2]

    def light(self, cam, t):
        p = self.power(t)
        c = self.plane.centre
        n = self.plane.normal
        q, z = cam.project(self.plane.corners())
        span = float(np.linalg.norm(q[1] - q[0])) * 0.5
        (cu, cv), = cam.project([c])[0] * 0.5
        w = self.plane.width
        return {"pool": (c[0] + n[0] * w * 0.12, c[2] + n[2] * w * 0.12, w * 0.55, w * 0.32,
                         0.11 * p, self.light_rgb),
                "haze": (cu, cv, span * 0.95, 0.075 * p, self.light_rgb),
                "pos": tuple(c), "power": p}

    def _rim_level(self, t):
        trace = 1.0
        if self.rim_trace:
            trace = window(t, self.rim_trace[0], self.rim_trace[1], out_cubic)
        kx, ky = self.key

        def level(lu, lv, s):
            if min(s, 1 - s) > trace / 2 + 1e-6:
                return 0.0
            d = (lu * kx + lv * ky) / math.sqrt(2)
            return self.rim_level * max(0.0, min(1.0, 0.18 + 0.82 * (d + 1) / 2) ** 1.6)
        return level

    def back(self, cam, t, buf, refl):
        p = self.power(t)
        hh, hw = buf.shape[:2]
        q, z = cam.project(self.plane.corners())
        if (z < 0.5).any():
            return
        # Bloom: the screen's light in the air, and the rim's.
        _, half, quarter = self.footage.at(t)
        rad = max(8.0, float(np.linalg.norm(q[1] - q[0])) * 0.5 * 0.05)
        img, xy = warp(quarter, q * 0.5, (hw, hh), Image.BILINEAR)
        if img is not None:
            m = int(rad * 3)
            canvas = Image.new("RGB", (img.width + 2 * m, img.height + 2 * m))
            canvas.paste(img, (m, m))
            g = blur(canvas, rad, down=4)
            add_np(buf, g, (xy[0] - m, xy[1] - m), self.glow * (0.25 + 0.75 * p))
        rim = aa_segments((hw, hh), [((a[0] * .5, a[1] * .5), (b[0] * .5, b[1] * .5), l)
                                     for a, b, l in rim_segments(self.plane, cam, self.bezel, 0.0,
                                                                 self._rim_level(t))], 2.0)
        add_np(buf, blur(rim, 7, down=2), (0, 0), 1.1, brand.CYAN)
        if self.reflect <= 0:
            return
        # The floor's reflection: the plate mirrored in y = 0, rough, fading.
        mq, mz = cam.project(self.plane.mirrored(self.plane.corners()))
        if (mz < 0.5).any():
            return
        img, xy = warp(half, mq * 0.5, (hw, hh), Image.BILINEAR)
        if img is None:
            return
        m = 8
        canvas = Image.new("RGB", (img.width + 2 * m, img.height + 2 * m))
        canvas.paste(img, (m, m))
        mirror_segs = []
        for a, b, l in rim_segments(_Mirror(self.plane), cam, self.bezel, 0.0, self._rim_level(t)):
            mirror_segs.append(((a[0] * .5 - xy[0] + m, a[1] * .5 - xy[1] + m),
                                (b[0] * .5 - xy[0] + m, b[1] * .5 - xy[1] + m), l))
        if mirror_segs:
            seg_mask = aa_segments(canvas.size, mirror_segs, 2.0)
            canvas = ImageChops.add(canvas, Image.merge("RGB", [seg_mask.point(lambda v: v * 0.4),
                                                                seg_mask, seg_mask]))
        canvas = blur(canvas, 3.0)
        a = np.asarray(canvas, np.float32)
        ox, oy = xy[0] - m, xy[1] - m
        # Fade with distance below the reflection's top edge (the mirrored
        # bottom edge), as a rough floor blurs and dims what it reflects.
        bl, br = mq[3] * 0.5, mq[2] * 0.5
        cols = np.arange(a.shape[1], dtype=np.float32) + ox
        slope = (br[1] - bl[1]) / (br[0] - bl[0]) if abs(br[0] - bl[0]) > 1e-3 else 0.0
        top = bl[1] + (cols - bl[0]) * slope
        rows = np.arange(a.shape[0], dtype=np.float32)[:, None] + oy
        height = max(10.0, float(np.linalg.norm(mq[0] - mq[3])) * 0.5)
        fade = np.exp(-np.clip(rows - top[None, :], 0, None) / (0.32 * height))
        y0, x0 = max(0, oy), max(0, ox)
        y1, x1 = min(hh, oy + a.shape[0]), min(hw, ox + a.shape[1])
        if y1 <= y0 or x1 <= x0:
            return
        sub = a[y0 - oy:y1 - oy, x0 - ox:x1 - ox] * fade[y0 - oy:y1 - oy, x0 - ox:x1 - ox, None]
        buf[y0:y1, x0:x1] += sub * refl[y0:y1, x0:x1, None] * self.reflect * (0.2 + 0.8 * p)

    def draw(self, img, cam, t, focus_inv, aperture):
        """The slab, sharp; or, out of focus, drawn on a copy and blurred
        as one, so its bezel and rim soften with its pixels."""
        zs = cam.to_view(self.plane.corners())[:, 2]
        coc = min(28.0, _coc(cam, float(np.mean(zs)), focus_inv, aperture))
        if coc <= 1.5:
            self._draw(img, cam, t)
            return
        W, H = img.size
        q, z = cam.project(np.vstack([self.plane.corners(self.bezel), self.plane.corners(self.bezel, -self.depth_u)]))
        if (z < 0.5).any():
            return
        pad = int(coc * 2.5) + 4
        box = bbox(q, W, H, pad=pad)
        if box is None:
            return
        layer = img.copy()
        self._draw(layer, cam, t)
        region = blur(layer.crop(box), coc * 0.5, down=2 if coc > 8 else 1)
        sil = aa_polygon_box(q[:4], box)
        sil = blur(sil, coc * 0.5 + 2)
        img.paste(region, box[:2], sil)

    def _draw(self, img, cam, t):
        p = self.power(t)
        W, H = img.size
        pl = self.plane
        front = pl.corners(self.bezel)
        backc = pl.corners(self.bezel, -self.depth_u)
        qf, zf = cam.project(front)
        qb, _ = cam.project(backc)
        qs, zs = cam.project(pl.corners())
        if (zf < 0.5).any():
            return
        kx, ky = self.key
        # The slab's sides that face the camera, lit by the key from its side.
        normals = [pl.up, pl.right, -pl.up, -pl.right]
        facing_key = [ky, kx, -ky, -kx]
        for i in range(4):
            j = (i + 1) % 4
            mid = (front[i] + front[j] + backc[i] + backc[j]) / 4
            if np.dot(normals[i], cam.pos - mid) <= 0:
                continue
            quad = [qf[i], qf[j], qb[j], qb[i]]
            box = bbox(quad, W, H)
            if box is None:
                continue
            lit = 0.25 + 0.75 * max(0.0, facing_key[i])
            colour = tuple(int(c) for c in np.array([16, 20, 24]) * (0.6 + lit) + np.array(brand.TEAL) * 0.25 * lit)
            mask = aa_polygon_box(quad, box)
            img.paste(colour, box, mask)
        box = bbox(qf, W, H)
        if box is None:
            return
        img.paste((8, 9, 11), box, aa_polygon_box(qf, box))
        # The screen itself.
        tex = self.footage.at(t)[0]
        if self.dim_box is not None:
            k = self.dim_box.progress(t)
            if k > 0:
                tex = Image.blend(tex, self._dimmed_at(t), k)
        if p > 0.002:
            warped, xy = warp(tex, qs, (W, H))
            if warped is not None:
                if p < 0.999:
                    warped = ImageEnhance.Brightness(warped).enhance(p)
                sbox = (xy[0], xy[1], xy[0] + warped.width, xy[1] + warped.height)
                img.paste(warped, xy, aa_polygon_box(qs, sbox))
        for o in self.overlays:
            o.draw(img, cam, t)
        # The rim light along the plate's edge, and a hairline at the glass.
        segs = rim_segments(pl, cam, self.bezel, 0.0, self._rim_level(t))
        if segs:
            rim = aa_segments((W, H), segs, 1.7)
            add_light(img, rim, (0, 0), brand.CYAN, 1.0)
            add_light(img, blur(rim, 5, down=2), (0, 0), brand.CYAN, 0.9)
        glass = rim_segments(pl, cam, 0.0, 0.0, lambda lu, lv, s: 0.16 * (0.5 + 0.5 * lv))
        if glass:
            add_light(img, aa_segments((W, H), glass, 1.1), (0, 0), brand.TEXT, 1.0)


class _Mirror:
    """A plane's reflection in the floor, for drawing its rim mirrored."""

    def __init__(self, plane):
        self.p = plane
        self.width, self.height = plane.width, plane.height

    def point(self, u, v, lift=0.0):
        pt = np.array(self.p.point(u, v, lift))
        pt[1] = -pt[1]
        return pt


class Box:
    """An isolation box: a cyan outline drawn round a window of the screen,
    corner ticks, and the window's policy typed on a tab above it."""

    def __init__(self, spec, screen, evidence):
        self.screen = screen
        self.rect = [float(v) for v in spec["rect"]]
        self.lift = float(spec.get("lift", 0.3))
        self.span = spec.get("draw", [0.2, 0.8])
        self.dim = float(spec.get("dim", 0.0))
        lab = spec.get("label")
        self.label = None
        if lab:
            ev = evidence[lab["evidence"]]
            text = lab.get("text", ev["heading"])
            for word in text.split():
                if not any(word in ln["text"] for ln in ev["lines"]):
                    raise ValueError(f"box label word {word!r} is not in evidence {lab['evidence']}")
            self.label_text = text
            self.label, self.label_panel, self.label_x = label_texture(text, 2.0)
            self.label_span = lab.get("type", [self.span[1], self.span[1] + 0.6])
            self.label_scale = float(lab.get("scale", 1.0))

    def progress(self, t):
        return window(t, self.span[0], self.span[1], out_cubic)

    def _uv(self, x, y):
        return x / self.screen.tex.width, y / self.screen.tex.height

    def draw(self, img, cam, t):
        p = self.progress(t)
        if p <= 0:
            return
        pl = self.screen.plane
        x0, y0, x1, y1 = self.rect
        W, H = img.size
        pts = [pl.point(*self._uv(x, y), self.lift) for x, y in ((x0, y0), (x1, y0), (x1, y1), (x0, y1))]
        q, _ = cam.project(pts)
        # Trace from the top left corner both ways round, meeting bottom right.
        per = [np.linalg.norm(q[(i + 1) % 4] - q[i]) for i in range(4)]
        total = sum(per)
        segs = []
        for i in range(4):
            a, b = q[i], q[(i + 1) % 4]
            s0 = sum(per[:i]) / total
            s1 = s0 + per[i] / total
            n = 16
            for k in range(n):
                sa, sb = s0 + (s1 - s0) * k / n, s0 + (s1 - s0) * (k + 1) / n
                if min(sa, 1 - sb) > p / 2:
                    continue
                pa, pb = a + (b - a) * (k / n), a + (b - a) * ((k + 1) / n)
                segs.append(((pa[0], pa[1]), (pb[0], pb[1]), 1.0))
        if segs:
            m = aa_segments((W, H), segs, 2.2)
            add_light(img, m, (0, 0), brand.CYAN, 1.0)
            add_light(img, blur(m, 6, down=2), (0, 0), brand.CYAN, 1.1)
        # Corner ticks once the outline has closed.
        tick = window(t, self.span[1] - 0.05, self.span[1] + 0.25, out_cubic)
        if tick > 0:
            g, L = 14.0, 30.0
            ticks = []
            for (cx, cy), (dx, dy) in (((x0, y0), (-1, -1)), ((x1, y0), (1, -1)),
                                       ((x1, y1), (1, 1)), ((x0, y1), (-1, 1))):
                ox, oy = cx + dx * g, cy + dy * g
                for ex, ey in ((ox - dx * L, oy), (ox, oy - dy * L)):
                    a, b = cam.project([pl.point(*self._uv(ox, oy), self.lift),
                                        pl.point(*self._uv(ex, ey), self.lift)])[0]
                    ticks.append(((a[0], a[1]), (b[0], b[1]), tick))
            add_light(img, aa_segments((W, H), ticks, 2.6), (0, 0), brand.CYAN, 1.0)
        if self.label is None:
            return
        lt = window(t, self.label_span[0], self.label_span[1], lambda x: x)
        if lt <= 0:
            return
        n = len(self.label_text)
        shown = min(n, int(lt * n + 0.999))
        out = self.label_panel.copy()
        cut = int(math.ceil(self.label_x[shown]))
        if shown >= n:
            out = self.label.copy()
        elif cut > 0:
            out.paste(self.label.crop((0, 0, cut, out.height)), (0, 0))
        body = window(t, self.label_span[0] - 0.12, self.label_span[0] + 0.1, out_cubic)
        if body < 0.999:
            out.putalpha(out.split()[3].point(lambda v: int(v * body)))
        lw = out.width / 2.0 * self.label_scale
        lh = out.height / 2.0 * self.label_scale
        gap = 12.0
        quad_pts = [(x0, y0 - gap - lh), (x0 + lw, y0 - gap - lh), (x0 + lw, y0 - gap), (x0, y0 - gap)]
        qq, _ = cam.project([pl.point(*self._uv(x, y), self.lift) for x, y in quad_pts])
        warped, xy = warp(out, qq, (W, H))
        if warped is not None:
            img.paste(warped.convert("RGB"), xy, warped.split()[3])


class Highlight:
    """A marker on one line of the screen: a cyan bar and a faint band."""

    def __init__(self, spec, screen):
        self.screen = screen
        self.rect = [float(v) for v in spec["rect"]]
        self.t_in = float(spec.get("in", 0.5))
        self.t_out = float(spec.get("out", 99.0))
        self.lift = float(spec.get("lift", 0.08))

    def draw(self, img, cam, t):
        k = min(window(t, self.t_in, self.t_in + 0.35, out_cubic), 1 - window(t, self.t_out - 0.3, self.t_out))
        if k <= 0:
            return
        pl = self.screen.plane
        tw, th = self.screen.tex.size
        x0, y0, x1, y1 = self.rect
        W, H = img.size
        grow = x0 + (x1 - x0) * out_cubic(min(1.0, (t - self.t_in) / 0.5))
        for (a0, b0, a1, b1), level in (((x0, y0, grow, y1), 0.10), ((x0 - 10, y0, x0 - 5, y1), 1.0)):
            pts = [pl.point(x / tw, y / th, self.lift) for x, y in ((a0, b0), (a1, b0), (a1, b1), (a0, b1))]
            q, _ = cam.project(pts)
            box = bbox(q, W, H)
            if box is None:
                continue
            add_light(img, aa_polygon_box(q, box), box[:2], brand.CYAN, level * k)


class Card:
    """An evidence card, standing on the stage as a lit glass plate."""

    kind = "card"

    def __init__(self, spec, footage, evidence):
        self.spec = spec
        self.id = spec.get("id", "card")
        self.ev_id = spec["evidence"]
        self.tex = CardTexture(evidence[self.ev_id], spec.get("lines", "all"),
                               width=spec.get("tex_width", 1600), scale=spec.get("tex_scale", 1.0),
                               compact=spec.get("compact", False))
        w, h = self.tex.size
        self.plane = Plane(spec.get("centre", [0, 6, 0]), spec.get("width", 12), w / h,
                           spec.get("yaw", 0), spec.get("pitch", 0), spec.get("roll", 0))
        ty = spec.get("type", {})
        self.type_start = float(ty.get("start", 0.0))
        self.type_step = float(ty.get("step", 0.25))
        self.cps = float(ty.get("cps", 90))
        self.appear = spec.get("appear", [0.0, 0.0])
        self.reflect = float(spec.get("reflect", 0.18))
        self.rim = float(spec.get("rim", 0.55))
        self._cache = (None, None)

    def centre_depth(self, cam):
        return cam.to_view([self.plane.centre])[0][2]

    def opacity(self, t):
        a0, a1 = self.appear
        return window(t, a0, a1, out_cubic) if a1 > a0 else (1.0 if t >= a0 else 0.0)

    def frame(self, t):
        key = round(t * 240)
        if self._cache[0] != key:
            self._cache = (key, self.tex.frame(t, self.type_start, self.type_step, self.cps))
        return self._cache[1]

    def light(self, cam, t):
        o = self.opacity(t)
        _, flash = self.frame(t)
        (cu, cv), = cam.project([self.plane.centre])[0] * 0.5
        q, _ = cam.project(self.plane.corners())
        span = float(np.linalg.norm(q[1] - q[0])) * 0.5
        c = self.plane.centre
        return {"pool": (c[0], c[2] + 1.0, self.plane.width * 0.5, self.plane.width * 0.3,
                         0.035 * o * (1 + flash), brand.CYAN),
                "haze": (cu, cv, span * 0.8, 0.03 * o * (1 + 2 * flash), brand.CYAN),
                "pos": tuple(c), "power": 0.4 * o}

    def back(self, cam, t, buf, refl):
        o = self.opacity(t)
        if o <= 0 or self.reflect <= 0:
            return
        hh, hw = buf.shape[:2]
        tex, flash = self.frame(t)
        mq, mz = cam.project(self.plane.mirrored(self.plane.corners()))
        if (mz < 0.5).any():
            return
        small = tex.resize((tex.width // 3, tex.height // 3), Image.BILINEAR)
        img, xy = warp(small, mq * 0.5, (hw, hh), Image.BILINEAR)
        if img is None:
            return
        m = 8
        canvas = Image.new("RGBA", (img.width + 2 * m, img.height + 2 * m))
        canvas.paste(img, (m, m))
        canvas = blur(canvas, 3.0)
        a = np.asarray(canvas, np.float32)
        rgb = a[..., :3] * (a[..., 3:4] / 255.0)
        ox, oy = xy[0] - m, xy[1] - m
        rows = np.arange(a.shape[0], dtype=np.float32)[:, None]
        fade = np.exp(-rows / (0.35 * a.shape[0]))
        y0, x0 = max(0, oy), max(0, ox)
        y1, x1 = min(hh, oy + a.shape[0]), min(hw, ox + a.shape[1])
        if y1 <= y0 or x1 <= x0:
            return
        sub = rgb[y0 - oy:y1 - oy, x0 - ox:x1 - ox] * fade[y0 - oy:y1 - oy, :, None][:, :x1 - x0]
        buf[y0:y1, x0:x1] += sub * refl[y0:y1, x0:x1, None] * self.reflect * o

    def draw(self, img, cam, t, focus_inv, aperture):
        o = self.opacity(t)
        if o <= 0:
            return
        W, H = img.size
        tex, flash = self.frame(t)
        q, z = cam.project(self.plane.corners())
        if (z < 0.5).any():
            return
        warped, xy = warp(tex, q, (W, H))
        if warped is None:
            return
        rgb = warped.convert("RGB")
        alpha = warped.split()[3]
        if o < 0.999:
            alpha = alpha.point(lambda v: int(v * o))
        A, B, C = inverse_depth_plane(q, z)
        if aperture > 0:
            gx = np.arange(warped.width, dtype=np.float32) + xy[0]
            gy = np.arange(warped.height, dtype=np.float32)[:, None] + xy[1]
            coc = aperture * cam.f * np.abs(A * gx + B * gy + C - focus_inv)
            if float(coc.max()) > 1.0:
                b1, b2 = blur(warped, 2.5), blur(warped, 7.0, down=2)
                k1 = np.clip((coc - 1.0) / 4.0, 0, 1)[..., None]
                k2 = np.clip((coc - 5.0) / 9.0, 0, 1)[..., None]
                s = np.asarray(warped, np.float32)
                mix = s * (1 - k1) + np.asarray(b1, np.float32) * k1
                mix = mix * (1 - k2) + np.asarray(b2, np.float32) * k2
                mixed = Image.fromarray(np.clip(mix, 0, 255).astype(np.uint8), "RGBA")
                rgb = mixed.convert("RGB")
                alpha = mixed.split()[3]
                if o < 0.999:
                    alpha = alpha.point(lambda v: int(v * o))
        img.paste(rgb, xy, alpha)
        level = self.rim * o * (1 + 1.6 * flash)
        segs = rim_segments(self.plane, cam, 0.0, 0.0,
                            lambda lu, lv, s: level * max(0.0, min(1.0, 0.3 + 0.7 * (lv + 1) / 2)))
        if segs:
            rim = aa_segments((W, H), segs, 1.5)
            add_light(img, rim, (0, 0), brand.CYAN, 0.9)
            add_light(img, blur(rim, 6, down=2), (0, 0), brand.CYAN, 0.8 + flash)


class Wordmark:
    """The NØNOS wordmark, standing on the stage like the screens."""

    kind = "wordmark"

    def __init__(self, spec, footage, evidence):
        self.spec = spec
        self.id = spec.get("id", "wordmark")
        self.tex = wordmark_texture(int(spec.get("tex_height", 360)), spec.get("sub", ""))
        w, h = self.tex.size
        self.plane = Plane(spec.get("centre", [0, 5, 0]), spec.get("width", 14), w / h,
                           spec.get("yaw", 0), spec.get("pitch", 0), spec.get("roll", 0))
        self.appear = spec.get("appear", [0.2, 1.0])
        self.reflect = float(spec.get("reflect", 0.22))

    def centre_depth(self, cam):
        return cam.to_view([self.plane.centre])[0][2]

    def opacity(self, t):
        return window(t, self.appear[0], self.appear[1], out_cubic)

    def light(self, cam, t):
        o = self.opacity(t)
        (cu, cv), = cam.project([self.plane.centre])[0] * 0.5
        q, _ = cam.project(self.plane.corners())
        span = float(np.linalg.norm(q[1] - q[0])) * 0.5
        c = self.plane.centre
        return {"pool": (c[0], c[2] + 1.0, self.plane.width * 0.5, self.plane.width * 0.25,
                         0.05 * o, brand.CYAN),
                "haze": (cu, cv, span * 0.9, 0.05 * o, brand.CYAN), "pos": tuple(c), "power": 0.5 * o}

    def back(self, cam, t, buf, refl):
        o = self.opacity(t)
        if o <= 0:
            return
        hh, hw = buf.shape[:2]
        mq, mz = cam.project(self.plane.mirrored(self.plane.corners()))
        small = self.tex.resize((self.tex.width // 2, self.tex.height // 2), Image.BILINEAR)
        img, xy = warp(small, mq * 0.5, (hw, hh), Image.BILINEAR)
        if img is None:
            return
        m = 8
        canvas = Image.new("RGBA", (img.width + 2 * m, img.height + 2 * m))
        canvas.paste(img, (m, m))
        canvas = blur(canvas, 5.0)
        a = np.asarray(canvas, np.float32)
        rgb = a[..., :3] * (a[..., 3:4] / 255.0)
        ox, oy = xy[0] - m, xy[1] - m
        rows = np.arange(a.shape[0], dtype=np.float32)[:, None]
        fade = np.exp(-rows / (0.28 * a.shape[0]))
        y0, x0 = max(0, oy), max(0, ox)
        y1, x1 = min(hh, oy + a.shape[0]), min(hw, ox + a.shape[1])
        if y1 > y0 and x1 > x0:
            sub = rgb[y0 - oy:y1 - oy, x0 - ox:x1 - ox] * fade[y0 - oy:y1 - oy, :, None][:, :x1 - x0]
            buf[y0:y1, x0:x1] += sub * refl[y0:y1, x0:x1, None] * self.reflect * o
        # Its glow in the air.
        q, _ = cam.project(self.plane.corners())
        img, xy = warp(small, q * 0.5, (hw, hh), Image.BILINEAR)
        if img is not None:
            m = 40
            canvas = Image.new("RGBA", (img.width + 2 * m, img.height + 2 * m))
            canvas.paste(img, (m, m))
            g = blur(canvas.split()[3], 18, down=2)
            add_np(buf, g, (xy[0] - m, xy[1] - m), 0.22 * o, brand.CYAN)

    def draw(self, img, cam, t, focus_inv, aperture):
        o = self.opacity(t)
        if o <= 0:
            return
        W, H = img.size
        q, z = cam.project(self.plane.corners())
        warped, xy = warp(self.tex, q, (W, H))
        if warped is None:
            return
        alpha = warped.split()[3]
        if o < 0.999:
            alpha = alpha.point(lambda v: int(v * o))
        img.paste(warped.convert("RGB"), xy, alpha)


KINDS = {"screen": Screen, "card": Card, "wordmark": Wordmark}


def build(spec, footage, evidence):
    try:
        cls = KINDS[spec["kind"]]
    except KeyError:
        raise ValueError(f"unknown object kind {spec.get('kind')!r}; one of {sorted(KINDS)}") from None
    return cls(spec, footage, evidence)
