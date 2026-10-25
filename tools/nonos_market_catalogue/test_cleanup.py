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
"""A package that fails stops the run, and is dropped from every branch's
cache; the cleanup never follows a hostile version out of it. Run:
python3 -m unittest discover -s tools/nonos_market_catalogue -t tools"""

import tempfile
import unittest
from pathlib import Path
from unittest import mock

from . import run, sources


class Cleanup(unittest.TestCase):
    def test_the_cleanup_never_follows_a_hostile_version_out_of_the_cache(self):
        with tempfile.TemporaryDirectory() as d:
            cache = Path(d) / "catalogue" / "v3.20" / "x86_64"
            for branch in ("main", "community"):
                (cache / branch / "x-1").mkdir(parents=True)
            kept = cache / "evil.apk"  # where <branch>/x-1/../../evil.apk lands
            kept.write_bytes(b"not the catalogue's")
            rec = {"P": "x", "V": "1/../../evil", "S": "1"}
            with mock.patch.object(run, "load", return_value=[{"package": "x"}]), \
                    mock.patch.object(run, "table", return_value={"x": rec}), \
                    mock.patch.object(run, "catalogue_entries", return_value=[]), \
                    mock.patch.object(run.lock, "read", return_value={}), mock.patch.object(run.lock, "held"), \
                    self.assertRaises(SystemExit):
                run.catalogue_run(Path("x.json"), Path(d), "https://e.org", "v3.20", "x86_64", "11" * 32, 1)
            self.assertTrue(kept.exists())

    def test_a_package_left_out_is_dropped_from_every_branch_cached(self):
        with tempfile.TemporaryDirectory() as d:
            cache = Path(d) / "catalogue" / "v3.20" / "x86_64"
            rec = {"P": "x", "V": "1-r0", "S": "1"}
            held = [sources.cached(cache, b, rec) for b in ("main", "community")]
            for at in held:
                at.parent.mkdir(parents=True)
                at.write_bytes(b"x")
            with mock.patch.object(run, "load", return_value=[{"package": "x"}]), \
                    mock.patch.object(run, "table", return_value={"x": rec}), \
                    mock.patch.object(run, "catalogue_entries", return_value=[]), \
                    mock.patch.object(run.lock, "read", return_value={}), mock.patch.object(run.lock, "held"), \
                    self.assertRaises(SystemExit):
                run.catalogue_run(Path("x.json"), Path(d), "https://e.org", "v3.20", "x86_64", "11" * 32, 1)
            self.assertEqual([at.exists() for at in held], [False, False])


if __name__ == "__main__":
    unittest.main()
