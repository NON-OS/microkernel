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
"""Boot an ESP folder or a disk under QEMU and OVMF, headless, run a list of
steps against it over QMP, and keep the serial log. A step is wait:SECONDS,
shot:NAME, key:QCODE[+QCODE], type:TEXT or until:TEXT[:SECONDS], which waits
for TEXT on the serial log."""

import argparse
import os
import subprocess

from qmp import Qmp
from steps import step
from vm import qemu_cmd, start_swtpm


def main():
    ap = argparse.ArgumentParser(description=__doc__)
    ap.add_argument("--esp", help="a folder to boot as a FAT ESP")
    ap.add_argument("--disk", action="append", default=[], help="FILE[,nvme|virtio], repeatable")
    ap.add_argument("--out", required=True, help="where screenshots and the serial log go")
    ap.add_argument("--res", default="", help="add a WxH mode to the VGA device")
    ap.add_argument("--tpm", action="store_true", help="attach a swtpm TPM 2.0 (CRB)")
    ap.add_argument("--mem", default="2048")
    ap.add_argument("--vars", required=True, help="a writable OVMF_VARS copy")
    ap.add_argument("steps", nargs="*")
    a = ap.parse_args()
    os.makedirs(a.out, exist_ok=True)
    serial, sock = os.path.join(a.out, "serial.log"), os.path.join(a.out, "qmp.sock")
    tpm = start_swtpm(a.out) if a.tpm else None
    vm = subprocess.Popen(qemu_cmd(a, serial, sock, tpm), stdout=subprocess.DEVNULL)
    q, ok = Qmp(sock), True
    try:
        for s in a.steps:
            ok = step(q, s, a.out, serial) and ok
    except (OSError, ConnectionError) as e:
        print(f"the machine went away during the steps: {e}", flush=True)
        ok = False
    finally:
        q.quit()
        vm.wait(timeout=30)
    raise SystemExit(0 if ok else 1)


if __name__ == "__main__":
    main()
