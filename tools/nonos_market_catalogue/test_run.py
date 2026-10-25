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
"""The run refuses any release or arch but the device's, stops on an index
that fails its signature, takes a package from the first branch that
serves it, and refuses, and drops, one of another size than the signed
index gives. Run:
python3 -m unittest discover -s tools/nonos_market_catalogue -t tools"""

import re
import tempfile
import unittest
from pathlib import Path
from unittest import mock

from . import run, sources

RUN_RS = Path(__file__).resolve().parents[2] / "userland/capsule_linux/src/linux/install/run.rs"


class Run(unittest.TestCase):
    def test_release_arch_and_branches_are_the_device_s(self):
        rs = RUN_RS.read_text()
        self.assertIn(f'RELEASE: &str = "{sources.RELEASE}";', rs)
        self.assertIn(f'ARCH: &str = "{sources.ARCH}";', rs)
        self.assertEqual(re.search(r'BRANCHES: \[&str; 2\] = \["(\w+)", "(\w+)"\]', rs).groups(), sources.BRANCHES)

    def test_another_release_or_an_unsigned_index_stops_the_run(self):
        with tempfile.TemporaryDirectory() as d:
            for release, arch in (("v3.19", "x86_64"), ("v3.20", "aarch64")):
                with self.assertRaises(SystemExit):
                    run.catalogue_run(Path("none"), Path(d), "https://e.org", release, arch, "11" * 32, 1)
            with mock.patch.object(sources, "fetch", return_value=b"not signed"), self.assertRaises(ValueError):
                sources.table("https://e.org", Path(d), ["x"])

    def test_the_first_branch_that_serves_it_and_only_its_signed_size(self):
        rec = {"P": "x", "V": "1-r0", "S": "3", "bases": ["https://e.org/main", "https://e.org/community"]}
        served = mock.Mock(side_effect=[ValueError("404"), b"abc"])
        with tempfile.TemporaryDirectory() as d, mock.patch.object(sources, "fetch", served):
            self.assertEqual(sources.apk_of(Path(d), dict(rec)), b"abc")
        with tempfile.TemporaryDirectory() as d, mock.patch.object(sources, "fetch", return_value=b"abc"):
            got = dict(rec)
            sources.apk_of(Path(d), got)
            self.assertEqual(got["base"], "https://e.org/main")
            at = sources.cached(Path(d), "main", rec)
            at.parent.mkdir(parents=True, exist_ok=True)
            at.write_bytes(b"abc")
            with self.assertRaises(ValueError):
                sources.apk_of(Path(d), dict(rec, S="4"))
            self.assertFalse(at.exists())
        with tempfile.TemporaryDirectory() as d, mock.patch.object(sources, "fetch", side_effect=ValueError("x")), \
                self.assertRaises(ValueError):
            sources.apk_of(Path(d), dict(rec))


if __name__ == "__main__":
    unittest.main()
