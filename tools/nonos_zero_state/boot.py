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

"""One headless boot under the flake's QEMU, held at power-off by the
hold-at-off lane, driven over its control socket."""

import subprocess
import sys
import time
from pathlib import Path

ROOT = Path(__file__).resolve().parents[2]
sys.path.insert(0, str(ROOT / "userland/capsule_install/e2e"))
from qmp import Qmp  # noqa: E402

QMP = ROOT / "target/qemu/qmp.sock"
# Held at power-off for the dump, on the IOMMU lane the boot gate uses, with
# a keyboard and a pointer setup and the desktop answer.
LANES = ("hold-at-off", "iommu=intel", "usb-kbd", "usb-tablet")


class Boot:
    def __init__(self, image, serial, log, mem):
        serial.write_text("")
        QMP.unlink(missing_ok=True)
        args = ["--image", str(image), "--stick", "--tpm", "--headless", "--mem", mem,
                "--serial", str(serial)]
        for lane in LANES:
            args += ["--lane", lane]
        self.proc = subprocess.Popen(["nix", "run", ".#qemu", "--", *args], cwd=ROOT,
                                     stdout=open(log, "w"), stderr=subprocess.STDOUT)
        self.q = Qmp(str(QMP), wait=900)

    def dump(self, path):
        """The guest's RAM as it stands, to a file."""
        r = self.q.cmd("dump-guest-memory", paging=False, protocol=f"file:{path}")
        if "error" in r:
            raise RuntimeError(f"dump-guest-memory: {r['error']}")

    def held(self, secs):
        """True once the guest powered itself off and QEMU holds it there."""
        end = time.time() + secs
        while time.time() < end:
            status = self.q.cmd("query-status").get("return", {}).get("status")
            if status in ("shutdown", "paused"):
                return True
            time.sleep(1)
        return False

    def stop(self):
        self.q.quit()
        try:
            self.proc.wait(timeout=60)
        except subprocess.TimeoutExpired:
            self.proc.terminate()
