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
"""The relay serves /linux/<blake3>.tar from --content only under the dev
mirror's name, only when the file's BLAKE3 is its name, and nothing of it
without --content. Run: python3 -m unittest discover -s tools/nonos_mirror_relay -t tools"""

import hashlib
import tempfile
import unittest
from pathlib import Path

from .content import DEV, answer


def b3(data: bytes) -> str:
    """A stand-in digest: the rule is that the name is the digest, whichever."""
    return hashlib.sha256(data).hexdigest()


class Content(unittest.TestCase):
    def setUp(self):
        self.dir = tempfile.TemporaryDirectory()
        self.at = Path(self.dir.name)
        self.good = b3(b"tar")
        (self.at / f"{self.good}.tar").write_bytes(b"tar")
        self.bad = "ab" * 32
        (self.at / f"{self.bad}.tar").write_bytes(b"not the pinned bytes")

    def tearDown(self):
        self.dir.cleanup()

    def test_a_file_whose_blake3_is_its_name_is_served(self):
        code, body, _ = answer(self.at, DEV, f"/linux/{self.good}.tar", b3)
        self.assertEqual((code, body), (200, b"tar"))

    def test_a_file_whose_bytes_are_not_its_name_is_502(self):
        self.assertEqual(answer(self.at, DEV, f"/linux/{self.bad}.tar", b3)[:2], (502, b""))
        self.assertEqual(answer(self.at, DEV, f"/linux/{'cd' * 32}.tar", b3)[:2], (404, b""))

    def test_another_host_path_or_no_content_is_not_served_from_it(self):
        path = f"/linux/{self.good}.tar"
        for host, p in (("dl-cdn.alpinelinux.org", path), ("10.0.2.3", path), (DEV, path + "?x"),
                        (DEV, path.upper()), (DEV, f"/linux/{self.good[:63]}.tar"), (DEV, "/linux/../x.tar"),
                        (DEV, f"/x/{self.good}.tar"), (DEV, f"/linux/{self.good}.tar.gz")):
            self.assertIsNone(answer(self.at, host, p, b3), (host, p))
        self.assertIsNone(answer(None, DEV, path, b3))

    def test_the_name_is_the_one_a_dev_image_asks_with(self):
        self.assertEqual(DEV, "10.0.2.2")


if __name__ == "__main__":
    unittest.main()
