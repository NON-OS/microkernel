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
"""A source recipe for upstream's release binary names the asset, its
sha256 and upstream's checksum file in the same release, in place of a
commit and a build, and optionally the Debian family and its needs. Run:
python3 -m unittest discover -s tools/nonos_market_catalogue -t tools"""

import hashlib
import unittest

from .apk_fixture import ELF
from .source_recipe import checked
from .test_source_recipe import ROOT, SOURCE

AT = "https://github.com/simplex-chat/simplex-chat/releases/download/v6.4.5/"
PROGRAM = ELF + b"simplex"
ASSET = dict({k: v for k, v in SOURCE.items() if k not in ("commit", "build")},
             package="simplex-chat", program="usr/bin/simplex-chat", mask="FILES_HOME NET_ANON NET_LISTEN_LO",
             source="https://github.com/simplex-chat/simplex-chat", asset=AT + "simplex-chat-ubuntu-22_04-x86_64",
             sha256=hashlib.sha256(PROGRAM).hexdigest(), sums=AT + "_sha256sums", family="debian",
             needs="libgmp10 zlib1g")


class AssetRecipe(unittest.TestCase):
    def test_a_good_recipe_keeps_its_asset_and_family(self):
        got = checked(ASSET, ROOT)
        self.assertEqual((got["asset"], got["sha256"], got["sums"]), (ASSET["asset"], ASSET["sha256"], ASSET["sums"]))
        self.assertEqual((got["family"], got["needs"], got["mask"]), ("debian", "libgmp10 zlib1g", 0x0E))
        alone = checked({k: v for k, v in ASSET.items() if k not in ("family", "needs")}, ROOT)
        self.assertNotIn("family", alone)

    def test_the_asset_and_its_sums_are_https_files_of_one_release(self):
        for key, bad in (("asset", "http://github.com/x/y/z"), ("asset", AT + ".x"), ("sums", "ftp://x.org/a"),
                         ("sums", "https://example.org/v6.4.5/_sha256sums"), ("sums", ASSET["asset"]),
                         ("sums", AT[:-1] + "x/_sha256sums")):
            with self.assertRaises(ValueError, msg=bad):
                checked(dict(ASSET, **{key: bad}), ROOT)

    def test_the_sha256_is_64_lower_hex(self):
        for bad in ("", "ab" * 31, ASSET["sha256"].upper(), "g" * 64, ASSET["sha256"] + "0"):
            with self.assertRaises(ValueError, msg=bad):
                checked(dict(ASSET, sha256=bad), ROOT)

    def test_its_keys_are_an_assets_and_family_and_needs_go_together(self):
        gone = [{k: v for k, v in ASSET.items() if k != g} for g in ("sha256", "sums", "family", "needs", "source")]
        for bad in gone + [dict(ASSET, commit="0" * 40), dict(ASSET, build="x"), dict(ASSET, family="alpine"),
                           dict(ASSET, package="a" * 54), dict(ASSET, extra="x")]:
            with self.assertRaises(ValueError, msg=bad):
                checked(bad, ROOT)
        checked(dict(ASSET, package="a" * 53), ROOT)

    def test_needs_are_distinct_debian_names_space_separated(self):
        for bad in ("", "libgmp10  zlib1g", "libgmp10 libgmp10", "Libgmp10", "g", "-gmp", "libgmp10,zlib1g",
                    " ".join(f"lib{i}" for i in range(257))):
            with self.assertRaises(ValueError, msg=bad):
                checked(dict(ASSET, needs=bad), ROOT)
        checked(dict(ASSET, needs=" ".join(f"lib{i}" for i in range(256))), ROOT)
