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

import argparse
import json
import os
import platform
import resource
import subprocess
import time


def main() -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument("name")
    parser.add_argument("out")
    parser.add_argument("cmd", nargs=argparse.REMAINDER)
    args = parser.parse_args()
    cmd = args.cmd[1:] if args.cmd[:1] == ["--"] else args.cmd
    if not cmd:
        raise SystemExit("missing command")
    os.makedirs(args.out, exist_ok=True)
    log_path = os.path.join(args.out, f"{args.name}.log")
    json_path = os.path.join(args.out, f"{args.name}.json")
    before = resource.getrusage(resource.RUSAGE_CHILDREN)
    started = time.perf_counter_ns()
    with open(log_path, "w", encoding="utf-8", errors="replace") as log:
        proc = subprocess.run(cmd, stdout=log, stderr=subprocess.STDOUT)
    ended = time.perf_counter_ns()
    after = resource.getrusage(resource.RUSAGE_CHILDREN)
    elapsed_ms = (ended - started) // 1_000_000
    user_ms = int((after.ru_utime - before.ru_utime) * 1000)
    sys_ms = int((after.ru_stime - before.ru_stime) * 1000)
    max_rss = max(0, after.ru_maxrss)
    if platform.system() == "Darwin":
        max_rss //= 1024
    data = {
        "name": args.name,
        "status": "pass" if proc.returncode == 0 else "fail",
        "exit_code": proc.returncode,
        "elapsed_ms": elapsed_ms,
        "user_cpu_ms": user_ms,
        "system_cpu_ms": sys_ms,
        "max_rss_kb": max_rss,
        "command": cmd,
        "log": log_path,
    }
    with open(json_path, "w", encoding="utf-8") as out:
        json.dump(data, out, indent=2, sort_keys=True)
        out.write("\n")
    print(json_path)
    return proc.returncode


if __name__ == "__main__":
    raise SystemExit(main())
