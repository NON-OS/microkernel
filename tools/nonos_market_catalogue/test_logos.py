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
"""The logos the tree commits agree with the record, the lock and the
documents (logos.py): a recipe names a logo only where its row allows it,
its source hashes to G, its raster to R, and its document signs R; a recipe
without one has "" for both, a zero logo_hash and no raster. Each rule also
refuses on its own, on a tree made here. Run:
python3 -m unittest discover -s tools/nonos_market_catalogue -t tools"""

import json
import math
import re
import tempfile
import unittest
from pathlib import Path

from . import logos
from .digest import blake3
from .lock_file import read
from .recipes import checked
from .recipes_file import load

ROOT = Path(__file__).resolve().parents[2]
RECIPES = load(ROOT / "userland/capsule_market/linux-catalogue.json", ROOT)
PINS = read(ROOT / "userland/capsule_market/linux-catalogue.lock.json")
RULED = f"UNCLEAR; {logos.RULING}"
RECORD = "\n".join(("# r", "## Table", "| App | a | b | c | Verdict |", "|---|---|---|---|---|",
                    "| c | y | y | y | CLEAR, with conditions |", "| u | y | y | y | UNCLEAR |",
                    f"| r | y | y | y | {RULED} |", "| n | n/a | n/a | n/a | NO LOGO (NONOS-drawn) |",
                    "## Clauses", "| u | y | y | y | CLEAR |"))
BASE = {"licence": "MIT", "mask": "TTY", "name": "X", "text": "x", "logo": "l.svg"}


def document(package):
    (e,) = json.loads((ROOT / f"nonos-data/market/catalogue/linux.{package}.json").read_text())["entries"]
    return e


class Committed(unittest.TestCase):
    def test_every_logo_is_allowed_vendored_rastered_locked_and_signed(self):
        rows = logos.verdicts((ROOT / logos.RECORD).read_text())
        for r in RECIPES:
            n, pin, at = r["package"], PINS[r["package"]], logos.raster_of(ROOT, r["package"])
            if not r.get("logo"):
                self.assertEqual((pin["G"], pin["R"], document(n)["logo_hash"]), ("", "", ""), n)
                self.assertFalse(at.exists(), n)
                continue
            self.assertTrue(logos.may_show(rows[n]), n)
            raster = at.read_bytes()
            self.assertEqual(blake3((ROOT / r["logo"]).read_bytes()), pin["G"], n)
            self.assertEqual(blake3(raster), pin["R"], n)
            self.assertEqual(document(n)["logo_hash"], pin["R"], n)
            side = math.isqrt(len(raster) // 4)
            self.assertTrue(len(raster) == 4 * side * side and 0 < side <= logos.OWN_MAX, n)

    def test_no_raster_is_committed_or_embedded_for_a_listing_that_has_no_logo(self):
        names = {logos.raster_of(ROOT, r["package"]).name for r in RECIPES if r.get("logo")}
        self.assertEqual({p.name for p in (ROOT / logos.RASTERS).iterdir()}, names)
        table = (ROOT / "userland/capsule_market/src/logos/table.rs").read_text()
        self.assertEqual({f"{n}.rgba" for n in re.findall(r'logo!\("([\w.-]+)"\)', table)}, names)


class Rules(unittest.TestCase):
    def test_a_row_shows_a_logo_only_when_clear_or_ruled(self):
        rows = logos.verdicts(RECORD)
        self.assertEqual(sorted(rows), ["c", "n", "r", "u"])
        self.assertEqual([logos.may_show(rows[a]) for a in "curn"], [True, False, True, False])

    def test_a_recipe_names_a_logo_only_where_the_record_allows(self):
        with tempfile.TemporaryDirectory() as d:
            root = Path(d) / "tree"
            (root / logos.RECORD).parent.mkdir(parents=True)
            (root / logos.RECORD).write_text(RECORD)
            for at in (root / "l.svg", Path(d) / "l.svg"):  # the second outside the tree
                at.write_text("<svg/>")
            for ok in "cr":
                self.assertEqual(checked(dict(BASE, package=ok), root)["logo"], "l.svg")
            for bad in (dict(BASE, package="u"), dict(BASE, package="n"), dict(BASE, package="z"),
                        dict(BASE, package="c", logo="none.svg"), dict(BASE, package="c", logo="../l.svg")):
                with self.assertRaises(ValueError, msg=bad):
                    checked(bad, root)

    def test_the_lock_holds_both_digests_and_a_logo_less_recipe_carries_no_bytes(self):
        pins = {"x": {"G": "aa" * 32, "R": "bb" * 32}, "y": {"G": "", "R": ""}}
        logos.held({"x": ("aa" * 32, "bb" * 32), "y": ("", "")}, pins)
        for marks, said in ((("cc" * 32, "bb" * 32), "logo source"), (("aa" * 32, "cc" * 32), "logo raster")):
            with self.assertRaises(ValueError) as stop:
                logos.held({"x": marks}, pins)
            self.assertIn(said, str(stop.exception))
        with tempfile.TemporaryDirectory() as d:
            root = Path(d)
            self.assertEqual(logos.digests({"package": "y"}, root), ("", ""))
            logos.raster_of(root, "y").parent.mkdir(parents=True)
            logos.raster_of(root, "y").write_bytes(bytes(16))
            with self.assertRaises(ValueError):
                logos.digests({"package": "y"}, root)
            with self.assertRaises(ValueError):
                logos.digests({"package": "x", "logo": "x.svg"}, root)


if __name__ == "__main__":
    unittest.main()
