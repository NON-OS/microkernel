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
"""The seal's market step and the source recipes' content: a release seal
stops on content nothing gives and never asks the catalogue to leave a
recipe out; a development seal asks both to leave it out, said by name; with
linux_sources empty neither is asked for it."""

import unittest
from pathlib import Path
from unittest import mock

from nonos_seal import capsules, market


class Step:
    def __init__(self):
        self.calls = []

    def run(self, *args, env=None):
        self.calls.append([str(a) for a in args])


def seal(development: bool, sources: str = "target/linux-sources", gives=None):
    """(the index tool's arguments, absent_ok ensure was called with)."""
    asked, step = [], Step()

    def ensure(out, absent_ok=False):
        asked.append(absent_ok)
        if gives is not None:
            raise gives
        return []

    with mock.patch.object(capsules, "DEVELOPMENT", development), \
            mock.patch.object(market.os.path, "isfile", return_value=True), \
            mock.patch.object(market, "place_userland"), mock.patch.object(market, "linux_sources", ensure), \
            mock.patch.object(market.dev_market, "chosen", return_value=False), \
            mock.patch.object(market.market_logos, "agree"), mock.patch.object(market.keys, "stage"), \
            mock.patch.object(market.os, "makedirs"):
        market.seal(step, mock.Mock(market_index=Path("idx")), None, "1", "", "mirror:80", "out", True, sources)
    (index,) = [c for c in step.calls if "tools/nonos-market-index" in c]
    return index, asked


class SealStep(unittest.TestCase):
    def test_a_release_never_asks_the_catalogue_to_leave_a_recipe_out(self):
        index, asked = seal(False)
        self.assertEqual(asked, [False])
        self.assertIn("--sources", index)
        self.assertNotIn("--sources-absent-ok", index)

    def test_a_release_stops_on_content_nothing_gives(self):
        with self.assertRaisesRegex(SystemExit, "the source recipes' content: demo: no cache"):
            seal(False, gives=ValueError("demo: no cache"))

    def test_a_development_seal_leaves_absent_content_out(self):
        index, asked = seal(True)
        self.assertEqual(asked, [True])
        self.assertIn("--sources-absent-ok", index)

    def test_with_linux_sources_empty_neither_is_asked(self):
        for development in (False, True):
            index, asked = seal(development, sources="")
            self.assertEqual(asked, [])
            self.assertFalse({"--sources", "--sources-absent-ok"} & set(index))


if __name__ == "__main__":
    unittest.main()
