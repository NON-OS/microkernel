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
"""The index is fetched fresh every run and read through the listed name,
so gvim never stands in for vim; a cached file larger than its cap is
fetched again; the run refuses another release by name, and its cleanup
never follows a hostile version out of the cache. Run:
python3 -m unittest discover -s tools/nonos_market_catalogue -t tools"""

import tempfile
import unittest
from pathlib import Path
from unittest import mock

from . import fetch, run, sources
from .apk_fixture import tar
from .test_fetch import Body

GVIM = tar([("APKINDEX", b"P:gvim\nV:9-r0\np:vim=9-r0\nS:1\n\nP:vim\nV:9-r0\nS:2\n\n")])


class Sources(unittest.TestCase):
    def test_the_index_is_fetched_fresh_and_read_by_the_listed_name(self):
        got = mock.Mock(return_value=b"raw")
        with tempfile.TemporaryDirectory() as d, mock.patch.object(sources, "fetch", got), \
                mock.patch.object(sources, "signed_index", return_value=GVIM):
            known = sources.table("https://e.org", Path(d), ["vim", "ghost"])
        self.assertTrue(all(c.kwargs.get("fresh") for c in got.call_args_list))
        self.assertEqual((known["vim"]["P"], known["vim"]["S"]), ("vim", "2"))
        self.assertNotIn("ghost", known)

    def test_a_cached_file_past_its_cap_is_fetched_again(self):
        with tempfile.TemporaryDirectory() as d:
            (Path(d) / "f").write_bytes(b"123456")
            with mock.patch.object(fetch.urllib.request, "urlopen", side_effect=lambda *a, **k: Body(b"new")):
                self.assertEqual(fetch.fetch("https://e.org/f", Path(d) / "f", 5), b"new")


class Run(unittest.TestCase):
    def test_another_release_is_refused_by_name(self):
        with self.assertRaises(SystemExit) as stop:
            run.catalogue_run(Path("x.json"), Path("."), "https://e.org", "v3.19", "x86_64", "11" * 32, 1)
        self.assertIn("installs from Alpine v3.20", str(stop.exception.code))

    def test_the_cleanup_never_follows_a_hostile_version_out_of_the_cache(self):
        with tempfile.TemporaryDirectory() as d:
            cache = Path(d) / "catalogue" / "v3.20" / "x86_64"
            (cache / "x-1").mkdir(parents=True)
            kept = cache.parent / "evil.apk"
            kept.write_bytes(b"not the catalogue's")
            rec = {"P": "x", "V": "1/../../evil", "S": "1"}
            with mock.patch.object(run, "load", return_value=[{"package": "x"}]), \
                    mock.patch.object(run, "table", return_value={"x": rec}), \
                    mock.patch.object(run, "catalogue_entries", return_value=[]):
                run.catalogue_run(Path("x.json"), Path(d), "https://e.org", "v3.20", "x86_64", "11" * 32, 1)
            self.assertTrue(kept.exists())


if __name__ == "__main__":
    unittest.main()
