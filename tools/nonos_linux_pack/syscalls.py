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
exit and exit_group are set, as capsule_linux always serves them, and bucket
512 is cleared, where a native call lands, so the set is market ready
(nonos_linux_policy::market, whose two constants test_syscalls pins).
"""

import argparse
import re
import sys
from pathlib import Path

LINE = re.compile(rb"\[PROVE\] syscall buckets ok=([0-9a-fA-F]{130})(?![0-9a-fA-F])")
SET_LEN = 65
ALWAYS_SERVED = (60, 231)
NATIVE_BUCKET = 512


def buckets(log: bytes) -> list:
    """Every set the log prints. A bit past bucket 512 is not a set."""
    sets = [bytes.fromhex(m.group(1).decode()) for m in LINE.finditer(log)]
    if any(s[SET_LEN - 1] & 0xFE for s in sets):
        raise ValueError("a bit past bucket 512")
    return sets


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
    args = ap.parse_args(argv)
    sets = []
    for log in args.from_log:
        found = buckets(log.read_bytes())
        if not found:
            print(f"linux-pack: no [PROVE] syscall buckets line in {log}", file=sys.stderr)
            return 1
        sets.extend(found)
    print(f"calls={market_set(sets)}")
    return 0
