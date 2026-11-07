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
"""nonos-bringup-gate: the device bring-up seal gate.

    nonos-bringup-gate boots --boots 20 --laptop <dir or logs...>
    nonos-bringup-gate boots --boots 1 <dir or logs...>
    nonos-bringup-gate trace <before.log> <after.log>

A directory's *.log files are taken in name order as the boot order. Prints
each fault and PASS or FAIL; exits 0 on PASS, 1 on FAIL, 2 on bad usage."""

import argparse
import os
import sys

from .gate import LAPTOP, run_faults, trace_faults


def logs_of(paths):
    out = []
    for p in paths:
        if os.path.isdir(p):
            names = sorted(f for f in os.listdir(p) if f.endswith(".log"))
            out += [(os.path.join(p, f), os.path.join(p, f)) for f in names]
        else:
            out.append((p, p))
    read = []
    for name, path in out:
        with open(path, encoding="utf-8", errors="replace") as fh:
            read.append((name, fh.read()))
    return read


def main(argv=None):
    ap = argparse.ArgumentParser(prog="nonos-bringup-gate")
    sub = ap.add_subparsers(dest="cmd", required=True)
    b = sub.add_parser("boots")
    b.add_argument("--boots", type=int, default=1)
    b.add_argument("--laptop", action="store_true", help="battery, AC, lid, keyboard, touchpad ... up")
    b.add_argument("--require", default="", help="more classes that must be up, comma separated")
    b.add_argument("paths", nargs="+")
    t = sub.add_parser("trace")
    t.add_argument("before")
    t.add_argument("after")
    a = ap.parse_args(argv)
    if a.cmd == "boots":
        need = tuple(LAPTOP if a.laptop else ()) + tuple(c for c in a.require.split(",") if c)
        logs = logs_of(a.paths)
        faults = run_faults(logs, a.boots, need)
        counted = len(logs)
    else:
        (_, before), (_, after) = logs_of([a.before, a.after])
        faults = trace_faults(before, after)
        counted = 2
    for f in faults:
        print(f)
    print(f"{'PASS' if not faults else 'FAIL'}: {counted} log(s), {len(faults)} fault(s)")
    return 0 if not faults else 1


if __name__ == "__main__":
    sys.exit(main())
