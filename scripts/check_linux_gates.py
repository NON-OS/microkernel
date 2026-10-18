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
"""The gates between a Linux guest and what is outside it, held where they
are (userland/capsule_linux/abi/guest-gates.txt): each gate still in its
file, each pair in its order, every call refused on purpose EPERM or ENOSYS,
which the seal counts as refused, and no file but those listed answering a
guest's frame. Needs nothing built."""

import re, sys
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
LINUX = ROOT / "userland/capsule_linux/src/linux"
TABLE = ROOT / "userland/capsule_linux/abi/guest-gates.txt"

def check(lines):
    bad, replies, gates = [], set(), 0
    for line in lines:
        kind, _, rest = line.partition(" ")
        fields = rest.split(" | ")
        if kind == "reply":
            replies.add(fields[0])
            continue
        text = (LINUX / fields[0]).read_text()
        gates += 1
        if kind == "gate" and fields[1] not in text:
            bad.append(f"{fields[0]}: gone: {fields[1]} ({fields[2]})")
        elif kind == "first":
            at, later = text.find(fields[1]), text.find(fields[2])
            if at < 0 or later < 0 or at > later:
                bad.append(f"{fields[0]}: {fields[1]} no longer comes before {fields[2]}")
        elif kind != "gate":
            bad.append(f"guest-gates.txt: no such kind {kind}")
    found = {str(p.relative_to(LINUX)) for p in LINUX.rglob("*.rs")
             if "mk_foreign_reply" in p.read_text()}
    for f in sorted(found - replies):
        bad.append(f"{f}: answers a guest's frame and is not in guest-gates.txt")
    for f in sorted(replies - found):
        bad.append(f"{f}: listed as answering a frame, and does not")
    refused = (LINUX / "serve/refused.rs").read_text()
    named = re.findall(r"\((\d+), ([\w:]+), \"", refused)
    if not named:
        bad.append("serve/refused.rs: no refused call found by the pattern")
    for nr, code in named:
        if code not in ("PERM", "errno::EPERM", "errno::ENOSYS"):
            bad.append(f"serve/refused.rs: call {nr} is refused with {code}, not EPERM or ENOSYS")
    return bad, gates, len(replies)

def main() -> int:
    lines = [l for l in TABLE.read_text().splitlines() if l and not l.startswith("#")]
    bad, gates, replies = check(lines)
    for b in bad:
        print(f"guest gates: {b}", file=sys.stderr)
    if bad:
        return 1
    print(f"guest gates: {gates} held, {replies} reply sites")
    return 0

if __name__ == "__main__":
    sys.exit(main())
