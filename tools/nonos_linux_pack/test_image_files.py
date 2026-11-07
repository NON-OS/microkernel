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

"""Files an image ships: every regular file of the tree at its path, one
member named by its content, a link or an empty tree refused."""

import os
import tempfile
import unittest
from pathlib import Path

from nonos_market_catalogue.tarwalk import walk

from . import image_files

ROOT = Path(__file__).resolve().parents[2]
LO = ROOT / "userland/capsule_market/image-files/libreoffice"


class Files(unittest.TestCase):
    def test_libreoffices_work_path_ships_where_it_reads_its_registry(self):
        m = image_files.member(LO)
        names = [e.name for e in walk(m.tar)]
        self.assertEqual(names, [b"usr/lib/libreoffice/share/registry/nonos-documents.xcd"])
        self.assertIn(b"$(home)/Documents", walk(m.tar)[0].body)
        self.assertTrue(m.name.startswith(b"nonos-files-"))
        self.assertEqual(image_files.member(LO), m, "the same tree, the same member")

    def test_a_link_or_an_empty_tree_is_refused(self):
        with tempfile.TemporaryDirectory() as d:
            with self.assertRaises(ValueError):
                image_files.files(Path(d))
            (Path(d) / "a").write_bytes(b"x")
            os.symlink("a", Path(d) / "b")
            with self.assertRaises(ValueError):
                image_files.files(Path(d))
