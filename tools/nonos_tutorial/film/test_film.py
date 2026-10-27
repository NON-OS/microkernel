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

"""The film's pure parts: the perspective coefficients, the easing and
keyframes, the timeline and narration parsers, the sound graph, the brand
SVG fill, and that CREDITS.txt lists every evidence string drawn.

  python3 -m unittest discover -s tools/nonos_tutorial/film -p 'test_*.py'"""

import sys
import tempfile
import unittest
import wave
from pathlib import Path

HERE = Path(__file__).resolve().parent
sys.path.insert(0, str(HERE))

import numpy as np  # noqa: E402
from PIL import Image  # noqa: E402

import audio  # noqa: E402
import ease  # noqa: E402
import geom  # noqa: E402
import raster  # noqa: E402
import timeline  # noqa: E402
from cards import drawn_strings, load_evidence  # noqa: E402

TIMELINES = sorted((HERE / "timelines").glob("*.json"))


class Perspective(unittest.TestCase):
    def test_corners_map_exactly(self):
        out_q = [(103.5, 40.0), (870.0, 95.0), (820.0, 610.0), (60.0, 560.0)]
        in_q = [(0, 0), (1920, 0), (1920, 1080), (0, 1080)]
        c = geom.perspective_coeffs(out_q, in_q)
        for (x, y), (X, Y) in zip(out_q, in_q):
            u, v = geom.apply_coeffs(c, x, y)
            self.assertAlmostEqual(u, X, places=6)
            self.assertAlmostEqual(v, Y, places=6)

    def test_identity_and_scale(self):
        sq = [(0, 0), (10, 0), (10, 10), (0, 10)]
        self.assertTrue(np.allclose(geom.perspective_coeffs(sq, sq), (1, 0, 0, 0, 1, 0, 0, 0), atol=1e-12))
        big = [(0, 0), (20, 0), (20, 20), (0, 20)]
        a, b, c, d, e, f, g, h = geom.perspective_coeffs(big, sq)
        self.assertAlmostEqual(a, 0.5)
        self.assertAlmostEqual(e, 0.5)
        self.assertAlmostEqual(g, 0.0)

    def test_pillow_agrees(self):
        """A marked texture pixel lands where the coefficients say."""
        tex = Image.new("RGB", (200, 100))
        tex.putpixel((150, 25), (255, 255, 255))
        quad = [(30, 20), (280, 10), (300, 190), (10, 170)]
        img, (x0, y0) = raster.warp(tex, quad, (320, 200), Image.NEAREST)
        arr = np.asarray(img).sum(-1)
        ys, xs = np.nonzero(arr)
        self.assertTrue(len(xs) > 0)
        out_q = [(x - x0, y - y0) for x, y in quad]
        c = geom.perspective_coeffs(out_q, [(0, 0), (200, 0), (200, 100), (0, 100)])
        u, v = geom.apply_coeffs(c, xs.mean() + 0.5, ys.mean() + 0.5)
        self.assertLess(abs(u - 150.5), 1.5)
        self.assertLess(abs(v - 25.5), 1.5)

    def test_camera_and_plane(self):
        cam = geom.Camera((0, 5, 20), (0, 5, 0), 30, 1920, 1080)
        (u, v), = cam.project([(0, 5, 0)])[0]
        self.assertAlmostEqual(u, 960)
        self.assertAlmostEqual(v, 540)
        pl = geom.Plane((0, 5, 0), 16, 16 / 9)
        q, z = cam.project(pl.corners())
        self.assertTrue(q[0][0] < q[1][0] and q[0][1] < q[3][1])
        m = pl.mirrored(pl.corners())
        self.assertTrue(np.allclose(m[:, 1], -pl.corners()[:, 1]))
        A, B, C = geom.inverse_depth_plane(q, z)
        for (x, y), zz in zip(q, z):
            self.assertAlmostEqual(A * x + B * y + C, 1 / zz, places=9)


class Easing(unittest.TestCase):
    def test_in_out_cubic(self):
        f = ease.in_out_cubic
        self.assertEqual(f(0), 0)
        self.assertEqual(f(1), 1)
        self.assertAlmostEqual(f(0.5), 0.5)
        xs = np.linspace(0, 1, 201)
        ys = [f(x) for x in xs]
        self.assertTrue(all(b >= a for a, b in zip(ys, ys[1:])))
        for x in xs:
            self.assertAlmostEqual(f(x) + f(1 - x), 1.0)
        # At rest at both ends: the first and last steps are tiny.
        self.assertLess(f(0.01), 1e-5)
        self.assertLess(1 - f(0.99), 1e-5)

    def test_every_ease_runs_zero_to_one(self):
        for name, fn in ease.EASES.items():
            self.assertAlmostEqual(fn(0), 0, msg=name)
            self.assertAlmostEqual(fn(1), 1, msg=name)

    def test_unknown_ease(self):
        with self.assertRaises(ValueError):
            ease.ease("linear-ish")

    def test_track_segments_rest_at_keys(self):
        tr = ease.Track([(0, 0.0), (1, 10.0), (2, 4.0)])
        self.assertEqual(tr(-1), 0.0)
        self.assertEqual(tr(1), 10.0)
        self.assertEqual(tr(5), 4.0)
        self.assertLess(abs(tr(1.001) - tr(0.999)), 1e-3)

    def test_camera_path_never_jumps(self):
        tr = ease.Track([(0, (0.0, 0.0, 30.0)), (2, (4.0, 1.0, 24.0)), (4, (6.0, 1.5, 20.0))],
                        "inOutCubic", "path")
        pts = np.array([tr(t) for t in np.linspace(0, 4, 121)])
        steps = np.linalg.norm(np.diff(pts, axis=0), axis=1)
        self.assertLess(steps[0], steps.max() * 0.05)
        self.assertLess(steps[-1], steps.max() * 0.05)
        self.assertLess(np.abs(np.diff(steps)).max(), steps.max() * 0.15)
        self.assertTrue(np.allclose(tr(2), (4.0, 1.0, 24.0), atol=0.6))


class Timeline(unittest.TestCase):
    def test_times(self):
        p = timeline.parse_time
        self.assertAlmostEqual(p("2bar", 128), 3.75)
        self.assertAlmostEqual(p("1b", 128), 0.46875)
        self.assertAlmostEqual(p("0:03.75", 128), 3.75)
        self.assertAlmostEqual(p("1:00:01", 128), 3601)
        self.assertAlmostEqual(p(1.5, 128), 1.5)
        self.assertAlmostEqual(p("2.5s", 128), 2.5)
        for bad in ("soon", "2 bars and a bit", True, ""):
            with self.assertRaises(timeline.TimelineError):
                p(bad, 128)

    def test_cut_on_the_bar(self):
        film = timeline.Film({"bpm": 128, "fps": 30, "shots": [
            {"id": "a", "length": "2bar"}, {"id": "b", "length": "1bar"}, {"id": "c", "length": "2b"}],
            "cuts": {"x": ["b", "a"], "full": ["a", "b", "c"]}})
        full = film.cut("full")
        self.assertEqual([s.start for s in full], [0.0, 3.75, 5.625])
        self.assertEqual(timeline.frame_of(3.75, 30), 113)
        shot, lt = timeline.shot_at(full, 4.0)
        self.assertEqual((shot.id, round(lt, 3)), ("b", 0.25))
        self.assertEqual([s.id for s in film.cut("x")], ["b", "a"])
        with self.assertRaises(timeline.TimelineError):
            film.cut("nope")

    def test_bad_films(self):
        with self.assertRaises(timeline.TimelineError):
            timeline.Film({"shots": [{"id": "a"}, {"id": "a"}]})
        with self.assertRaises(timeline.TimelineError):
            timeline.Film({"shots": [{"id": "a"}], "cuts": {"x": ["b"]}})
        with self.assertRaises(timeline.TimelineError):
            timeline.Film({"shots": [{"length": 2}]})

    def test_footage_aliases_and_utc(self):
        with tempfile.TemporaryDirectory() as d:
            (Path(d) / "000719.png").write_bytes(b"")
            (Path(d) / "times.txt").write_text("719 1793108125.739\n")
            fo = timeline.Footage(timeline.Footage.parse_arg([f"b434={d}"]))
            self.assertEqual(fo.path("b434:000719.png"), Path(d) / "000719.png")
            self.assertEqual(timeline.format_utc(fo.utc("b434:000719.png")), "2026-10-27 13:35:25 UTC")
            with self.assertRaises(timeline.TimelineError):
                fo.path("other:1.png")

    def test_any_folder_of_frames_plays_as_a_sequence(self):
        import objects
        self.assertEqual([objects.sequence_index(t, 5, 2) for t in (0, 0.49, 0.5, 1.99, 9)], [0, 0, 1, 3, 4])
        self.assertEqual(objects.sequence_index(3, 10, 0), 0)
        with tempfile.TemporaryDirectory() as d:
            for n in (10, 20, 30, 40):
                Image.new("RGB", (64, 36), (n, n, n)).save(Path(d) / f"{n:06d}.png")
            fo = timeline.Footage({"cap": d})
            src = objects.ScreenSource({"frames": "cap:", "from": "000020.png", "fps": 1}, fo)
            self.assertEqual(src.refs, ["cap:000020.png", "cap:000030.png", "cap:000040.png"])
            self.assertEqual(src.ref_at(1.5), "cap:000030.png")
            self.assertEqual(src.at(1.5)[0].getpixel((0, 0)), (30, 30, 30))
            self.assertEqual(src.at(1.5)[1].size, (32, 18))
            still = objects.ScreenSource("cap:000010.png", fo)
            self.assertEqual(still.ref_at(99), "cap:000010.png")

    def test_shipped_timelines_load(self):
        self.assertTrue(TIMELINES)
        for path in TIMELINES:
            film = timeline.Film.load(path)
            for name in film.cuts:
                shots = film.cut(name)
                for s in shots:
                    self.assertAlmostEqual((s.length / (60 / film.bpm)) % 1, 0, msg=f"{s.id} off the beat")


def _wav(path, seconds):
    with wave.open(str(path), "wb") as w:
        w.setnchannels(1)
        w.setsampwidth(2)
        w.setframerate(8000)
        w.writeframes(b"\0\0" * int(8000 * seconds))


class Narration(unittest.TestCase):
    def test_parse_sorts_and_reads_lengths(self):
        with tempfile.TemporaryDirectory() as d:
            _wav(Path(d) / "a.wav", 1.0)
            _wav(Path(d) / "b.wav", 0.5)
            out = timeline.parse_narration({"lines": [{"file": "b.wav", "at": "2bar"},
                                                      {"file": "a.wav", "at": "0:01.5"}]}, 128, d)
            self.assertEqual([x[0] for x in out], ["a.wav", "b.wav"])
            self.assertAlmostEqual(out[0][1], 1.5)
            self.assertAlmostEqual(out[1][1], 3.75)
            self.assertAlmostEqual(out[0][2], 1.0)

    def test_overlap_and_bad_lines_refused(self):
        with tempfile.TemporaryDirectory() as d:
            _wav(Path(d) / "a.wav", 3.0)
            _wav(Path(d) / "b.wav", 1.0)
            with self.assertRaises(timeline.TimelineError):
                timeline.parse_narration({"lines": [{"file": "a.wav", "at": 0}, {"file": "b.wav", "at": 2}]}, 128, d)
        for bad in ({}, {"lines": []}, {"lines": [{"file": "x.wav"}]}, {"lines": [{"file": "x", "at": -1}]}):
            with self.assertRaises(timeline.TimelineError):
                timeline.parse_narration(bad)


class Sound(unittest.TestCase):
    def test_silent(self):
        self.assertEqual(audio.audio_args(15.0), ([], None, []))
        cmd = audio.encode_cmd("o.mp4", 30, (1920, 1080), 15.0)
        for want in ("libx264", "18", "yuv420p", "+faststart"):
            self.assertIn(want, cmd)

    def test_music_ducked_under_narration_and_faded(self):
        inputs, graph, maps = audio.audio_args(15.0, "bed.mp3", [("01.wav", 3.75, 2.0), ("02.wav", 7.5, 2.0)], "n")
        self.assertEqual(inputs, ["-i", "bed.mp3", "-i", "n/01.wav", "-i", "n/02.wav"])
        self.assertIn("afade=t=out:st=13.000:d=2.000", graph)
        self.assertIn("adelay=3750|3750", graph)
        self.assertIn("adelay=7500|7500", graph)
        self.assertIn("sidechaincompress", graph)
        self.assertIn("[aout]", maps)


class BrandMarks(unittest.TestCase):
    def test_nonzero_and_evenodd(self):
        ring = "M0 0 L10 0 L10 10 L0 10 Z M3 3 L3 7 L7 7 L7 3 Z"   # inner wound the other way
        same = "M0 0 L10 0 L10 10 L0 10 Z M3 3 L7 3 L7 7 L3 7 Z"   # inner wound the same way
        def centre(d, rule):
            return np.asarray(raster.svg_mask(d, 10, (100, 100), ss=1, rule=rule))[50, 50]
        self.assertEqual(centre(ring, "nonzero"), 0)
        self.assertEqual(centre(same, "nonzero"), 255)
        self.assertEqual(centre(same, "evenodd"), 0)

    def test_logo_parses(self):
        import brand
        (w, h), paths = raster.svg_paths(brand.LOGO)
        self.assertEqual((w, h), (1428.0, 500.0))
        self.assertTrue(all(fill.lower() != "none" for fill, _, _ in paths))


class Honesty(unittest.TestCase):
    """Every string an evidence card, a box label or the backdrop draws is
    listed in CREDITS.txt, and comes from evidence.json."""

    def setUp(self):
        self.credits = (HERE / "CREDITS.txt").read_text()
        self.evidence = load_evidence()

    def _drawn(self, spec):
        out = []
        for o in spec.get("objects", []):
            if o["kind"] == "card":
                out += [(o["evidence"], s) for s in drawn_strings(self.evidence[o["evidence"]], o.get("lines", "all"))]
            for ov in o.get("overlays", []):
                lab = ov.get("label")
                if lab:
                    text = lab.get("text", self.evidence[lab["evidence"]]["heading"])
                    out.append((lab["evidence"], text))
                    for word in text.split():
                        self.assertTrue(any(word in ln["text"] for ln in self.evidence[lab["evidence"]]["lines"]),
                                        f"label word {word!r} is not in its evidence")
        back = spec.get("stage", {}).get("backdrop")
        if back:
            out += [(back["evidence"], ln["text"]) for ln in self.evidence[back["evidence"]]["lines"]]
        return out

    def test_every_drawn_string_is_credited(self):
        for path in TIMELINES:
            film = timeline.Film.load(path)
            for shot in film.shots.values():
                specs = [shot.spec] + ([shot.spec["after"]] if shot.spec.get("after") else [])
                for spec in specs:
                    for ev, s in self._drawn(spec):
                        self.assertIn(s, self.credits, f"{path.name} {shot.id}: {ev} draws {s!r}, not in CREDITS.txt")

    def test_every_evidence_line_is_credited_with_a_source(self):
        for key, ev in self.evidence.items():
            for ln in ev["lines"]:
                self.assertIn(ln["text"], self.credits, f"{key}: {ln['text']!r}")
                self.assertTrue(ln.get("source"), f"{key}: {ln['text']!r} has no source")

    def test_slugs_name_credited_images(self):
        for path in TIMELINES:
            for shot in timeline.Film.load(path).shots.values():
                slug = shot.spec.get("hud", {}).get("slug", "")
                for word in slug.replace("·", " ").split():
                    if any(c.isdigit() for c in word) and len(word) >= 9:
                        self.assertIn(word, self.credits, f"{shot.id}: slug names {word}")


if __name__ == "__main__":
    unittest.main()
