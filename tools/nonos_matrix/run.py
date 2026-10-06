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
"""Run one command for a row, keep its whole output as the row's log, and read
the numbers a reader needs out of it: the exit status, the time, and the test
counts cargo prints."""

import re
import subprocess
import time

TEST = re.compile(r"^test result: (\w+)\. (\d+) passed; (\d+) failed", re.M)


def run(cmd, cwd, log, env=None, timeout=3600):
    t0 = time.time()
    try:
        r = subprocess.run(cmd, cwd=cwd, env=env, shell=True, capture_output=True, text=True, timeout=timeout)
        out, rc = r.stdout + r.stderr, r.returncode
    except subprocess.TimeoutExpired as e:
        out, rc = (e.stdout or b"").decode(errors="replace") + "\nTIMEOUT", 124
    with open(log, "a") as f:
        f.write(f"$ (cd {cwd} && {cmd})\n{out}\n[exit {rc}]\n\n")
    passed = sum(int(m[1]) for m in TEST.findall(out))
    failed = sum(int(m[2]) for m in TEST.findall(out))
    return {"cmd": cmd, "cwd": cwd, "rc": rc, "seconds": round(time.time() - t0, 1),
            "passed": passed, "failed": failed}
