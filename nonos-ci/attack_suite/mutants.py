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


"""Mutants of passing logs: each takes one expectation away, and each must
then fail the check. A mutant that still passes is a rule that bites on
nothing, and fails the mutant run. Each is applied to every log on its own,
as the boots wrote them:

  - every attack's lines removed, its first refusal flipped to ESCAPED, its
    refusals emptied of how, and its errnos renamed;
  - every matrix row's needed refusal reworded so it no longer matches;
  - an escape said without the colon, and an attack line split mid-word;
  - the attacker's fault: its trap for another pid, the log ending at the
    trap, a kernel fault after it, and the fault replaced by a line that
    claims its refusal."""

import re

from .fault import FAULT, TRAP
from .judge import judge
from .mutant_edits import FORGED, at_trap, first, own, per_line
from .rules import ATTACKS, ROWS

def transforms():
    for name in ATTACKS:
        if name == "fault-containment":
            yield f"{name}: its fault marker removed", per_line(FAULT.search, lambda l: "")
            yield f"{name}: its fault replaced by a line claiming the refusal", \
                per_line(FAULT.search, lambda l: FORGED)
            continue
        yield f"{name}: its lines removed", per_line(own(name), lambda l: "")
        yield f"{name}: its first refusal flipped", first(own(name), lambda l: l.replace(" refused:", " ESCAPED:", 1))
        yield f"{name}: its refusals say not how", per_line(own(name), lambda l: l.split(" refused:", 1)[0] + " refused:\n")
        yield f"{name}: its errnos renamed", per_line(own(name), lambda l: re.sub(r"\bE[A-Z]+ \((\d+)\)", r"ENOENT (\1)", l))
    for row, name, needs in ROWS:
        for need in needs:
            if need != ".":
                yield (f"matrix row {row}: {need!r} reworded",
                       per_line(lambda l, n=name, p=need: own(n)(l) and re.search(p, l),
                                lambda l, p=need: re.sub(p, "<reworded>", l)))
    yield "an escape said without the colon", lambda t: t + "[ATTACK] cap-escape ESCAPED\n"
    yield "an attack line split mid-word", lambda t: t + "[ATTACK] cap-escape ESC\nAPED: x\n"
    yield "fault: the trap names another pid", per_line(TRAP.match, lambda l: re.sub(r"pid=0x[0-9a-f]+", "pid=0xffff", l))
    yield "fault: the log ends at the trap", at_trap
    yield "fault: the kernel faults after", lambda t: t + "[PANIC PF] fatal kernel fault, no recovery path rip=0x1\n" \
        if FAULT.search(t) else t


def mutants(logs):
    for desc, f in transforms():
        changed = [f(t) for t in logs]
        if changed != list(logs):
            yield desc, changed


def survivors(logs, boots=None):
    """The mutants that pass, and how many were run. Logs that do not pass
    themselves have no mutants to judge; that is the caller's failure."""
    run = list(mutants(logs))
    return [desc for desc, m in run if not judge(m, boots).bad], len(run)
