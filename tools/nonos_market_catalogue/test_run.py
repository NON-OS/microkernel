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
"""The run stops on a fault that leaves nothing true to list, drops a cached
index that fails its signature, and refuses a package of another size than
the signed index gives. Run:
python3 -m unittest discover -s tools/nonos_market_catalogue -t tools"""

import tempfile
import unittest
from pathlib import Path
from unittest import mock

from . import run


class Run(unittest.TestCase):
    def test_a_missing_file_or_an_unsigned_index_stops_the_run(self):
        with tempfile.TemporaryDirectory() as d:
            cache = Path(d)
            with self.assertRaises(SystemExit):
                run.catalogue_run(cache / "none.json", cache, "https://e.org", "x86_64", "11" * 32, 1)
            (cache / "main-APKINDEX.tar.gz").write_bytes(b"not signed")
            with self.assertRaises(ValueError):
                run.table(cache, "https://e.org", "x86_64")
            self.assertFalse((cache / "main-APKINDEX.tar.gz").exists())

    def test_a_package_of_another_size_than_the_index_says_is_refused(self):
        with tempfile.TemporaryDirectory() as d, mock.patch.object(run, "fetch", return_value=b"abc"):
            rec = {"P": "x", "V": "1-r0", "S": "4", "base": "https://e.org"}
            with self.assertRaises(ValueError):
                run.apk_of(Path(d), rec)
            self.assertEqual(run.apk_of(Path(d), dict(rec, S="3")), b"abc")


if __name__ == "__main__":
    unittest.main()
