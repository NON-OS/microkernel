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
"""An asset listing sits in Debian's family, named by its release, and the
sources lock pins the asset's URL, sha256, BLAKE3 and checksum file; a
program that is not the published binary, or pins that do not hold the
needs, stop the run by name. Run: python3 -m unittest discover -s tools/nonos_market_catalogue -t tools"""

import json
import tempfile
import unittest
from pathlib import Path

from .asset_needs import DIR
from .digest import blake3
from .source_recipe import checked
from .source_run import source_entries
from .test_asset_recipe import ASSET, PROGRAM, ROOT

KEY, MARKS = "11" * 32, {"simplex-chat": ("", "")}
SUM = "ab" * 32
PINS = f"libgmp10 2:6.3.0+dfsg-3 pool/main/g/gmp/libgmp10_6.3.0+dfsg-3_amd64.deb {SUM}\n" \
       f"zlib1g 1:1.3.dfsg-3 pool/main/z/zlib/zlib1g_1.3.dfsg-3_amd64.deb {SUM}\n".encode()
GOOD = {"usr/bin/simplex-chat": PROGRAM, DIR + "simplex-chat": PINS}


def run_in(d: Path, files: dict, recipe=ASSET, update=False):
    for path, body in files.items():
        (d / "src/simplex-chat" / path).parent.mkdir(parents=True, exist_ok=True)
        (d / "src/simplex-chat" / path).write_bytes(body)
    return source_entries([dict(checked(recipe, ROOT), logo_hash="")], ("10.0.2.2:8080", d / "out", d / "src"),
                          d / "s.lock.json", MARKS, KEY, 7, update)


class SourceAsset(unittest.TestCase):
    def test_an_asset_is_listed_in_debians_family_and_pinned_by_its_digests(self):
        with tempfile.TemporaryDirectory() as d:
            (e,) = run_in(Path(d), GOOD, update=True)
            rel = e["releases"][0]
            self.assertEqual((e["listing_id"], rel["release_id"]), ("linux.kali.nonos-simplex-chat", "nonos-simplex-chat@v6.4.5"))
            self.assertIn(f"upstream's release binary {ASSET['asset']}", json.dumps(rel["validation"]))
            pin = json.loads((Path(d) / "s.lock.json").read_text())["sources"]["simplex-chat"]
            self.assertEqual((pin["asset"], pin["sha256"], pin["sums"]), (ASSET["asset"], ASSET["sha256"], ASSET["sums"]))
            self.assertEqual((pin["A"], pin["B"]), (blake3(PROGRAM), e["capsule_id"]))
            self.assertNotIn("commit", pin)
            self.assertEqual(run_in(Path(d), {}), [e], "held to the lock it wrote")
            with self.assertRaisesRegex(ValueError, "simplex-chat: sums is"):
                run_in(Path(d), {}, recipe=dict(ASSET, sums=ASSET["sums"] + "2"))

    def test_a_program_that_is_not_the_published_binary_is_refused(self):
        with tempfile.TemporaryDirectory() as d, self.assertRaisesRegex(ValueError, "not the published asset's"):
            run_in(Path(d), dict(GOOD, **{"usr/bin/simplex-chat": PROGRAM + b"!"}), update=True)

    def test_pins_must_hold_every_need_and_only_a_family_has_them(self):
        no_family = {k: v for k, v in ASSET.items() if k not in ("family", "needs")}
        for files, recipe in (({"usr/bin/simplex-chat": PROGRAM}, ASSET),
                              (dict(GOOD, **{DIR + "simplex-chat": PINS.split(b"\n")[0]}), ASSET),
                              (GOOD, no_family), (dict(GOOD, **{DIR + "simplex-chat": b"x y\n"}), ASSET)):
            with tempfile.TemporaryDirectory() as d, self.assertRaisesRegex(ValueError, "nonos-simplex-chat:"):
                run_in(Path(d), files, recipe, update=True)
        with tempfile.TemporaryDirectory() as d:
            (e,) = run_in(Path(d), {"usr/bin/simplex-chat": PROGRAM}, no_family, update=True)
            self.assertEqual(e["listing_id"], "linux.nonos-simplex-chat")
