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

"""The list in docs/security/privacy.md, read from its tables. A table is
known by its first header cell; each row's first cell holds one or more
`backticked` names. Prose around the tables is for people and is not read."""

import re

TICKED = re.compile(r"`([^`]+)`")
# First header cell: what that table's first column names.
KINDS = {
    "capsule": "capsules",
    "address": "places",
    "build setting": "places",
    "literal": "places",
}


def rows(text):
    """Each table as (first header cell, [first cell of each row])."""
    table, out = None, []
    for line in text.splitlines():
        cells = [c.strip() for c in line.strip().strip("|").split("|")]
        if not line.lstrip().startswith("|"):
            table = None
        elif table is None:
            table = (cells[0].lower(), [])
            out.append(table)
        elif not set(cells[0]) <= set("-: "):
            table[1].append(cells[0])
    return out


def read(text):
    """{"capsules": names, "places": literals}, as privacy.md lists them."""
    out = {"capsules": set(), "places": set()}
    for head, firsts in rows(text):
        kind = KINDS.get(head)
        for cell in firsts if kind else ():
            names = TICKED.findall(cell)
            if head == "build setting":
                names = ["build:" + n for n in names]
            out[kind].update(names)
    return out
