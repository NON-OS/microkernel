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
"""A catalogue listing is free, pins the .apk, and carries the guest_policy
of the program the installer records, under the publisher's ids. Run:
python3 -m unittest discover -s tools/nonos_market_catalogue -t tools"""

import unittest

from nonos_linux_pack.policy import listing_id, release_id

from .apk_fixture import ELF, checksum, make
from .catalogue import catalogue_entries
from .digest import blake3
from .recipes import UNMEASURED

KEY = "11" * 32
BASE = "https://dl-cdn.alpinelinux.org/alpine/v3.20/main/x86_64"
RECIPE = {"package": "tool", "licence": "MIT", "mask": 0x108, "dests": None,
          "calls": UNMEASURED, "name": "Tool", "text": "A tool."}
RAW = make([("usr/bin/other", ELF + b"o"), ("usr/bin/tool", ELF + b"t")])
REC = {"P": "tool", "V": "1.0-r0", "L": "MIT", "base": BASE, "C": checksum(RAW), "S": str(len(RAW))}


def listed(recipe=RECIPE, rec=REC, raw=RAW):
    return catalogue_entries([recipe], {rec["P"]: rec}, lambda r: raw, KEY, 1000)


class Listing(unittest.TestCase):
    def test_the_release_pins_the_apk_and_its_policy_names_the_program(self):
        (e,) = listed()
        rel = e["releases"][0]
        self.assertEqual((e["listing_id"], rel["release_id"]), ("linux.tool", "tool@1.0-r0"))
        self.assertEqual(rel["package_hash"], blake3(RAW))
        self.assertEqual(rel["package_url"], f"{BASE}/tool-1.0-r0.apk")
        p = bytes.fromhex(rel["ext"]["guest_policy"])
        lid = listing_id(bytes.fromhex(KEY), "linux.tool")
        self.assertEqual(len(p), 256)
        self.assertEqual(p[16:48].hex(), blake3(ELF + b"t"))
        self.assertEqual(int.from_bytes(p[48:56], "little"), 0x108)
        self.assertEqual((p[56:88], p[88:120]), (lid, release_id(lid, "tool@1.0-r0")))
        self.assertEqual((p[120:152], p[152:217]), (bytes(32), UNMEASURED))

    def test_it_is_sold_free_with_nothing_to_bind(self):
        (e,) = listed()
        self.assertEqual(e["commerce"]["settle_mode"], "free")
        self.assertEqual(e["commerce"]["unit_price"], "0")
        self.assertEqual(e["price"]["kind"], "free")

    def test_a_licence_the_index_changed_leaves_the_app_out(self):
        self.assertEqual(listed(rec=dict(REC, L="MIT AND custom")), [])

    def test_a_cited_upstream_licence_names_what_it_ships(self):
        gpl = dict(RECIPE, licence="GPL-3.0-or-later", index_licence="MIT")
        (e,) = listed(recipe=gpl)
        self.assertIn("licence GPL-3.0-or-later", e["releases"][0]["validation"]["note"])
        self.assertEqual(listed(recipe=gpl, rec=dict(REC, L="LGPL-3.0-or-later")), [], "the index moved")

    def test_a_package_with_no_program_or_missing_from_the_index_is_left_out(self):
        self.assertEqual(listed(raw=make([("usr/share/tool/x", b"x")])), [])
        self.assertEqual(catalogue_entries([RECIPE], {}, lambda r: RAW, KEY, 1000), [])

    def test_a_package_that_does_not_inflate_is_left_out(self):
        self.assertEqual(listed(raw=RAW[:-7]), [])


if __name__ == "__main__":
    unittest.main()
