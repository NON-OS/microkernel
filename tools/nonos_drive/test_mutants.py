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

"""Each typing rule of qmp.py, removed (mutants.json), must fail test_keys.py.
Each mutant is applied to a copy of the two files and the keys suite must
fail; the unmutated copy must pass; a mutant that no longer applies exactly
once fails too, so none rots into testing nothing."""

import json
import os
import shutil
import subprocess
import sys
import tempfile
import unittest
from pathlib import Path

HERE = Path(__file__).resolve().parent
ROOT = HERE.parents[1]
COPY = ("userland/capsule_install/e2e/qmp.py", "tools/nonos_drive/__init__.py",
        "tools/nonos_drive/test_keys.py")
MUTANTS = json.loads((HERE / "mutants.json").read_text())


def keys_suite(tree: Path) -> int:
    """test_keys.py's exit status in `tree`; a hang fails."""
    try:
        return subprocess.run([sys.executable, "-m", "unittest", "tools.nonos_drive.test_keys"],
                              cwd=tree, stdout=subprocess.DEVNULL, stderr=subprocess.DEVNULL,
                              check=False, timeout=60).returncode
    except subprocess.TimeoutExpired:
        return 1


class Mutants(unittest.TestCase):
    def test_each_mutant_applies_once_and_the_keys_suite_kills_it(self):
        with tempfile.TemporaryDirectory() as d:
            tree = Path(d)
            for rel in COPY:
                (tree / rel).parent.mkdir(parents=True, exist_ok=True)
                shutil.copyfile(ROOT / rel, tree / rel)
            (tree / "tools/__init__.py").touch()
            self.assertEqual(keys_suite(tree), 0, "the unmutated copy fails")
            stale, survivors = [], []
            for m in MUTANTS:
                path = tree / m["file"]
                text = path.read_text()
                if text.count(m["old"]) != 1:
                    stale.append(m["name"])
                    continue
                path.write_text(text.replace(m["old"], m["new"]))
                survivors += [m["name"]] if keys_suite(tree) == 0 else []
                path.write_text(text)
            self.assertEqual((stale, survivors), ([], []), "(no longer apply once, survive)")


if __name__ == "__main__":
    unittest.main()
