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

"""A frame, from its shot.

A scene is rendered at 1.25 times the frame and brought down for clean
edges: the soft stage at half that size (background, floor, reflections,
blooms, dust, then one depth-of-field blur), the sharp plates over it far
to near, dust in front of the focus, and then the finished frame's grade
(vignette, fine grain) and its type. A shot can end in a second scene the
Ø's slash wipes to, a cyan blade along the mark's own angle."""

import math

import numpy as np
from PIL import Image, ImageChops

import brand
import hud
import objects
from cards import load_evidence
from ease import Track, in_out_cubic, window
from geom import Camera
from raster import blur
from stage import Stage
from timeline import format_utc, frame_of, shot_at

SCALE = 1.25


def _vec(v):
    return tuple(float(x) for x in v)


class CameraTrack:
    def __init__(self, spec):
        keys = spec.get("keys") or [[0, {"pos": [0, 6, 30], "target": [0, 6, 0], "fov": 30}]]
        fn = spec.get("ease", "inOutCubic")
        mode = spec.get("mode", "path")
        self.pos = Track([(k[0], _vec(k[1]["pos"])) for k in keys], fn, mode)
        self.target = Track([(k[0], _vec(k[1]["target"])) for k in keys], fn, mode)
        self.fov = Track([(k[0], float(k[1].get("fov", 30))) for k in keys], fn, mode)
        self.roll = Track([(k[0], float(k[1].get("roll", 0))) for k in keys], fn, mode)
        self.aperture = float(spec.get("aperture", 0.6))
        self.focus = spec.get("focus")

    def camera(self, t, w, h):
        return Camera(self.pos(t), self.target(t), self.fov(t), w, h, roll=self.roll(t))


class Scene:
    def __init__(self, spec, footage, evidence, rw, rh):
        self.spec = spec
        self.rw, self.rh = rw, rh
        self.stage = Stage(spec.get("stage", {}), evidence, rw, rh)
        self.objects = [objects.build(o, footage, evidence) for o in spec.get("objects", [])]
        self.by_id = {o.id: o for o in self.objects}
        self.cam = CameraTrack(spec.get("camera", {}))
        self.back_blur = float(spec.get("stage", {}).get("blur", 1.6))

    def focus_inv(self, cam, t):
        f = self.cam.focus
        if f is None:
            return 1.0 / max(1e-3, min(o.centre_depth(cam) for o in self.objects)) if self.objects else 0.03
        if isinstance(f, (int, float)):
            return 1.0 / float(f)
        if isinstance(f, str):
            return 1.0 / max(1e-3, self.by_id[f].centre_depth(cam))
        # {"on": id, "uv": [u, v]}, or "uv_keys": [[t, [u, v]], ...] to pull
        # focus along a plate as the camera glides over it.
        obj = self.by_id[f["on"]]
        if "uv_keys" in f:
            uv = Track([(k[0], _vec(k[1])) for k in f["uv_keys"]], "inOutCubic", "path")(t)
        else:
            uv = f.get("uv", [0.5, 0.5])
        p = obj.plane.point(uv[0], uv[1])
        return 1.0 / max(1e-3, cam.to_view([p])[0][2])

    def render(self, t):
        cam = self.cam.camera(t, self.rw, self.rh)
        lights = [o.light(cam, t) for o in self.objects]
        buf, refl = self.stage.background(cam, t, lights)
        for o in self.objects:
            o.back(cam, t, buf, refl)
        finv = self.focus_inv(cam, t)
        ap = self.cam.aperture
        buf += self.stage.dust(cam, t, finv, ap, lights, near=False)
        noise = np.random.default_rng(int(t * 1000) + 11).random(buf.shape[:2], dtype=np.float32)
        back = Image.fromarray(np.clip(buf + noise[..., None] - 0.5, 0, 255).astype(np.uint8), "RGB")
        if self.back_blur > 0:
            back = blur(back, self.back_blur)
        img = back.resize((self.rw, self.rh), Image.BILINEAR)
        for o in sorted(self.objects, key=lambda o: -o.centre_depth(cam)):
            o.draw(img, cam, t, finv, ap)
        near = self.stage.dust(cam, t, finv, ap, lights, near=True)
        if near.max() > 0.5:
            nimg = Image.fromarray(np.clip(near, 0, 255).astype(np.uint8), "RGB").resize(img.size, Image.BILINEAR)
            img = ImageChops.add(img, nimg)
        return img.resize((brand.W, brand.H), Image.LANCZOS)


class Grade:
    """Vignette and fine grain, the same for every shot."""

    def __init__(self, seed=5, frames=8):
        h, w = brand.H, brand.W
        y, x = np.mgrid[0:h, 0:w].astype(np.float32)
        r = np.sqrt(((x - w / 2) / (w / 2)) ** 2 + ((y - h / 2) / (h / 2)) ** 2) / math.sqrt(2)
        self.vig = (1 - 0.42 * np.clip(r, 0, 1) ** 2.2).astype(np.float32)
        rng = np.random.default_rng(seed)
        self.grain = [rng.normal(0, 1, (h, w)).astype(np.float16) for _ in range(frames)]

    def __call__(self, img, frame, grain=2.6, vignette=1.0):
        a = np.asarray(img, np.float32)
        if vignette:
            a *= (1 - (1 - self.vig) * vignette)[..., None]
        g = self.grain[frame % len(self.grain)].astype(np.float32)
        lum = a.mean(-1, keepdims=True)
        amp = grain * (0.55 + 0.45 * np.clip(lum / 90.0, 0, 1))
        a += g[..., None] * amp
        return Image.fromarray(np.clip(a + 0.5, 0, 255).astype(np.uint8), "RGB")


class Wipe:
    """The Ø's slash as a cut: a cyan blade crossing the frame at the
    mark's angle, the next scene behind it."""

    def __init__(self, angle=52.0):
        h, w = brand.H, brand.W
        a = math.radians(angle)
        nx, ny = math.sin(a), math.cos(a)
        y, x = np.mgrid[0:h, 0:w].astype(np.float32)
        self.s = (x - w / 2) * nx + (y - h / 2) * ny
        self.span = float(np.abs(self.s).max()) + 80

    def __call__(self, before, after, k):
        s0 = -self.span + 2 * self.span * k
        m = np.clip((s0 - self.s) / 3.0, 0, 1)[..., None]
        a = np.asarray(before, np.float32) * (1 - m) + np.asarray(after, np.float32) * m
        d = self.s - s0
        blade = np.exp(-(d / 2.2) ** 2) * 1.0 + np.exp(-(d / 34.0) ** 2) * 0.35 + \
            np.exp(-np.clip(d, 0, None) / 160.0) * (d > 0) * 0.06
        a += blade[..., None] * np.array(brand.CYAN, np.float32) * min(1.0, 4 * k * (1 - k) + 0.25)
        return Image.fromarray(np.clip(a, 0, 255).astype(np.uint8), "RGB")


class ShotRenderer:
    def __init__(self, shot, footage, evidence, film):
        spec = shot.spec
        self.shot = shot
        self.film = film
        rw, rh = int(brand.W * SCALE), int(brand.H * SCALE)
        self.scene = Scene(spec, footage, evidence, rw, rh)
        self.after = None
        if spec.get("after"):
            a = spec["after"]
            self.after = Scene(a, footage, evidence, rw, rh)
            w = a.get("wipe", {})
            self.wipe_at = [film.time(v) for v in w.get("at", [shot.length - 1.0, shot.length - 0.5])]
            self.wipe = Wipe(float(w.get("angle", 52.0)))
        h = spec.get("hud", {})
        self.hud = h
        self.utc = self._utc(h.get("utc"), footage, evidence)
        self.grain = float(spec.get("stage", {}).get("grain", 2.6))

    def _utc(self, ref, footage, evidence):
        """The timecode: "auto:<screen>" is the UTC time of the frame that
        screen shows (from times.txt, so it follows a sequence), an
        "evidence:<id>" card's time, or the text given."""
        self.utc_screen = None
        if not ref:
            return ""
        if ref.startswith("auto:"):
            obj = self.scene.by_id[ref[5:]]
            for r in obj.footage.refs:
                if footage.utc(r) is None:
                    raise ValueError(f"no times.txt time for {r}; give the UTC time in the timeline")
            self.utc_screen, self.footage_roots = obj, footage
            return format_utc(footage.utc(obj.src_ref))
        if ref.startswith("evidence:"):
            return evidence[ref[9:]]["utc"]
        return ref

    def render(self, t, frame, grade):
        film = self.film
        img = None
        if self.after is not None and t >= self.wipe_at[0]:
            after = self.after.render(t)
            if t >= self.wipe_at[1]:
                img = after
            else:
                k = in_out_cubic((t - self.wipe_at[0]) / (self.wipe_at[1] - self.wipe_at[0]))
                img = self.wipe(self.scene.render(t), after, k)
        if img is None:
            img = self.scene.render(t)
        img = grade(img, frame, self.grain)
        h = self.hud
        until = film.time(h.get("until", self.shot.length + 1))
        if t < until:
            utc = self.utc
            if self.utc_screen is not None:
                utc = format_utc(self.footage_roots.utc(self.utc_screen.footage.ref_at(t)))
            img = hud.slug(img, h.get("slug", ""), t)
            img = hud.timecode(img, utc, t)
        st = h.get("statement")
        if st:
            img = hud.statement(img, st["text"], t, film.time(st["in"]), film.time(st["out"]), st.get("y"))
        cp = h.get("caption")
        if cp:
            img = hud.caption(img, cp["text"], t, film.time(cp["in"]), film.time(cp["out"]), cp.get("y"))
        tg = h.get("tag")
        if tg:
            img = hud.tag(img, tg["text"], t, film.time(tg["in"]), film.time(tg["out"]), tg.get("y", 760))
        fo = self.shot.spec.get("fade_out")
        if fo:
            k = window(t, film.time(fo[0]), film.time(fo[1]))
            if k > 0:
                a = np.asarray(img, np.float32) * (1 - k) + np.array(brand.INK, np.float32) * k
                img = Image.fromarray(np.clip(a, 0, 255).astype(np.uint8), "RGB")
        return img


class FrameSource:
    """Renders any frame of a cut; one per worker process."""

    def __init__(self, film, cut, footage, evidence_path=None):
        self.film = film
        self.shots = film.cut(cut)
        self.footage = footage
        self.evidence = load_evidence(evidence_path) if evidence_path else load_evidence()
        self.grade = Grade()
        self.cache = {}
        total = sum(s.length for s in self.shots)
        self.n_frames = frame_of(total, film.fps)
        self.duration = total

    def frame(self, i):
        t = i / self.film.fps
        shot, lt = shot_at(self.shots, t)
        key = (shot.id, shot.start)
        if key not in self.cache:
            if len(self.cache) > 2:
                self.cache.pop(next(iter(self.cache)))
            self.cache[key] = ShotRenderer(shot, self.footage, self.evidence, self.film)
        return self.cache[key].render(lt, i, self.grade)
