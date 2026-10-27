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

"""The stage: a dark volume the real screens are lit in.

An Ink ground under a near-black floor that holds a faint, rough
reflection; the screens' light spilled onto it; a haze around each light;
a far slab of the machine's own boot lines, out of focus; dust in the air
caught by the light. Everything here is soft and is drawn at half the
render size; only the screens and cards are drawn sharp."""

import math

import numpy as np
from PIL import Image, ImageDraw, ImageFilter

import brand
from raster import font, warp
from geom import Plane


def _noise(n, octaves, rng):
    """Tileable multi-octave value noise, n x n, 0..1."""
    acc = np.zeros((n, n), np.float32)
    amp, total = 1.0, 0.0
    for o in range(octaves):
        cells = 4 * 2 ** o
        g = rng.random((cells, cells)).astype(np.float32)
        img = Image.fromarray((g * 255).astype(np.uint8)).resize((n, n), Image.BICUBIC)
        acc += amp * np.asarray(img, np.float32) / 255.0
        total += amp
        amp *= 0.5
    acc /= total
    return (acc - acc.min()) / max(1e-6, acc.max() - acc.min())


def _disc(r):
    """A bokeh disc of radius r: flat, a soft edge, a faint brighter rim."""
    s = int(math.ceil(r)) + 2
    y, x = np.mgrid[-s:s + 1, -s:s + 1].astype(np.float32)
    d = np.sqrt(x * x + y * y)
    edge = np.clip(r + 0.5 - d, 0, 1)
    rim = np.exp(-((d - r * 0.85) ** 2) / max(0.5, (r * 0.12) ** 2)) * 0.25 if r > 3 else 0
    return (edge * (0.8 + rim)).astype(np.float32)


class Stage:
    def __init__(self, spec, evidence, rw, rh, seed=7):
        self.spec = spec or {}
        self.rw, self.rh = rw, rh
        self.hw, self.hh = rw // 2, rh // 2
        self.rng = np.random.default_rng(self.spec.get("seed", seed))
        self.ink = np.array(brand.INK, np.float32)
        self.floor_rgb = np.array(self.spec.get("floor_colour", (5, 6, 8)), np.float32)
        self.noise = _noise(256, 4, self.rng)
        self.discs = {r: _disc(r) for r in range(1, 25)}
        self._particles()
        self._backdrop(evidence)

    # Dust: points in a box, drifting slowly up and sideways.
    def _particles(self):
        p = self.spec.get("particles", {})
        n = int(p.get("count", 120))
        lo = np.array(p.get("low", [-34, 0.5, -34]), np.float32)
        hi = np.array(p.get("high", [34, 20, 24]), np.float32)
        self.p0 = lo + self.rng.random((n, 3)).astype(np.float32) * (hi - lo)
        self.pv = (self.rng.random((n, 3)).astype(np.float32) - 0.5) * np.array([0.25, 0.12, 0.2])
        self.pv[:, 1] += 0.06
        self.pphase = self.rng.random(n).astype(np.float32) * 6.28
        self.plevel = float(p.get("level", 0.55))
        self.pspan = hi - lo
        self.plo = lo

    def particles_at(self, t):
        p = self.p0 + self.pv * t
        p[:, 0] += 0.25 * np.sin(t * 0.4 + self.pphase)
        return self.plo + np.mod(p - self.plo, self.pspan)

    # The far slab of boot lines.
    def _backdrop(self, evidence):
        b = self.spec.get("backdrop")
        self.slab = None
        if not b:
            return
        lines = [ln["text"] for ln in evidence[b["evidence"]]["lines"]]
        f = font(brand.MONO, 34)
        lh = 64
        block = lines + [""]
        rows = block * 4
        tw = 2600
        tex = Image.new("L", (tw, lh * len(rows) + 40), 0)
        d = ImageDraw.Draw(tex)
        for i, text in enumerate(rows):
            d.text((60, 20 + i * lh), text, font=f, fill=255)
        self.slab_period = lh * len(block)
        self.slab_tex = tex.filter(ImageFilter.GaussianBlur(float(b.get("soft", 2.0))))
        self.slab = b
        win_h = int(tw / (b.get("width", 46) / b.get("height", 26)))
        self.slab_win = win_h
        ramp_v = np.minimum(1, np.minimum(np.linspace(0, 4, win_h), np.linspace(4, 0, win_h)))
        ramp_h = np.minimum(1, np.minimum(np.linspace(0, 3, tw), np.linspace(3, 0, tw)))
        self.slab_ramp = Image.fromarray((np.outer(ramp_v, ramp_h) * 255).astype(np.uint8))

    def _slab_layer(self, cam, t, out):
        b = self.slab
        tw = self.slab_tex.width
        off = int(t * b.get("scroll", 18)) % self.slab_period
        win = self.slab_tex.crop((0, off, tw, off + self.slab_win))
        win = Image.fromarray((np.asarray(win, np.float32) * np.asarray(self.slab_ramp, np.float32)
                               / 255.0).astype(np.uint8))
        plane = Plane(b.get("centre", [0, 10, -30]), b.get("width", 46), tw / self.slab_win,
                      yaw=b.get("yaw", 0), pitch=b.get("pitch", 0))
        q, _ = cam.project(plane.corners())
        img, xy = warp(win, q * 0.5, (self.hw, self.hh), Image.BILINEAR)
        if img is None:
            return
        level = float(b.get("level", 0.14))
        col = np.array(brand.rgb(b.get("colour", "teal")), np.float32)
        x0, y0 = xy
        a = np.asarray(img, np.float32)[..., None] / 255.0 * level
        out[y0:y0 + a.shape[0], x0:x0 + a.shape[1]] += a * col

    def background(self, cam, t, lights):
        """The half-size background as float32 RGB, and the floor's
        reflection strength (0..1) per pixel."""
        hw, hh = self.hw, self.hh
        out = np.empty((hh, hw, 3), np.float32)
        out[:] = self.ink
        rays = cam.rays(hw, hh)
        pos = cam.pos.astype(np.float32)
        dy = rays[..., 1]
        hit = dy < -1e-5
        tt = np.where(hit, -pos[1] / np.where(hit, dy, -1.0), 0.0).astype(np.float32)
        fx = pos[0] + tt * rays[..., 0]
        fz = pos[2] + tt * rays[..., 2]
        dist = tt * np.sqrt((rays ** 2).sum(-1))
        fog = np.where(hit, np.exp(-dist / float(self.spec.get("fog", 70.0))), 0.0).astype(np.float32)

        # The horizon glows faintly, as a volume does where it is deepest.
        v = np.arange(hh, dtype=np.float32)[:, None]
        far = cam.pos + cam.fwd * np.array([1, 0, 1]) * 1e4
        far[1] = 0
        (hu, hv), = cam.project([far])[0] * 0.5
        band = np.exp(-((v - hv) / (0.22 * hh)) ** 2)
        out += band[..., None] * np.array(brand.TEAL, np.float32) * float(self.spec.get("horizon", 0.05))

        if self.slab is not None:
            self._slab_layer(cam, t, out)

        floor = self.floor_rgb[None, None, :] * fog[..., None] + out * (1 - fog[..., None])
        pool = np.zeros((hh, hw), np.float32)
        pool_rgb = np.zeros((hh, hw, 3), np.float32)
        for li in lights:
            lx, lz, sx, sz, level, colour = li["pool"]
            g = np.exp(-(((fx - lx) / sx) ** 2 + ((fz - lz) / sz) ** 2)) * level
            pool += g
            pool_rgb += g[..., None] * np.asarray(colour, np.float32)
        floor += pool_rgb * fog[..., None] * float(self.spec.get("pool", 1.0))
        out = np.where(hit[..., None], floor, out)

        # Haze: each light's glow in the air around it.
        if not hasattr(self, "_grid"):
            self._grid = np.mgrid[0:hh, 0:hw].astype(np.float32)
        yy, xx = self._grid
        for li in lights:
            cu, cv, rad, level, colour = li["haze"]
            g = np.exp(-(((xx - cu) ** 2 + ((yy - cv) * 1.35) ** 2) / (rad * rad)))
            out += g[..., None] * np.asarray(colour, np.float32) * level * float(self.spec.get("haze", 1.0))

        # The key light, a soft shaft from above left.
        beam = float(self.spec.get("beam", 0.0))
        if beam:
            k = np.clip(1 - np.abs((xx - hw * 0.18) * 0.82 + (yy * 0.55) - hw * 0.30) / (hw * 0.30), 0, 1)
            out += (k ** 2 * np.exp(-yy / hh * 1.2))[..., None] * np.array(brand.CYAN, np.float32) * beam

        n = self.noise[(np.abs(fz * 3.0).astype(np.int32)) % 256, (np.abs(fx * 3.0).astype(np.int32)) % 256]
        refl = np.where(hit, fog * (0.55 + 0.9 * n), 0.0).astype(np.float32)
        return out, refl

    def dust(self, cam, t, focus_inv, aperture, lights, near):
        """Dust as float32 RGB at half size: the motes behind the focus
        (near=False) or in front of it (near=True), each a bokeh disc."""
        hw, hh = self.hw, self.hh
        out = np.zeros((hh, hw, 3), np.float32)
        pts = self.particles_at(t)
        uv, z = cam.project(pts)
        uv *= 0.5
        f = cam.f * 0.5
        col = np.array(self.spec.get("dust_colour", (110, 225, 232)), np.float32)
        for (u, v), zz, p in zip(uv, z, pts):
            if zz < 1.0:
                continue
            inv = 1.0 / zz
            if (inv > focus_inv) != near:
                continue
            r = min(24.0, max(1.0, abs(inv - focus_inv) * aperture * f * 0.9 + 0.8))
            light = 0.25
            for li in lights:
                lx, ly, lz = li["pos"]
                light += li["power"] * math.exp(-math.dist((lx, ly, lz), p) / 14.0)
            level = self.plevel * light * (2.2 / (r ** 1.15)) * 255 / 255
            disc = self.discs[int(round(r))]
            s = disc.shape[0] // 2
            x0, y0 = int(u) - s, int(v) - s
            x1, y1 = x0 + disc.shape[1], y0 + disc.shape[0]
            if x1 <= 0 or y1 <= 0 or x0 >= hw or y0 >= hh:
                continue
            dx0, dy0 = max(0, -x0), max(0, -y0)
            dx1, dy1 = disc.shape[1] - max(0, x1 - hw), disc.shape[0] - max(0, y1 - hh)
            out[y0 + dy0:y0 + dy1, x0 + dx0:x0 + dx1] += disc[dy0:dy1, dx0:dx1, None] * col * level
        return out
