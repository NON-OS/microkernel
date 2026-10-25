#!/usr/bin/env python3
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
"""Boot the riscv64 kernel under QEMU virt with OpenSBI and hold its serial log
to how far the port goes today: the markers in order, ending at the named stop
where the VM layer is not ported yet, and no trap, panic or fatal line.

Usage: check_riscv64_boot.py --kernel <ELF> [--timeout SECONDS]
Exit 0 when the log holds; otherwise print what is missing or forbidden.
"""
import argparse
import subprocess
import sys
import tempfile

MARKERS = [
    "[MEM] phys init OK",
    "[NONOS] Microkernel init",
    "[RNG] seeded from:",
    "[VM] riscv64: the VM layer is not ported",
]
FORBIDDEN = ["KERNEL FATAL TRAP", "[FATAL]", "panicked", "PANIC"]


def check(log: str) -> list[str]:
    bad, at = [], 0
    for m in MARKERS:
        found = log.find(m, at)
        if found < 0:
            bad.append(f"missing, or out of order: {m!r}")
        else:
            at = found + len(m)
    bad += [f"forbidden: {f!r}" for f in FORBIDDEN if f in log]
    return bad


def main() -> int:
    ap = argparse.ArgumentParser()
    ap.add_argument("--kernel", required=True)
    ap.add_argument("--timeout", type=int, default=60)
    a = ap.parse_args()
    with tempfile.NamedTemporaryFile(suffix=".log") as serial:
        cmd = ["qemu-system-riscv64", "-M", "virt", "-cpu", "rv64", "-m", "512",
               "-display", "none", "-monitor", "none", "-serial", f"file:{serial.name}",
               "-bios", "default", "-kernel", a.kernel]
        try:
            subprocess.run(cmd, timeout=a.timeout, check=False)
        except subprocess.TimeoutExpired:
            pass
        log = open(serial.name, errors="replace").read()
    bad = check(log)
    for b in bad:
        print(f"check_riscv64_boot: {b}")
    print("check_riscv64_boot: " + ("FAIL" if bad else "the boot holds to the riscv64 cell"))
    return 1 if bad else 0


if __name__ == "__main__":
    sys.exit(main())
