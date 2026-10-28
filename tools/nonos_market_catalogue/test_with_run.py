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
"""A run finds each recipe's extras in the fresh signed index, stops when
one is missing, holds them to the lock, and an update pins them. Run:
python3 -m unittest discover -s tools/nonos_market_catalogue -t tools"""

import tempfile
import unittest
from contextlib import ExitStack
from pathlib import Path
from unittest import mock

from . import run

REC = {"P": "x", "V": "1-r0", "S": "1"}
RECIPE = {"package": "x", "with": ["y"]}
FOUND = [{"listing_id": "linux.x"}]


def ran(stack: ExitStack, known: dict, update: bool):
    """The run's mocks for one recipe adding `y`, and what it returned."""
    def patch(at, name, **how):
        return stack.enter_context(mock.patch.object(at, name, **how))

    table = patch(run, "table", return_value=known)
    patch(run, "load", return_value=[RECIPE])
    patch(run, "catalogue_entries", return_value=FOUND)
    patch(run, "source_entries", return_value=[])
    patch(run.logos, "digests", return_value=("", ""))
    patch(run.logos, "held")
    patch(run.lock, "read", return_value={})
    patch(run.lock, "bytes_held")
    held, write = patch(run.lock, "held"), patch(run.lock, "write")
    d = stack.enter_context(tempfile.TemporaryDirectory())
    out = run.catalogue_run(Path("x.json"), Path(d), "https://e.org", "v3.20", "x86_64", "11" * 32, 1, update)
    return table, held, write, out


class WithRun(unittest.TestCase):
    def test_an_extra_the_index_lacks_stops_the_run(self):
        with ExitStack() as stack, self.assertRaisesRegex(SystemExit, "lacks the extras y"):
            ran(stack, {"x": REC}, False)

    def test_extras_are_fetched_held_and_pinned(self):
        known = {"x": REC, "y": dict(REC, P="y")}
        with ExitStack() as stack:
            table, held, write, out = ran(stack, known, False)
            self.assertEqual((table.call_args.args[2], out), (["x", "y"], FOUND))
            self.assertEqual(held.call_args.args[2:], (["x"], ["y"]))
        with ExitStack() as stack:
            _, held, write, _ = ran(stack, known, True)
            self.assertEqual((held.called, write.call_args.args[2], write.call_args.args[5]), (False, ["x"], ["y"]))


if __name__ == "__main__":
    unittest.main()
