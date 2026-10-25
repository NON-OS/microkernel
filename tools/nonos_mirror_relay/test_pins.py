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
"""The relay serves a package the catalogue lists only as the bytes its lock
pins, passes anything else through, and tampers only when asked. Run:
python3 -m unittest discover -s tools/nonos_mirror_relay -t tools"""

import unittest

from .pins import ALPINE, checked, pinned, tampered

PINS = {"age": {"V": "1.2.1-r0", "C": "Q1x=", "S": "3", "B": "aa" * 32}}
AGE = "/alpine/v3.20/community/x86_64/age-1.2.1-r0.apk"


def good(_):
    return "aa" * 32


class Pins(unittest.TestCase):
    def test_only_the_locked_version_on_alpine_s_cdn_is_pinned(self):
        self.assertEqual(pinned(ALPINE, AGE, PINS), ("age", "aa" * 32))
        self.assertEqual(pinned(ALPINE, AGE.replace("community", "main"), PINS), ("age", "aa" * 32))
        for host, path in ((ALPINE, AGE.replace("1.2.1", "1.2.0")), ("deb.debian.org", AGE),
                           (ALPINE, AGE.replace("v3.20", "v3.19")), (ALPINE, AGE.replace("x86_64", "aarch64")),
                           (ALPINE, AGE + "?x"), (ALPINE, "/alpine/v3.20/main/x86_64/APKINDEX.tar.gz")):
            self.assertIsNone(pinned(host, path, PINS), (host, path))

    def test_a_pinned_package_is_sent_only_as_the_bytes_it_pins(self):
        self.assertEqual(checked(ALPINE, AGE, b"apk", PINS, (), good), b"apk")
        with self.assertRaises(ValueError):
            checked(ALPINE, AGE, b"apk", PINS, (), lambda _: "bb" * 32)

    def test_anything_else_passes_through_unhashed(self):
        def never(_):
            raise AssertionError("hashed an unpinned file")
        dep = "/alpine/v3.20/main/x86_64/musl-1.2.5-r0.apk"
        self.assertEqual(checked(ALPINE, dep, b"dep", PINS, {"age"}, never), b"dep")

    def test_a_tampered_package_differs_and_only_when_asked(self):
        bad = checked(ALPINE, AGE, b"apk", PINS, {"age"}, good)
        self.assertNotEqual(bad, b"apk")
        self.assertEqual(len(bad), 3)
        self.assertNotEqual(tampered(b""), b"")


if __name__ == "__main__":
    unittest.main()
