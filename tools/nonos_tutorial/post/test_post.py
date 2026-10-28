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

"""The cut's rules: an edit's times found from the recording's captions,
a zoom that stays 16:9 and inside the frame, chapters that cross-fade where
their marks say, cards in the desktop's own colours and faces, and an edit
of the tour that names only captions the tour has. Nothing here runs
ffmpeg."""

import contextlib
import io
import json
import re
import sys
import tempfile
import unittest
from pathlib import Path

HERE = Path(__file__).resolve().parent
sys.path.insert(0, str(HERE.parents[1]))

from nonos_tutorial.post import brand, cards, edit, effects, join  # noqa: E402

SAID = [(10.0, "This is NØNOS 0.9.3"), (40.0, "The boot menu. Esc holds"), (70.0, "Step 4, the mode")]


class Edit(unittest.TestCase):
    def test_a_time_is_seconds_a_clock_or_a_caption(self):
        self.assertEqual(edit.seconds(12, SAID), 12.0)
        self.assertEqual(edit.seconds("1:02:03.5", SAID), 3723.5)
        self.assertEqual(edit.seconds({"cue": "The boot menu"}, SAID), 40.0)
        self.assertEqual(edit.seconds({"cue": "Step 4", "shift": -1.5}, SAID), 68.5)

    def test_a_caption_the_recording_lacks_is_named(self):
        with self.assertRaisesRegex(ValueError, "Monero"):
            edit.seconds({"cue": "Monero"}, SAID)

    def test_an_effect_at_a_caption_is_placed_in_the_clip_as_played(self):
        clip = edit.resolve({"from": 40, "to": 80, "speed": 2,
                             "zoom": [{"at": {"cue": "Step 4"}, "for": 3, "box": [0, 0, 960, 540]}]}, SAID)
        self.assertEqual((clip["length"], clip["zoom"][0]["at"]), (20.0, 15.0))

    def test_a_clip_that_ends_first_is_refused(self):
        with self.assertRaises(ValueError):
            edit.resolve({"from": 50, "to": 40}, SAID)

    def test_an_optional_chapter_with_no_recorded_section_is_left_out(self):
        film = {"chapters": [{"title": "kept", "clips": [{"from": 1, "to": 2}]},
                             {"title": "chat", "optional": True,
                              "clips": [{"from": {"cue": "SimpleX"}, "to": 9}]}]}
        with contextlib.redirect_stdout(io.StringIO()):
            self.assertEqual([c["title"] for c in edit.chapters(film, SAID)], ["kept"])

    def test_a_chapter_that_is_not_optional_stops_the_cut(self):
        film = {"chapters": [{"title": "boot", "clips": [{"from": {"cue": "Nope"}, "to": 9}]}]}
        with self.assertRaises(ValueError):
            edit.chapters(film, SAID)

    def test_cues_are_read_as_the_drive_writes_them(self):
        with tempfile.TemporaryDirectory() as d:
            path = Path(d) / "cues.txt"
            path.write_text("0:00:10.0\tThis is NØNOS 0.9.3\n0:02:59.5\t\n", encoding="utf-8")
            self.assertEqual(edit.cues(path), [(10.0, "This is NØNOS 0.9.3"), (179.5, "")])


class Zoom(unittest.TestCase):
    def test_a_box_is_made_16_9_about_its_centre(self):
        x, y, w, h = effects.box169((356, 240, 760, 100))
        self.assertAlmostEqual(w / h, 16 / 9, places=2)
        self.assertEqual((x + w / 2, y + h / 2), (356 + 380, 240 + 50))

    def test_a_box_at_an_edge_stays_inside_the_frame(self):
        for box in [(1800, 1000, 400, 100), (-50, -50, 200, 200), (0, 0, 4000, 100)]:
            x, y, w, h = effects.box169(box)
            self.assertTrue(0 <= x and 0 <= y and x + w <= 1920 and y + h <= 1080, (box, (x, y, w, h)))

    def test_the_zoom_scales_each_frame_then_follows_with_a_crop(self):
        scale, crop = effects.zoom([{"at": 2, "for": 5, "box": [356, 240, 760, 428]}])
        self.assertTrue(scale.startswith("scale=") and ":eval=frame" in scale, scale)
        self.assertTrue(crop.startswith("crop=1920:1080:"), crop)
        # crop's iw is the size the graph was set up with, not each frame's:
        # a clamp on it held every zoom in the top-left corner.
        self.assertNotIn("iw-", crop)

    def test_the_ease_is_still_outside_its_window_and_smooth_inside(self):
        k = effects.ease(2, 7, 1)
        self.assertIn("clip((t-2)/1,0,1)", k)
        self.assertIn("clip((7-t)/1,0,1)", k)
        self.assertIn("(3-2*", k)

    def test_no_zoom_is_no_filter(self):
        self.assertEqual(effects.zoom([]), [])


class Clip(unittest.TestCase):
    def clip(self, **extra):
        return dict({"speed": 8.0, "length": 12.0}, **extra)

    def test_a_clip_runs_at_its_speed_at_60_frames(self):
        g = effects.graph(self.clip(), {})
        self.assertTrue(g.startswith("[0:v]setpts=(PTS-STARTPTS)/8,fps=60"), g)
        self.assertTrue(g.endswith("[v]"), g)

    def test_a_sped_clip_says_so_in_the_mono_face(self):
        g = effects.graph(self.clip(tag="×8"), {"tag": "/w/tag.txt"})
        self.assertIn(f"fontfile='{brand.MONO}'", g)
        self.assertIn(f"fontcolor={brand.ACCENT}", g)

    def test_the_boot_menu_is_put_back_to_16_10_with_bars(self):
        scale, pad = effects.aspect(1.6)
        self.assertEqual(scale, "scale=1728:1080")
        self.assertTrue(pad.startswith("pad=1920:1080:(ow-iw)/2:0"), pad)

    def test_a_lower_third_fades_and_slides_in_its_window_only(self):
        g = effects.graph(self.clip(lower=[{"at": 2, "for": 5, "title": "Settings", "text": "x"}]),
                          {"title0": "/w/t", "line0": "/w/l"})
        self.assertIn("fade=t=in:st=2:d=0.4:alpha=1", g)
        self.assertIn("fade=t=out:st=6.6:d=0.4:alpha=1", g)
        self.assertIn("enable='between(t,2,7)'", g)
        self.assertIn(f"color=c={brand.PANEL}@0.9", g)
        self.assertTrue(g.endswith("[b1]null[v]"), g)


class Join(unittest.TestCase):
    def test_each_part_starts_one_fade_before_the_last_ends(self):
        for got, want in zip(join.starts([4, 10, 20], 0.8), [0.0, 3.2, 12.4]):
            self.assertAlmostEqual(got, want)
        self.assertAlmostEqual(join.total([4, 10, 20], 0.8), 32.4)

    def test_the_cross_fades_fall_where_the_chapters_start(self):
        g = join.graph([[0], [1, 2], [3]], [4, 10, 20], 0.8)
        self.assertIn("[1:v][2:v]concat=n=2:v=1:a=0", g)
        self.assertIn("[p0][p1]xfade=transition=fade:duration=0.8:offset=3.200[x1]", g)
        self.assertIn("[x1][p2]xfade=transition=fade:duration=0.8:offset=12.400[film]", g)

    def test_one_part_needs_no_fade(self):
        self.assertTrue(join.graph([[0]], [5], 0.8).endswith("[p0]null[film]"))

    def test_the_chapter_list_starts_at_zero(self):
        self.assertEqual(join.chapter_list(["Title", "Boot"], [0.0, 65.4]), "0:00 Title\n1:05 Boot\n")
        self.assertEqual(join.clock(3725), "1:02:05")

    def test_the_metadata_marks_each_chapter_in_milliseconds(self):
        meta = join.ffmetadata(["A=B", "C"], [0.0, 3.2], 10.0)
        self.assertTrue(meta.startswith(";FFMETADATA1\n"))
        self.assertIn("START=3200\nEND=10000\ntitle=C", meta)
        self.assertIn("title=A\\=B", meta)


class Brand(unittest.TestCase):
    def test_the_colours_are_the_desktops_own(self):
        palette = (HERE.parents[2] / "userland/capsule_desktop_shell/src/render/palette.rs").read_text()
        rgb = {name: f"0x{value[2:].replace('_', '')[2:]}" for name, value in
               re.findall(r"pub const (\w+): u32 = (0x[0-9A-F_]+);", palette)}
        self.assertEqual((brand.BACK, brand.PANEL, brand.ACCENT, brand.TEXT, brand.DIM),
                         (rgb["BAR"], rgb["PANEL_SOLID"], rgb["ACCENT"], rgb["TEXT"], rgb["TEXT_DIM"]))

    def test_the_faces_are_in_the_tree(self):
        for face in (brand.TITLE, brand.MEDIUM, brand.BODY, brand.MONO):
            self.assertTrue(face.is_file(), face)

    def test_a_card_is_drawn_in_them(self):
        cmd = cards.command({"label": "/w/a", "title": "/w/b", "line": "/w/c"}, 3, "/o/1.mp4")
        vf = cmd[cmd.index("-vf") + 1]
        self.assertIn(f"color=c={brand.CARD}:s=1920x1080:r=60:d=3", cmd)
        for face in (brand.MONO, brand.TITLE, brand.BODY):
            self.assertIn(f"fontfile='{face}'", vf)
        self.assertIn(f"color={brand.ACCENT}:t=fill", vf)


class Film(unittest.TestCase):
    def test_the_tours_edit_cuts_only_at_captions_the_tour_has(self):
        tour = (HERE.parent / "tour.steps").read_text(encoding="utf-8").splitlines()
        captions = [line.strip()[4:] for line in tour if line.strip().startswith("say:")]
        film = json.loads((HERE / "film.json").read_text(encoding="utf-8"))
        named = re.findall(r'"cue": "([^"]+)"', json.dumps(film, ensure_ascii=False))
        self.assertTrue(named)
        for cue in named:
            self.assertTrue(any(c.startswith(cue) for c in captions), cue)


if __name__ == "__main__":
    unittest.main()
