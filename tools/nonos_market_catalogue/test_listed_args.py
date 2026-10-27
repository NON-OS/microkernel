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
"""A recipe's args: each rule refuses on its own, the block is the bytes
nonos_linux_policy::args reads (its args_tests.rs pins the same), and a
listing carries it after the policy. Run:
python3 -m unittest discover -s tools/nonos_market_catalogue -t tools"""

import unittest
from pathlib import Path

from .apk_fixture import ELF, checksum, make
from .catalogue import catalogue_entries
from .listed_args import MAX_WORDS, block
from .recipes import checked

PINNED = ("4e4c58415247303102232d2d6461656d6f6e2d616464726573733d6e6f64652e6578616d706c653a3138303831"
          "122d2d756e747275737465642d6461656d6f6e")
BASE = {"package": "x", "licence": "MIT", "mask": "TTY NET_ANON", "name": "X", "text": "x"}


class Args(unittest.TestCase):
    def test_the_block_is_the_crate_s_pinned_bytes(self):
        self.assertEqual(block(["--daemon-address=node.example:18081", "--untrusted-daemon"]).hex(), PINNED)

    def test_each_rule_refuses_on_its_own(self):
        for bad, said in ((["-a"], "not --name"), (["--A"], "not --name"), (["--k="], "not --name"),
                          (["--k=a b"], "not --name"), (["--k=é"], "not --name"), (["--a;b"], "not --name"),
                          (["--k=" + "v" * 252], "not --name"), (["--a", "--a=1"], "twice"),
                          ([], "1 to"), ([f"--w{i}" for i in range(MAX_WORDS + 1)], "1 to"),
                          ([f"--{c}=" + "v" * 180 for c in "abcdef"], "past 1024"),
                          ("--a", "list of strings"), ([3], "list of strings")):
            with self.assertRaisesRegex(ValueError, said, msg=bad):
                block(bad)
        self.assertEqual(len(block([f"--{c}=" + "v" * 180 for c in "abcde"])), 9 + 5 * 185)
        self.assertEqual(block(["--k=" + "v" * 251])[9], 255)

    def test_a_recipe_takes_a_list_for_args_alone(self):
        self.assertEqual(checked(dict(BASE, args=["--off"]), Path("."))["args"], block(["--off"]))
        self.assertEqual(checked(BASE, Path("."))["args"], b"")
        for item in (dict(BASE, args="--off"), dict(BASE, args=["--off", "x"]), dict(BASE, name=["X"])):
            with self.assertRaises(ValueError, msg=item):
                checked(item, Path("."))

    def test_the_listing_carries_them_after_its_policy(self):
        recipe = checked(dict(BASE, args=["--node=a:1", "--off"]), Path("."))
        raw = make([("usr/bin/x", ELF)])
        rec = {"P": "x", "V": "1-r0", "L": "MIT", "base": "https://e.org", "C": checksum(raw), "S": str(len(raw))}
        (e,) = catalogue_entries([recipe], {"x": rec}, lambda r: raw, "11" * 32, 1)
        p = bytes.fromhex(e["releases"][0]["ext"]["guest_policy"])
        self.assertEqual((len(p), p[256:]), (256 + len(recipe["args"]), recipe["args"]))


if __name__ == "__main__":
    unittest.main()
