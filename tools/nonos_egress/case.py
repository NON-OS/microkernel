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

"""The scaffolding each audit test stands on: a fresh small tree, an edit
that must apply exactly once, and a run that must name what it finds."""

import tempfile
import unittest
from pathlib import Path

from . import tree
from .__main__ import PAGES, audit

REPO = Path(__file__).resolve().parents[2]


def run(root):
    pages = sorted((root / "docs/security").glob(PAGES))
    return audit(root, "\n".join(p.read_text() for p in pages))[2]


class Case(unittest.TestCase):
    def setUp(self):
        self.dir = tempfile.TemporaryDirectory()
        self.root = tree.make(Path(self.dir.name))

    def tearDown(self):
        self.dir.cleanup()

    def edit(self, rel, old, new):
        p = self.root / rel
        text = p.read_text()
        self.assertEqual(text.count(old), 1, rel)
        p.write_text(text.replace(old, new))

    def caught(self, *words):
        bad = run(self.root)
        self.assertTrue(any(all(w in b for w in words) for b in bad), bad)
