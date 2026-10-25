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
"""The catalogue's lock: the version, checksum and size of each package the
recipes resolved to, and the BLAKE3 of its whole .apk, which its listing
signs, committed beside them, so a seal of a tree signs the same bytes
every time or stops. A fresh signed index that has moved past
the lock stops the seal by name, and only `--update-lock`, a maintainer's
deliberate run, moves the lock. A file of any other format, shape or
encoding is refused whole."""

import json
from pathlib import Path

from .lock_file import FIELDS, FORMAT, read

__all__ = ["bytes_held", "held", "read", "write"]


def held(known: dict, pins: dict, names: list) -> None:
    """ValueError naming every recipe the index no longer resolves to its pin."""
    moved = [f"{n}: locked {pins[n]['V']}, the index has {known[n]['V']}" for n in names
             if n in pins and n in known and any(known[n].get(f) != pins[n][f] for f in FIELDS)]
    unpinned = [n for n in names if n not in pins]
    gone = [n for n in names if n in pins and n not in known]
    if moved or unpinned or gone or set(pins) - set(names):
        said = moved + [f"{n}: not in the lock" for n in unpinned] + [f"{n}: not in the index" for n in gone]
        said += [f"{n}: locked, no recipe" for n in sorted(set(pins) - set(names))]
        raise ValueError("the lock does not hold; review, then --update-lock: " + "; ".join(said))


def apk(entry: dict) -> tuple:
    """(package, the BLAKE3 its listing signs) for a catalogue entry."""
    return entry["listing_id"].removeprefix("linux."), entry["releases"][0]["package_hash"]


def bytes_held(pins: dict, found: list) -> None:
    """ValueError naming every listing whose .apk is not the one locked."""
    moved = [f"{n}: the .apk is {b}, locked {pins[n]['B']}" for n, b in map(apk, found) if pins[n]["B"] != b]
    if moved:
        raise ValueError("the lock does not hold; review, then --update-lock: " + "; ".join(moved))


def write(path: Path, known: dict, names: list, found: list) -> None:
    signed = dict(map(apk, found))
    pins = {n: {**{f: known[n][f] for f in FIELDS}, "B": signed[n]} for n in sorted(names)}
    rows = ",\n".join(f"  {json.dumps(n)}: {json.dumps(p)}" for n, p in pins.items())
    path.write_text('{"format": ' + json.dumps(FORMAT) + ', "packages": {\n' + rows + "\n}}\n")
