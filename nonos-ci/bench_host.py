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

import json
import os
import platform
import subprocess
import sys
import time


def main() -> int:
    out_dir = sys.argv[1] if len(sys.argv) > 1 else "target/bench"
    os.makedirs(out_dir, exist_ok=True)

    def run(*cmd: str) -> str:
        try:
            return subprocess.check_output(cmd, text=True, stderr=subprocess.DEVNULL).strip()
        except Exception:
            return "unavailable"

    data = {
        "schema": "nonos.bench.host.v1",
        "timestamp_utc": time.strftime("%Y-%m-%dT%H:%M:%SZ", time.gmtime()),
        "platform": platform.platform(),
        "machine": platform.machine(),
        "processor": platform.processor(),
        "python": sys.version.split()[0],
        "commit": run("git", "rev-parse", "HEAD"),
        "branch": run("git", "rev-parse", "--abbrev-ref", "HEAD"),
        "dirty": bool(run("git", "status", "--porcelain")),
        "rustc": run("rustc", "--version"),
        "cargo": run("cargo", "--version"),
        "qemu": run("qemu-system-x86_64", "--version").splitlines()[0],
        "cpu": run("/usr/sbin/sysctl", "-n", "machdep.cpu.brand_string"),
        "mem_bytes": run("/usr/sbin/sysctl", "-n", "hw.memsize"),
    }
    path = os.path.join(out_dir, "host.json")
    with open(path, "w", encoding="utf-8") as out:
        json.dump(data, out, indent=2, sort_keys=True)
        out.write("\n")
    print(path)
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
