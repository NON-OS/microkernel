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

"""The relay serves an app image's list only when its BLAKE3 is its name,
the image only beside it and of its length, whole or by the one range the
installer asks per chunk. Run: python3 -m unittest discover -s
tools/nonos_mirror_relay -t tools"""

import hashlib
import struct
import tempfile
import unittest
from pathlib import Path

from .content import DEV
from .image import answer


def b3(data: bytes) -> str:
    return hashlib.sha256(data).hexdigest()


class Images(unittest.TestCase):
    def setUp(self):
        self.dir = tempfile.TemporaryDirectory()
        self.at = Path(self.dir.name)
        self.image = bytes(range(256)) * 40
        self.list = b"NONOSAC1" + struct.pack("<IIQ", 1, 65536, len(self.image)) + bytes(8) + bytes(32)
        self.pin = b3(self.list)
        (self.at / f"{self.pin}.chunks").write_bytes(self.list)
        (self.at / f"{self.pin}.image").write_bytes(self.image)

    def tearDown(self):
        self.dir.cleanup()

    def ask(self, kind, rng=""):
        return answer(self.at, DEV, f"/linux/{self.pin}.{kind}", b3, rng)

    def test_the_list_and_the_whole_image(self):
        self.assertEqual(self.ask("chunks")[:2], (200, self.list))
        self.assertEqual(self.ask("image")[:2], (200, self.image))

    def test_one_range_is_206_with_its_content_range(self):
        code, body, _, headers = self.ask("image", "bytes=100-199")
        self.assertEqual((code, body), (206, self.image[100:200]))
        self.assertEqual(headers["Content-Range"], f"bytes 100-199/{len(self.image)}")
        for bad in ("bytes=200-100", f"bytes=0-{len(self.image)}", "bytes=5-", "bytes=0-1,4-5", "lines=1-2"):
            self.assertEqual(self.ask("image", bad)[0], 416, bad)

    def test_a_list_off_its_name_or_an_image_off_its_length_is_not_served(self):
        (self.at / f"{self.pin}.chunks").write_bytes(self.list + b"x")
        self.assertEqual(self.ask("chunks")[0], 502)
        self.assertEqual(self.ask("image")[0], 502)
        (self.at / f"{self.pin}.chunks").write_bytes(self.list)
        (self.at / f"{self.pin}.image").write_bytes(self.image[:-1])
        self.assertEqual(self.ask("image")[0], 404)
        self.assertEqual(answer(self.at, DEV, f"/linux/{'cd' * 32}.chunks", b3)[0], 404)

    def test_another_host_path_or_no_content_is_not_an_image_request(self):
        for host, p in (("10.0.2.3", f"/linux/{self.pin}.image"), (DEV, f"/linux/{self.pin}.img"),
                        (DEV, f"/linux/{self.pin}.tar"), (DEV, f"/linux/{self.pin[:63]}.chunks")):
            self.assertIsNone(answer(self.at, host, p, b3), (host, p))
        self.assertIsNone(answer(None, DEV, f"/linux/{self.pin}.chunks", b3))
