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


"""The edits a mutant makes to one log, line by line."""

from .fault import TRAP

FORGED = "[ATTACK] fault-containment refused: the kernel ended pid 41 at its fault on 0x0, and 9 lines followed\n"


def own(name):
    return lambda l: f"[ATTACK] {name} " in l


def per_line(keep, change):
    """A transform of one log: `change` each line `keep` picks."""
    return lambda t: "".join(change(l) if keep(l) else l for l in t.splitlines(keepends=True))


def first(keep, change):
    def f(t):
        lines = t.splitlines(keepends=True)
        i = next((i for i, l in enumerate(lines) if keep(l)), None)
        return t if i is None else "".join(lines[:i] + [change(lines[i])] + lines[i + 1:])
    return f


def at_trap(t):
    lines = t.splitlines(keepends=True)
    i = next((i for i, l in enumerate(lines) if TRAP.match(l)), None)
    return t if i is None else "".join(lines[:i + 1])
