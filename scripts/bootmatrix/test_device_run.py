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
"""The runner and a cell's scratch disk (run.py): made before the boot, named
on the QEMU line, and deleted after. QEMU is stood in for. Run from the
repository root: python3 -m unittest discover -s scripts -p 'test_*.py'."""

import tempfile
import unittest
from pathlib import Path
from unittest import mock

from bootmatrix import run
from bootmatrix.device_table import DEVICE_CELLS

NVME = next(c for c in DEVICE_CELLS if c.name == "q35-iommu-nvme")


class RunnerDisk(unittest.TestCase):
    def test_the_disk_is_on_the_line_and_gone_after(self):
        seen = {}

        def boot(argv, log, timeout, kill_at_serving=False, also=()):
            disk = next(w.split(",")[0][5:] for w in argv if w.endswith("id=celldisk,format=raw"))
            seen["disk"], seen["there"], seen["also"] = disk, Path(disk).exists(), also
            return "", True, "ready"

        with tempfile.TemporaryDirectory() as tmp:
            out = Path(tmp)
            for name in ("blk.img", "vars.fd"):
                (out / name).write_bytes(b"")
            paths = run.Paths("qemu", {"up": "esp"}, "code.fd", str(out / "vars.fd"), str(out / "blk.img"))
            with mock.patch.object(run, "boot", boot), mock.patch.object(run, "judge", lambda *a: []):
                result = run.one(NVME, paths, "tcg", out, 1, 1)
            self.assertTrue(seen["there"], "the disk was not made before the boot")
            self.assertTrue(seen["disk"].endswith("q35-iommu-nvme-1.device.img"), seen["disk"])
            self.assertFalse(Path(seen["disk"]).exists(), "the disk outlived the boot")
            self.assertEqual(seen["also"], NVME.expect)
            self.assertEqual(result.failures, [])


if __name__ == "__main__":
    unittest.main()
