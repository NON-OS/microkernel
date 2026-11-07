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

"""The box an image's entry carries: the program found in the members, the
last copy of it, an ELF; the services by name, the person's files among
them."""

import unittest

from . import image_box, image_format, image_members, image_vector
from .blake3 import blake3


def members():
    return image_members.members(image_vector.packages())


class Box(unittest.TestCase):
    def test_the_program_and_its_digest_are_the_members_bytes(self):
        b = image_box.box(members(), "WAYLAND FILES_HOME", "usr/bin/demo")
        self.assertEqual(b["program"], "/usr/bin/demo")
        self.assertEqual(b["mask"], ["WAYLAND", "FILES_HOME"])
        self.assertEqual(b["mask_bits"], "0x9")
        self.assertEqual(len(b["program_blake3"]), 64)
        body = image_box.program(members(), "/usr/bin/demo")
        self.assertEqual(blake3(body).hex(), b["program_blake3"])

    def test_a_later_members_copy_is_the_one_that_lands(self):
        first = image_vector.tar({"usr/bin/p": b"\x7fELF one"})
        later = image_vector.tar({"usr/bin/p": b"\x7fELF two"})
        parts = [image_format.Part(b"a", 0, first, first), image_format.Part(b"b", 0, later, later)]
        self.assertEqual(image_box.program(parts, "usr/bin/p"), b"\x7fELF two")

    def test_a_path_the_device_would_not_bind_is_refused(self):
        same = b"\x7fELF same"
        first = image_vector.tar({"usr/lib/a": same})
        later = image_vector.tar({"usr/lib/b": same})
        parts = [image_format.Part(b"a", 0, first, first), image_format.Part(b"b", 0, later, later)]
        self.assertEqual(image_box.bound_at(parts, same), "usr/lib/a")
        self.assertEqual(image_box.box(parts, "WAYLAND", "usr/lib/a")["program"], "/usr/lib/a")
        with self.assertRaises(ValueError):
            image_box.box(parts, "WAYLAND", "usr/lib/b")

    def test_what_cannot_be_a_box_is_refused(self):
        with self.assertRaises(ValueError):
            image_box.box(members(), "WAYLAND", "usr/bin/missing")
        with self.assertRaises(ValueError):
            image_box.box(members(), "WAYLAND", "usr/share/demo/x")
        user = image_box.box(members(), "WAYLAND FILES_HOME FILES_USER", "usr/bin/demo")
        self.assertEqual(user["mask"], ["WAYLAND", "FILES_HOME", "FILES_USER"])
        self.assertEqual(user["mask_bits"], "0x19")
        with self.assertRaises(ValueError):
            image_box.box(members(), "WAYLAND NOPE", "usr/bin/demo")
