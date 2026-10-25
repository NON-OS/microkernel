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
"""The destination list the policy command writes, against the bytes
nonos_linux_policy's build writes for the same items (build/dests_arg.rs),
and what that crate refuses."""

import unittest

from .dests import Refused, dests_hash, dests_list

# build/dests_arg.rs's dests_of on ITEMS, and dests_hash of it.
ITEMS = ["api.example.org:8000-8080", "[2001:db8::]/32:1-65535", "10.1.2.0/24:443", "b.example:443"]
LIST = ("0004041801bb01bb040a01020006200001ffff1020010db800000000000000000000"
        "00004e0001bb01bb09622e6578616d706c654e001f401f900f6170692e6578616d706c652e6f7267")
HASH = "4a8a2dcd9ba7191beafb51234b3542f54a48c3ee60282e8ba65ea2f175f9201c"



class Dests(unittest.TestCase):
    def test_the_list_and_its_hash_are_the_crates_in_any_order(self):
        for items in (ITEMS, ITEMS[::-1]):
            self.assertEqual(dests_list(items).hex(), LIST)
        self.assertEqual(dests_hash(bytes.fromhex(LIST)).hex(), HASH)

    def test_what_the_crate_refuses_is_refused(self):
        for item in ("10.1.2.1/24:443", "example.org", "example.org:0", "example.org:90-80",
                     "Example.org:443", "10.0.0.0/33:443", "[::ffff:1.2.3.4]/128:443",
                     "1.2.3.4:443", "a..org:443", "-a.org:443", "999.0.0.0/8:443", "2001:db8::/32:443"):
            with self.assertRaises(Refused, msg=item):
                dests_list([item])
        with self.assertRaises(Refused):
            dests_list(["a.org:443", "a.org:443"])
        with self.assertRaises(Refused):
            dests_list([f"h{i}.org:443" for i in range(65)])



if __name__ == "__main__":
    unittest.main()
