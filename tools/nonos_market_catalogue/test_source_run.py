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
"""A source listing comes from the NONOS mirror, held to the lock, its policy
naming its program; with no mirror it is left out, and a fault stops the run
by name. Run: python3 -m unittest discover -s tools/nonos_market_catalogue -t tools"""

import io
import tempfile
import unittest
from pathlib import Path
from unittest import mock

from nonos_linux_pack.policy import listing_id, release_id

from . import apk, pack_size
from .apk_fixture import ELF
from .digest import blake3
from .source_recipe import checked
from .source_run import source_entries
from .test_source_recipe import ROOT, SOURCE

KEY, RECIPE, MARKS = "11" * 32, checked(SOURCE, ROOT), {"darkfi-drk": ("", "")}


class SourceRun(unittest.TestCase):
    def run_in(self, d: Path, files: dict, update=False, mirror="10.0.2.2:8080"):
        for path, body in files.items():
            (d / "src/darkfi-drk" / path).parent.mkdir(parents=True, exist_ok=True)
            (d / "src/darkfi-drk" / path).write_bytes(body)
        return source_entries([dict(RECIPE, logo_hash="")], (mirror, d / "out", d / "src"), d / "s.lock.json",
                              MARKS, KEY, 7, update)

    def test_a_listing_from_the_mirror_whose_policy_names_the_program(self):
        with tempfile.TemporaryDirectory() as d:
            files = {"usr/bin/drk": ELF + b"drk", "usr/share/drk/a": b"a"}
            with self.assertRaisesRegex(ValueError, "darkfi-drk: not in the lock"):
                self.run_in(Path(d), files)
            (e,) = self.run_in(Path(d), {}, update=True)
            rel, b = e["releases"][0], e["capsule_id"]
            self.assertEqual(self.run_in(Path(d), {}), [e], "held to the lock it wrote")
            self.assertEqual((e["listing_id"], rel["package_hash"]), ("linux.nonos-darkfi-drk", b))
            self.assertEqual(rel["package_url"], f"http://10.0.2.2:8080/linux/{b}.tar")
            self.assertEqual(blake3((Path(d) / "out" / f"{b}.tar").read_bytes()), b)
            p, lid = bytes.fromhex(rel["ext"]["guest_policy"]), listing_id(bytes.fromhex(KEY), e["listing_id"])
            self.assertEqual(p[16:48].hex(), blake3(ELF + b"drk"))
            self.assertEqual((p[56:88], p[88:120]), (lid, release_id(lid, rel["release_id"])))
            with self.assertRaisesRegex(ValueError, "darkfi-drk: B is"):
                self.run_in(Path(d), {"usr/share/drk/a": b"moved"})

    def test_every_fault_stops_the_run_naming_the_package(self):
        # No program, not an ELF, another name first, past MAX_IMAGE or MAX_PACK (cut here), no content.
        for files in ({"usr/bin/other": ELF}, {"usr/bin/drk": b"#!/bin/sh"}, {"usr/bin/a": ELF, "usr/bin/drk": ELF},
                      {"usr/bin/drk": ELF + b"past"}, {"usr/bin/drk": ELF, "usr/share/x": bytes(20000)}, {}):
            with tempfile.TemporaryDirectory() as d, mock.patch.object(apk, "MAX_IMAGE", len(ELF)), mock.patch.object(
                    pack_size, "MAX_PACK", 1 << 14), self.assertRaisesRegex(ValueError, "nonos-darkfi-drk:", msg=files):
                self.run_in(Path(d), files, update=True)

    def test_with_no_mirror_nothing_is_listed_and_it_is_said(self):
        with tempfile.TemporaryDirectory() as d, mock.patch("sys.stderr", new_callable=io.StringIO) as said:
            self.assertEqual(self.run_in(Path(d), {"usr/bin/drk": ELF}, mirror=""), [])
            self.assertFalse((Path(d) / "out").exists() or (Path(d) / "s.lock.json").exists())
        self.assertIn("skip nonos-darkfi-drk: no NONOS package mirror", said.getvalue())
