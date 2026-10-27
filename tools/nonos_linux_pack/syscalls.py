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

"""syscalls --from-log: the set a program's runs measured, for policies.txt.

nonos.prove prints `[PROVE] syscall buckets ok=<130 hex digits>` for a sealed
family on a debug build: bit b at byte b/8, bit b%8, set when bucket
b had a served call. The set is the union over every line in every log named.
A program the vendor ships has no listing to prove, so a debug build's kernel
says `[MEASURE] app=<16 hex digits> ok=<130 hex digits>` as each family ends
(src/process/foreign/seal/measure_line.rs). With --app, the set is the union
of those lines for that app alone; without it, a log with none of the first
kind names the apps its [MEASURE] lines measured.
exit and exit_group are set, as capsule_linux always serves them, and bucket
512 is cleared, where a native call lands, so the set is market ready
(nonos_linux_policy::market, whose two constants test_syscalls pins).
"""

import argparse
import re
import sys
from pathlib import Path

LINE = re.compile(rb"\[PROVE\] syscall buckets ok=([0-9a-fA-F]{130})(?![0-9a-fA-F])")
MEASURE = re.compile(rb"\[MEASURE\] app=([0-9a-f]{16}) ok=([0-9a-fA-F]{130})(?![0-9a-fA-F])")
APP = re.compile(r"[0-9a-f]{16}")
SET_LEN = 65
ALWAYS_SERVED = (60, 231)
NATIVE_BUCKET = 512


def buckets(log: bytes, app=None) -> list:
    """Every set the log prints, or with `app` every set measured for that
    app. A bit past bucket 512 is not a set."""
    if app is None:
        found = [m.group(1) for m in LINE.finditer(log)]
    else:
        found = [m.group(2) for m in MEASURE.finditer(log) if m.group(1).decode() == app]
    sets = [bytes.fromhex(h.decode()) for h in found]
    if any(s[SET_LEN - 1] & 0xFE for s in sets):
        raise ValueError("a bit past bucket 512")
    return sets


def apps(log: bytes) -> dict:
    """Each app a [MEASURE] line names, and how many runs measured it."""
    seen = {}
    for m in MEASURE.finditer(log):
        seen[m.group(1).decode()] = seen.get(m.group(1).decode(), 0) + 1
    return seen


def market_set(sets) -> str:
    out = bytearray(SET_LEN)
    for s in sets:
        for i, b in enumerate(s):
            out[i] |= b
    for nr in ALWAYS_SERVED:
        out[nr // 8] |= 1 << (nr % 8)
    out[NATIVE_BUCKET // 8] &= ~(1 << (NATIVE_BUCKET % 8)) & 0xFF
    return out.hex()


def main(argv) -> int:
    ap = argparse.ArgumentParser(prog="nonos-linux-pack syscalls", description=__doc__,
                                 formatter_class=argparse.RawDescriptionHelpFormatter)
    ap.add_argument("--from-log", type=Path, action="append", required=True,
                    help="a serial log of the owner's debug run; repeatable, the sets unite")
    ap.add_argument("--app", help="the 16 hex digits a [MEASURE] line names its app by")
    args = ap.parse_args(argv)
    if args.app is not None and not APP.fullmatch(args.app):
        print("linux-pack: --app is 16 lowercase hex digits", file=sys.stderr)
        return 2
    sets = []
    for log in args.from_log:
        text = log.read_bytes()
        found = buckets(text, args.app)
        if not found:
            what = f"[MEASURE] line for app {args.app}" if args.app else "[PROVE] syscall buckets line"
            print(f"linux-pack: no {what} in {log}", file=sys.stderr)
            for app, runs in sorted(apps(text).items()):
                print(f"linux-pack: [MEASURE] app={app}: {runs} run(s)", file=sys.stderr)
            return 1
        sets.extend(found)
    print(f"calls={market_set(sets)}")
    return 0
