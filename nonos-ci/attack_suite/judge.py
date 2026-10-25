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


"""Pass only when every attack the judged boots carry was attempted and
refused, with "[ATTACK] <name> refused: <how>", no line anywhere says
ESCAPED, and every isolation matrix row has a refusal of its own. An attack
that never ran fails the run. Each boot's log is judged on its own, so one
boot's lines never stand in for another's."""

import collections
import re

from .attempts import attempts, notes as note_lines, stray_escapes
from .dropped import DROPPED
from .rules import ATTACKS, BOOTS, ROWS

Result = collections.namedtuple("Result", "attempts notes rows bad boots")


def rows(seen, expected):
    out = []
    for row, name, needs in ROWS:
        if name not in expected:
            continue
        hows = [a.how for a in seen.get(name, ()) if a.verdict == "refused"]
        out.append((row, [n for n in needs if not any(re.search(n, h) for h in hows)]))
    return out


def judge(logs, boots=None):
    """The verdict on the boots' serial logs, one string each; `boots` names
    which boots they are (rules.BOOTS), all of them by default."""
    logs = [logs] if isinstance(logs, str) else list(logs)
    boots = sorted(boots or BOOTS)
    expected = [a for a in ATTACKS if any(a in BOOTS[b] for b in boots)]
    found = [a for log in logs for a in attempts(log)]
    seen = collections.defaultdict(list)
    for a in found:
        seen[a.name].append(a)
    bad = [] if any(l.strip() for l in logs) else ["the log is empty: no boot's serial line was read"]
    bad += [f"{a.name}: ESCAPED: {a.how}" for a in found if a.verdict == "ESCAPED"]
    bad += [f"a line says ESCAPED or is no verdict: {l[:160]}" for log in logs for l in stray_escapes(log)]
    for name in expected:
        if name not in seen:
            bad.append(f"{name}: never attempted")
        elif not any(a.verdict == "refused" and a.how for a in seen[name]):
            bad.append(f"{name}: no refusal that says how")
    checked = rows(seen, expected)
    bad += [f"matrix row {row}: no refusal matching {n!r}" for row, missing in checked for n in missing]
    notes = [n for log in logs for n in note_lines(log)]
    notes += [f"{n}: not one of the suite's attacks" for n in sorted(seen) if n not in ATTACKS and n not in DROPPED]
    notes += [f"{n}: a dropped row was shown; put it back in rules.py" for n in sorted(seen) if n in DROPPED]
    return Result(found, notes, checked, bad, boots)
