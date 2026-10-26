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
"""Every rule here has a mutant in mutants.json, one exact substitution that
removes it. Each is applied to a copy of the tree and the suite must fail;
the unmutated copy must pass; a mutant that no longer applies exactly once
fails too, so none rots into testing nothing. Run:
python3 -m unittest discover -s tools/nonos_market_catalogue -t tools"""

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
MARKET = "userland/capsule_market/"
COPY = ("tools/nonos_market_catalogue", "tools/nonos_linux_pack", "tools/nix/store.json",
        *(MARKET + f for f in ("linux-catalogue.json", "linux-guests.json", "linux-tools.json")),
        "userland/capsule_linux/keys/alpine", "userland/capsule_linux/src/linux/install",
        "nonos-data/market/catalogue", "nonos-data/market/logos", "userland/capsule_market/logos",
        "userland/capsule_market/src/logos/table.rs",
        "userland/capsule_market/linux-catalogue.lock.json", "HANDOFF/catalogue/logos-licences.md",
        "nonos-data/market/index.json", "tools/devnet/market/index.json")
MUTANTS = json.loads((HERE / "mutants.json").read_text())


def suite(tree: Path) -> int:
    """The suite's exit status in `tree`, this test left out; a hang fails."""
    try:
        return subprocess.run([sys.executable, "-m", "unittest", "discover", "-s",
                               "tools/nonos_market_catalogue", "-t", "tools"], cwd=tree,
                              env=dict(os.environ, NONOS_IN_MUTANT="1"),
                              stdout=subprocess.DEVNULL, stderr=subprocess.DEVNULL, check=False, timeout=120).returncode
    except subprocess.TimeoutExpired:
        return 1


@unittest.skipIf(os.environ.get("NONOS_IN_MUTANT"), "inside a mutant run")
class Mutants(unittest.TestCase):
    def test_each_mutant_applies_once_and_the_suite_kills_it(self):
        with tempfile.TemporaryDirectory() as d:
            tree = Path(d)
            for rel in COPY:
                (tree / rel).parent.mkdir(parents=True, exist_ok=True)
                (shutil.copytree if (ROOT / rel).is_dir() else shutil.copyfile)(ROOT / rel, tree / rel)
            self.assertEqual(suite(tree), 0, "the unmutated copy fails")
            stale, survivors = [], []
            for m in MUTANTS:
                path = tree / "tools/nonos_market_catalogue" / m["file"]
                text = path.read_text()
                if text.count(m["old"]) != 1:
                    stale.append(m["name"])
                    continue
                path.write_text(text.replace(m["old"], m["new"]))
                survivors += [m["name"]] if suite(tree) == 0 else []
                path.write_text(text)
            self.assertEqual((stale, survivors), ([], []), "(no longer apply once, survive the suite)")


if __name__ == "__main__":
    unittest.main()
