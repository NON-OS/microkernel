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
"""A download is bounded before it is read, written to the cache only whole,
and named when it fails, a cut transfer included. Run:
python3 -m unittest discover -s tools/nonos_market_catalogue -t tools"""

import http.client
import tempfile
import unittest
from pathlib import Path
from unittest import mock

from . import fetch


class Body:
    def __init__(self, data=b"", error=None):
        self.data, self.error = data, error

    def __enter__(self):
        return self

    def __exit__(self, *exc):
        return False

    def read(self, n):
        if self.error:
            raise self.error
        part, self.data = self.data[:n], self.data[n:]
        return part


class Fetch(unittest.TestCase):
    def test_a_download_is_capped_atomic_and_named_when_it_fails(self):
        with tempfile.TemporaryDirectory() as d:
            to = Path(d) / "f"
            with mock.patch.object(fetch.urllib.request, "urlopen", side_effect=lambda *a, **k: Body(b"12345")):
                with self.assertRaises(ValueError):
                    fetch.fetch("https://e.org/f", to, 4)
                self.assertFalse(to.exists() or to.with_name("f.part").exists())
                self.assertEqual(fetch.fetch("https://e.org/f", to, 5), b"12345")
            with mock.patch.object(fetch.urllib.request, "urlopen", return_value=Body(b"new")):
                self.assertEqual(fetch.fetch("https://e.org/f", to, 5, fresh=True), b"new")
            clock = iter(range(0, 10_000, 100))
            with mock.patch.object(fetch.urllib.request, "urlopen", return_value=Body(b"x" * 9)), \
                    mock.patch.object(fetch.time, "monotonic", lambda: next(clock)), \
                    mock.patch.object(fetch, "CHUNK", 1), self.assertRaises(ValueError):
                fetch.fetch("https://e.org/slow", Path(d) / "slow", 9, seconds=250)
            for error in (http.client.IncompleteRead(b"12"), OSError("reset")):
                with mock.patch.object(fetch.urllib.request, "urlopen", return_value=Body(error=error)), \
                        self.assertRaises(ValueError):
                    fetch.fetch("https://e.org/g", Path(d) / "g", 9)


if __name__ == "__main__":
    unittest.main()
