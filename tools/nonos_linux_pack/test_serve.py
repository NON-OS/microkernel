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
"""SERVE_ONE's version 2 bytes against the crate's: PACKED in
nonos_linux_policy's tests/serve_pack_tests.rs, which the crate decodes to
app.simplex on 5225. A version 1 policy is the bytes test_policy.py pins."""

import re
import unittest

from .policy import MASK, encode, mask_of
from .serve import serve_of
from .test_policy import CALLS, CRATE

RUST = (CRATE / "tests/serve_pack_tests.rs").read_text()
PACKED = "".join(re.findall(r'^\s*"([0-9a-f]{64})",$', RUST.split("const PACKED")[1], re.M)[:8])
IDS = [bytes([b]) * 32 for b in (0x11, 0x22, 0x33, 0x44)]
SERVED = MASK["NET_ANON"] | MASK["FILES_HOME"] | MASK["SERVE_ONE"]


def packed(mask: int, serve) -> bytes:
    return encode(IDS[0], mask, IDS[1], IDS[2], IDS[3], bytes.fromhex(CALLS), serve)


class Serve(unittest.TestCase):
    def test_the_bytes_are_the_ones_the_crate_decodes(self):
        self.assertEqual(len(PACKED), 512)
        self.assertEqual(packed(SERVED, serve_of("app.simplex:5225")).hex(), PACKED)

    def test_a_name_or_port_the_crate_refuses_is_refused(self):
        for good, want in (("a:1", (1, "a")), ("z0-9._:65535", (65535, "z0-9._")), ("q" * 32 + ":80", (80, "q" * 32))):
            self.assertEqual(serve_of(good), want)
        for bad in ("app.simplex:0", "app.simplex:65536", "app.simplex:05225", "app.simplex:+5", "app.simplex",
                    ":5225", "App:1", "0app:1", ".app:1", "a b:1", "a\0b:1", "q" * 33 + ":1", "\xe9t\xe9:1",
                    "app:1\n", 5225, None):
            with self.assertRaises(ValueError, msg=repr(bad)):
                serve_of(bad)
        with self.assertRaisesRegex(ValueError, "the policy crate refuses"):
            packed(SERVED, (0, "app.simplex"))

    def test_the_bit_and_the_field_go_together_and_never_with_wayland(self):
        with self.assertRaisesRegex(ValueError, "SERVE_ONE needs"):
            packed(MASK["NET_ANON"], (5225, "app.simplex"))
        with self.assertRaisesRegex(ValueError, "SERVE_ONE with WAYLAND"):
            packed(SERVED | MASK["WAYLAND"], (5225, "app.simplex"))
        with self.assertRaisesRegex(ValueError, "set by serve"):
            mask_of("NET_ANON SERVE_ONE")


if __name__ == "__main__":
    unittest.main()
