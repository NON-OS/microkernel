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
"""The list of what no lane runs yet, one `<path> <class> <owner> <reason>`
line each, and tools/nix/test-lanes.json, the markers as nix reads them."""

import json
from pathlib import Path

BASELINE = Path("scripts/baselines/untested-crates.txt")
LANES = Path("tools/nix/test-lanes.json")
CLASSES = ("a", "t", "c", "d", "e", "orphan")
OWNERS = ("A", "B", "C", "D", "lead", "upstream")
WRITE = "run scripts/check_test_lanes.py --write-lanes"


def read(root):
    """{path: (class, owner)}, and a problem for each line that is not well formed."""
    entries, bad = {}, []
    path = root / BASELINE
    for n, line in enumerate(path.read_text().splitlines() if path.exists() else [], 1):
        if not line.strip() or line.startswith("#"):
            continue
        f = line.split(None, 3)
        why = ("needs <path> <class> <owner> <reason>" if len(f) < 4 else
               f"class {f[1]!r} is not one of {' '.join(CLASSES)}" if f[1] not in CLASSES else
               f"owner {f[2]!r} is not one of {' '.join(OWNERS)}" if f[2] not in OWNERS else
               f"{f[0]} is listed twice" if f[0] in entries else None)
        if why:
            bad.append(f"{BASELINE}:{n}: {why}")
        else:
            entries[f[0]] = (f[1], f[2])
    return entries, bad


def render(marked):
    """test-lanes.json for the markers: a sorted object of dir to test-args."""
    return json.dumps(marked, indent=1, sort_keys=True) + "\n"


def drift(root, marked):
    """Each way tools/nix/test-lanes.json differs from what the markers say."""
    path = root / LANES
    text = path.read_text() if path.exists() else None
    if text == render(marked):
        return []
    try:
        have = json.loads(text) if text is not None else None
    except ValueError:
        have = None
    if not isinstance(have, dict):
        return [f"{LANES} is missing or not a JSON object; {WRITE}"]
    out = [f"{LANES} names {d}, whose manifest asks for no lane" for d in sorted(set(have) - set(marked))]
    out += [f"{LANES} lacks {d}, whose manifest asks for the lane" for d in sorted(set(marked) - set(have))]
    out += [f"{LANES} gives {d} test-args {have[d]!r}, its manifest {marked[d]!r}"
            for d in sorted(set(have) & set(marked)) if have[d] != marked[d]]
    return [f"{p}; {WRITE}" for p in out or [f"{LANES} matches the markers but not byte for byte"]]
