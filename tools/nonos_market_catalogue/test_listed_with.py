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
"""A recipe's with: each rule refuses on its own, the block is the bytes
nonos_linux_policy::with reads (its with_tests.rs pins the same), and a
listing carries it after the args, or after the policy when there are none.
Run: python3 -m unittest discover -s tools/nonos_market_catalogue -t tools"""

import unittest
from pathlib import Path

from .apk_fixture import ELF, checksum, make
from .catalogue import catalogue_entries
from .listed_args import block as args_block
from .listed_with import MAX_NAMES, block
from .recipes import checked

PINNED = "4e4c58574954483102117a6174687572612d7064662d6d757064660c706f70706c65722d64617461"
BASE = {"package": "x", "licence": "MIT", "mask": "TTY NET_ANON", "name": "X", "text": "x"}


def policy_tail(recipe: dict) -> bytes:
    raw = make([("usr/bin/x", ELF)])
    rec = {"P": "x", "V": "1-r0", "L": "MIT", "base": "https://e.org", "C": checksum(raw), "S": str(len(raw))}
    (e,) = catalogue_entries([recipe], {"x": rec}, lambda r: raw, "11" * 32, 1)
    return bytes.fromhex(e["releases"][0]["ext"]["guest_policy"])[256:]


class With(unittest.TestCase):
    def test_the_block_is_the_crate_s_pinned_bytes(self):
        self.assertEqual(block(["zathura-pdf-mupdf", "poppler-data"], "zathura").hex(), PINNED)
        self.assertEqual(len(block(["a" * 64] * 1, "x")), 9 + 65)

    def test_each_rule_refuses_on_its_own(self):
        for bad, said in ((["A"], "not a package"), (["../x"], "not a package"), (["-x"], "not a package"),
                          (["a" * 65], "not a package"), (["a", "a"], "twice"), (["b", "x"], "own package"),
                          ([], "1 to"), ([f"p{i}" for i in range(MAX_NAMES + 1)], "1 to"),
                          ("a", "list of strings"), ([3], "list of strings")):
            with self.assertRaisesRegex(ValueError, said, msg=bad):
                block(bad, "x")
        self.assertEqual(block([f"p{i}" for i in range(MAX_NAMES)], "x")[8], MAX_NAMES)

    def test_a_recipe_takes_a_list_for_with_and_bounds_it_with_its_args(self):
        self.assertEqual(checked(dict(BASE, **{"with": ["y"]}), Path("."))["with_block"], block(["y"], "x"))
        self.assertEqual(checked(BASE, Path("."))["with_block"], b"")
        many = [f"--{c}=" + "v" * 180 for c in "abcde"]
        for item in (dict(BASE, **{"with": "y"}), dict(BASE, **{"with": ["y", "Y"]}),
                     dict(BASE, args=many, **{"with": ["y" * 60, "z" * 60]})):
            with self.assertRaises(ValueError, msg=item):
                checked(item, Path("."))

    def test_the_listing_carries_them_after_its_args(self):
        both = checked(dict(BASE, args=["--off"], **{"with": ["y", "z"]}), Path("."))
        self.assertEqual(policy_tail(both), args_block(["--off"]) + block(["y", "z"], "x"))
        self.assertEqual(policy_tail(checked(dict(BASE, **{"with": ["y"]}), Path("."))), block(["y"], "x"))
        self.assertEqual(policy_tail(checked(dict(BASE, args=["--off"]), Path("."))), args_block(["--off"]))


if __name__ == "__main__":
    unittest.main()
