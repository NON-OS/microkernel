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
"""A later plan never gives a formatted volume fewer sectors than it had."""

import os
import struct
import subprocess
import sys
import tempfile
import unittest

from .image import PLAN_LBA, SECTOR, kept_sectors

HERE = os.path.dirname(os.path.abspath(__file__))
TOOL = os.path.join(os.path.dirname(HERE), "nonos-data-plan.py")
BASE = 262_144


def plan(image, sectors, *extra):
    out = subprocess.run([sys.executable, TOOL, image, "--volume-sectors", str(sectors), *extra],
                         check=True, capture_output=True, text=True)
    return out.stdout


def window(image):
    with open(image, "rb") as img:
        img.seek(PLAN_LBA * SECTOR)
        return struct.unpack_from("<2Q", img.read(SECTOR), 8)


class Kept(unittest.TestCase):
    def setUp(self):
        self.dir = tempfile.TemporaryDirectory()
        self.image = os.path.join(self.dir.name, "data.img")
        open(self.image, "wb").close()

    def tearDown(self):
        self.dir.cleanup()

    def test_blank_image_keeps_nothing(self):
        self.assertEqual(kept_sectors(self.image, BASE), 0)

    def test_smaller_plan_keeps_the_formatted_size(self):
        plan(self.image, 500_000)
        said = plan(self.image, 100_000)
        self.assertEqual(window(self.image), (BASE, 500_000))
        self.assertIn("keeping the volume at 500000 sectors", said)

    def test_larger_plan_grows(self):
        plan(self.image, 100_000)
        plan(self.image, 500_000)
        self.assertEqual(window(self.image), (BASE, 500_000))

    def test_imports_go_past_the_kept_volume(self):
        plan(self.image, 500_000)
        blob = os.path.join(self.dir.name, "m.gguf")
        with open(blob, "wb") as f:
            f.write(b"x" * 1000)
        said = plan(self.image, 100_000, "--import", blob)
        self.assertIn(f"at LBA {BASE + 500_000}", said)

    def test_fresh_makes_the_size_asked(self):
        plan(self.image, 500_000)
        plan(self.image, 100_000, "--fresh")
        self.assertEqual(window(self.image), (BASE, 100_000))


def gpt(image, alternate):
    """A primary GPT at LBA 1, 128 entries of 128 bytes from LBA 2, with its
    backup header at `alternate`, as the sealed stick carries."""
    header = b"EFI PART" + struct.pack("<2I", 0x10000, 92) + bytes(8)
    header += struct.pack("<4Q", 1, alternate, 34, alternate - 33)
    header += bytes(16) + struct.pack("<Q2I", 2, 128, 128)
    with open(image, "r+b") as img:
        img.seek(0)
        img.write(b"\x55" * SECTOR + header.ljust(SECTOR, b"\0") + b"\x66" * 32 * SECTOR)


def table(image):
    with open(image, "rb") as img:
        return img.read(34 * SECTOR)


class StickTable(unittest.TestCase):
    """The stick's partition table, copied onto a data disk with the store,
    put its backup inside the volume, and firmware wrote it back over a
    sealed model at every boot."""

    def setUp(self):
        self.dir = tempfile.TemporaryDirectory()
        self.image = os.path.join(self.dir.name, "data.img")
        with open(self.image, "wb") as f:
            f.truncate(128 << 20)

    def tearDown(self):
        self.dir.cleanup()

    def test_backup_inside_the_volume_is_removed(self):
        gpt(self.image, 786_431)
        said = plan(self.image, 1_098_034)
        self.assertIn("removed the partition table", said)
        self.assertEqual(table(self.image), bytes(34 * SECTOR))
        self.assertEqual(window(self.image), (BASE, 1_098_034))

    def test_backup_past_everything_is_left(self):
        gpt(self.image, 4_000_000)
        before = table(self.image)
        said = plan(self.image, 1_098_034)
        self.assertNotIn("removed", said)
        self.assertEqual(table(self.image), before)

    def test_qemu_data_disk_never_keeps_it(self):
        sys.path.insert(0, os.path.dirname(HERE))
        from nonos_qemu.disk import drop_gpt
        gpt(self.image, 4_000_000)
        self.assertTrue(drop_gpt(self.image))
        self.assertEqual(table(self.image), bytes(34 * SECTOR))
        self.assertFalse(drop_gpt(self.image))


if __name__ == "__main__":
    unittest.main()
