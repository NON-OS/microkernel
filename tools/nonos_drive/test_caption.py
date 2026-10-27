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

"""A caption reaches the recording whole: written beside the caption file and
renamed over it, so ffmpeg, reading the file at every frame, sees the old
caption or the new one and never a file cut short. A reader that opened the
old caption keeps it whole; a write in place would change it under the
reader, and a write never renamed would leave the old caption up."""

import os
import sys
import tempfile
import unittest
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parents[1]))

from nonos_drive import caption  # noqa: E402


class Caption(unittest.TestCase):
    def setUp(self):
        self.dir = tempfile.TemporaryDirectory()
        self.path = Path(self.dir.name) / caption.NAME

    def tearDown(self):
        self.dir.cleanup()

    def test_the_new_caption_is_there_whole(self):
        caption.say(self.path, "Welcome to NØNOS.")
        self.assertEqual(self.path.read_bytes(), "Welcome to NØNOS.".encode("utf-8"))
        caption.say(self.path, "The boot menu.")
        self.assertEqual(self.path.read_text(), "The boot menu.")

    def test_a_reader_of_the_old_caption_keeps_it_whole(self):
        caption.say(self.path, "the old caption, a long line")
        with open(self.path) as reader:
            caption.say(self.path, "new")
            self.assertEqual(reader.read(), "the old caption, a long line")
        self.assertEqual(self.path.read_text(), "new")

    def test_it_is_a_new_file_each_time(self):
        caption.say(self.path, "one")
        first = os.stat(self.path).st_ino
        with open(self.path):
            caption.say(self.path, "two")
            self.assertNotEqual(os.stat(self.path).st_ino, first, "renamed over, not written in")

    def test_nothing_is_left_beside_it(self):
        caption.say(self.path, "a caption")
        caption.say(self.path, "")
        self.assertEqual(sorted(p.name for p in Path(self.dir.name).iterdir()), [caption.NAME])

    def test_an_empty_caption_clears_it(self):
        caption.say(self.path, "on screen")
        caption.say(self.path, "")
        self.assertEqual(self.path.read_text(), "")


if __name__ == "__main__":
    unittest.main()
