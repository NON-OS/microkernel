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

"""The image and its chunk list, as nonos_app_image reads them: the
committed vector written again byte for byte, every field where the
parser looks, and every refusal the parser makes refused here first."""

import os
import struct
import unittest

from . import image_format, image_vector
from .blake3 import blake3
from .image_hash import digest


class Format(unittest.TestCase):
    def test_the_committed_vector_is_written_again_byte_for_byte(self):
        image, listing = image_vector.build()
        self.assertEqual(image, (image_vector.VECTORS / "small.image").read_bytes())
        self.assertEqual(listing, (image_vector.VECTORS / "small.chunks").read_bytes())
        root = (image_vector.VECTORS / "small.measurement").read_text().strip()
        self.assertEqual(image_format.measurement(listing).hex(), root)

    def test_packing_twice_gives_the_same_bytes_and_measurement(self):
        a, la = image_vector.build()
        b, lb = image_vector.build()
        self.assertEqual(a, b)
        self.assertEqual(image_format.measurement(la), image_format.measurement(lb))

    def test_the_header_and_rows_are_where_the_parser_reads_them(self):
        image, _ = image_vector.build()
        magic, version, flags, count, _, names_len, _, total = struct.unpack_from("<8sHHHHII Q", image)
        self.assertEqual((magic, version, flags, count, total), (b"NONOSAI1", 1, 0, 3, len(image)))
        self.assertEqual(image[32:64], bytes(32))
        rows = [image[64 + 64 * i:128 + 64 * i] for i in range(count)]
        self.assertEqual([r[19] for r in rows], [1, 0, 0])
        lens = [struct.unpack_from("<H", r, 16)[0] for r in rows]
        self.assertEqual(sum(lens), names_len)
        at = 64 + 64 * count
        self.assertEqual(image[at:at + names_len], b"demolibdemodemo-data")
        stored = sum(struct.unpack_from("<Q", r, 0)[0] for r in rows)
        self.assertEqual(at + names_len + stored, len(image))

    def test_the_list_names_each_chunk_by_its_blake3(self):
        image = os.urandom(3 * image_format.CHUNK_MIN + 5)
        listing = image_format.chunk_list(image, image_format.CHUNK_MIN)
        self.assertEqual(listing[:8], b"NONOSAC1")
        self.assertEqual(len(listing), 32 + 4 * 32)
        self.assertEqual(listing[-32:], blake3(image[-5:]))
        for bad in (1000, image_format.CHUNK_MIN // 2, image_format.CHUNK_MAX * 2):
            with self.assertRaises(ValueError):
                image_format.chunk_list(image, bad)
        with self.assertRaises(ValueError):
            image_format.chunk_list(b"", image_format.CHUNK_MIN)

    def test_b3sum_and_the_pure_blake3_agree(self):
        data = os.urandom((1 << 20) + 1025)
        self.assertEqual(digest(data), blake3(data))

    def test_what_the_parser_refuses_the_builder_refuses(self):
        t = b"\1" * 512
        ok = image_format.Part(b"a", image_format.TAR, t, t)
        self.assertTrue(image_format.build([ok]))
        for bad in ([], [ok, ok], [ok._replace(name=b"A b")], [ok._replace(name=b"-a")],
                    [ok._replace(name=b"a/b")], [ok._replace(name=b"a" * 65)],
                    [ok._replace(tar=t[:100], stored=t[:100])], [ok._replace(stored=b"x")],
                    [ok._replace(codec=2)]):
            with self.assertRaises(ValueError, msg=repr(bad)[:80]):
                image_format.build(bad)
