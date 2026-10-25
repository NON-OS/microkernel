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

"""The parts file read, one row a part, with the two columns the tree gives."""

import pathlib
import sys

from coverage_facts import ROOT, in_table

PARTS = ROOT / "docs/hardware/coverage-parts.tsv"
FIELDS = ("area", "part", "years", "common", "ids", "table", "feature", "owner", "priority", "gap")


def rows():
    out = []
    for n, line in enumerate(PARTS.read_text().splitlines(), 1):
        if not line.strip() or line.startswith("#"):
            continue
        cells = line.split("\t")
        if len(cells) != len(FIELDS):
            sys.exit(f"{PARTS}:{n}: {len(cells)} fields, {len(FIELDS)} wanted")
        out.append(dict(zip(FIELDS, cells)))
    return out


def sealed(rel):
    """Whether a capsule driver's trust files are committed; listed is not signed."""
    parts = pathlib.Path(rel).parts
    if len(parts) < 2 or not parts[1].startswith("capsule_"):
        return True
    name = parts[1][len("capsule_"):]
    return (ROOT / f"nonos-data/trust/capsules/{name}.manifest.bin").is_file()


def facts(row, features):
    ids = [i for i in row["ids"].split() if i != "-"]
    row["matched"] = in_table(ids, row["table"])
    feature = row["feature"]
    if feature == "-":
        row["image"] = "no feature"
    elif feature == "kernel":
        row["image"] = "kernel"
    elif feature in features and not sealed(row["table"]):
        row["image"] = "listed, unsealed"
    else:
        row["image"] = "yes" if feature in features else "no"
    return row
