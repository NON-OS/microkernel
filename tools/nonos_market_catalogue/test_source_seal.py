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
"""A catalogue run takes source recipes out of the Alpine path: none is
looked up in Alpine's index or lock, none listed with no NONOS mirror, and
with one, a source recipe that fails stops the run by name. Run:
python3 -m unittest discover -s tools/nonos_market_catalogue -t tools"""

import io
import tempfile
import unittest
from pathlib import Path
from unittest import mock

from . import run
from .apk_fixture import ELF
from .source_recipe import checked
from .test_source_recipe import ROOT, SOURCE

ALPINE = {"package": "x", "licence": "MIT", "mask": 0x100, "dests": None, "name": "X", "text": "x"}


def seal(cache: Path, site: tuple, update=False) -> tuple:
    """(entries, the names Alpine's index was asked for)."""
    asked = []
    with mock.patch.object(run, "load", return_value=[checked(SOURCE, ROOT), ALPINE]), \
            mock.patch.object(run, "table", side_effect=lambda m, c, names: asked.extend(names) or {}), \
            mock.patch.object(run, "catalogue_entries", return_value=[{"listing_id": "linux.x"}]), \
            mock.patch.object(run.lock, "read", return_value={}), mock.patch.object(run.lock, "held"), \
            mock.patch.object(run.lock, "bytes_held"), mock.patch.object(run.lock, "write"), \
            mock.patch("sys.stderr", new_callable=io.StringIO):
        got = run.catalogue_run(cache / "c.json", cache, "https://e.org", "v3.20", "x86_64", "11" * 32, 1,
                                update, site)
    return [e["listing_id"] for e in got], asked


class SourceSeal(unittest.TestCase):
    def test_with_no_mirror_the_alpine_recipes_alone_are_listed(self):
        with tempfile.TemporaryDirectory() as d:
            self.assertEqual(seal(Path(d), ("", None, None)), (["linux.x"], ["x"]))

    def test_with_a_mirror_each_source_recipe_is_listed_or_stops_the_run(self):
        with tempfile.TemporaryDirectory() as d:
            at = Path(d)
            with self.assertRaisesRegex(SystemExit, "nonos-darkfi-drk: no --sources"):
                seal(at, ("10.0.2.2:8080", at / "out", None))
            (at / "src/darkfi-drk/usr/bin").mkdir(parents=True)
            (at / "src/darkfi-drk/usr/bin/drk").write_bytes(ELF)
            site = ("10.0.2.2:8080", at / "out", at / "src")
            with self.assertRaisesRegex(SystemExit, "darkfi-drk: not in the lock"):
                seal(at, site)
            self.assertEqual(seal(at, site, update=True), (["linux.x", "linux.nonos-darkfi-drk"], ["x"]))
            self.assertTrue((at / "c.sources.lock.json").is_file())
            self.assertEqual(seal(at, site)[0], ["linux.x", "linux.nonos-darkfi-drk"])
