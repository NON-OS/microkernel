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
"""Drive one QEMU across many commands: start it in the background, then
shoot, press keys, type, wait for serial text, and stop it. The machine is
the one run.py starts (vm.py); its state lives in --out."""

import argparse
import os
import subprocess
import sys

from qmp import Qmp
from steps import step
from vm import qemu_cmd, start_swtpm


def start(a):
    os.makedirs(a.out, exist_ok=True)
    serial, sock = os.path.join(a.out, "serial.log"), os.path.join(a.out, "qmp.sock")
    tpm = start_swtpm(a.out) if a.tpm else None
    log = open(os.path.join(a.out, "qemu.log"), "w")
    vm = subprocess.Popen(qemu_cmd(a, serial, sock, tpm), stdout=log, stderr=log, start_new_session=True)
    open(os.path.join(a.out, "qemu.pid"), "w").write(str(vm.pid))
    print(f"started pid {vm.pid}")


def main():
    ap = argparse.ArgumentParser(description=__doc__)
    ap.add_argument("--out", required=True)
    sub = ap.add_subparsers(dest="cmd", required=True)
    s = sub.add_parser("start")
    s.add_argument("--esp")
    s.add_argument("--disk", action="append", default=[])
    s.add_argument("--res", default="")
    s.add_argument("--tpm", action="store_true")
    s.add_argument("--window", action="store_true", help="an SDL window, for recording")
    s.add_argument("--mem", default="2048")
    s.add_argument("--vars", required=True)
    d = sub.add_parser("do", help="steps as run.py takes them")
    d.add_argument("steps", nargs="+")
    sub.add_parser("stop")
    a = ap.parse_args()
    if a.cmd == "start":
        return start(a)
    q = Qmp(os.path.join(a.out, "qmp.sock"), wait=5)
    if a.cmd == "stop":
        return q.quit()
    ok = all([step(q, s, a.out, os.path.join(a.out, "serial.log")) for s in a.steps])
    sys.exit(0 if ok else 1)


if __name__ == "__main__":
    main()
