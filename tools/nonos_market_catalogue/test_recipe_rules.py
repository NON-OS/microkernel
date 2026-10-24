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
"""Each recipe rule refuses on its own (unknown key, a name not Alpine's or in
another family's namespace, a set of the wrong length, a dests file not
text), and a dests list reaches the policy as its hash, never zero. Run:
python3 -m unittest discover -s tools/nonos_market_catalogue -t tools"""

import tempfile
import unittest
from pathlib import Path

from nonos_linux_pack.dests import dests_hash, dests_list

from .apk_fixture import ELF, checksum, make
from .catalogue import catalogue_entries
from .recipes import UNMEASURED, checked

BASE = {"package": "x", "licence": "MIT", "mask": "TTY", "name": "X", "text": "x"}
ROOT = Path(".")


class Rules(unittest.TestCase):
    def test_each_bad_field_is_refused_by_its_own_rule(self):
        for item in (dict(BASE, colour="red"), dict(BASE, package="../x"), dict(BASE, package="kali.x"),
                     dict(BASE, package="nonos-rg"), dict(BASE, package="qwen-max"),
                     dict(BASE, package="blackarch.x"), dict(BASE, calls="ff" * 64),
                     dict(BASE, calls="ff" * 64 + "0000")):
            with self.assertRaises(ValueError, msg=item):
                checked(item, ROOT)

    def test_a_dests_file_that_is_not_text_is_refused(self):
        with tempfile.TemporaryDirectory() as d:
            (Path(d) / "dests").write_bytes(b"\xff\xfe\x00")
            with self.assertRaises(ValueError):
                checked(dict(BASE, mask="NET_ANON", dests="dests"), Path(d))


class Policy(unittest.TestCase):
    def test_a_dests_list_is_bound_by_its_hash(self):
        listed = dests_list(["example.org:443"])
        recipe = dict(BASE, mask=0x102, dests=listed, calls=UNMEASURED)
        raw = make([("usr/bin/x", ELF)])
        rec = {"P": "x", "V": "1-r0", "L": "MIT", "base": "https://e.org", "C": checksum(raw), "S": str(len(raw))}
        (e,) = catalogue_entries([recipe], {"x": rec}, lambda r: raw, "11" * 32, 1)
        p = bytes.fromhex(e["releases"][0]["ext"]["guest_policy"])
        self.assertEqual(p[120:152], dests_hash(listed))
        self.assertNotEqual(p[120:152], bytes(32))

    def test_a_fetch_that_fails_leaves_the_app_out(self):
        recipe = dict(BASE, mask=0x100, dests=None, calls=UNMEASURED)
        rec = {"P": "x", "V": "1-r0", "L": "MIT", "base": "https://e.org", "C": "Q1" + "A" * 28, "S": "9"}

        def broken(r):
            raise OSError("connection reset")
        self.assertEqual(catalogue_entries([recipe], {"x": rec}, broken, "11" * 32, 1), [])
        raw = make([("usr/bin/x", ELF)])
        wrong = dict(rec, C=checksum(make([("usr/bin/y", ELF)])), S=str(len(raw)))
        self.assertEqual(catalogue_entries([recipe], {"x": wrong}, lambda r: raw, "11" * 32, 1), [])


if __name__ == "__main__":
    unittest.main()
