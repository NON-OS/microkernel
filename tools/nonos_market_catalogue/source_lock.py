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
"""The source recipes' lock, linux-catalogue.sources.lock.json beside the
Alpine one (lock.py): per recipe the commit it was built from, the BLAKE3
and size of its bundled content tar, which its listing signs, and its
logo's BLAKE3s. A rebuild that does not give the pinned bytes stops the
seal by name, as a moved Alpine package does; only --update-lock moves it.
No file pins nothing, so every source recipe is then said unpinned."""

import json
from pathlib import Path

from . import logos
from .lock_file import HEX, MAX_BYTES

FORMAT = "nonos.linux-catalogue-sources-lock.v1"
PINNED = ("commit", "B", "S", "G", "R")


def path_of(recipes: Path) -> Path:
    return recipes.with_name(recipes.stem + ".sources.lock.json")


def read(path: Path) -> dict:
    """{package: {commit, B, S, G, R}}, {} with no file, or ValueError."""
    if not path.is_file():
        return {}
    raw = path.read_bytes()
    try:
        doc = json.loads(raw.decode("utf-8")) if len(raw) <= MAX_BYTES else None
    except (UnicodeDecodeError, json.JSONDecodeError) as why:
        raise ValueError(f"{path}: not JSON: {why}") from None
    if not isinstance(doc, dict) or doc.get("format") != FORMAT or set(doc) != {"format", "sources"} \
            or not isinstance(doc["sources"], dict):
        raise ValueError(f"{path}: not a {FORMAT} file")
    for name, pin in doc["sources"].items():
        if not isinstance(pin, dict) or set(pin) != set(PINNED) or not all(isinstance(v, str) for v in pin.values()):
            raise ValueError(f"{path}: {name!r} is not pinned by {', '.join(PINNED)}")
        wide = {"commit": (40,), "B": (64,), "G": (0, 64), "R": (0, 64)}
        if not all(len(pin[f]) in n and HEX >= set(pin[f]) for f, n in wide.items()) or not pin["S"].isdigit():
            raise ValueError(f"{path}: {name}: a pin is not a commit, BLAKE3 and size")
    return doc["sources"]


def held(pins: dict, built: dict, marks: dict) -> None:
    """ValueError naming every recipe whose commit or content is not the
    one locked; `built` is {package: {commit, B, S}} from this run."""
    moved = [f"{n}: {f} is {b[f]}, locked {pins[n][f]}" for n, b in sorted(built.items()) if n in pins
             for f in ("commit", "B", "S") if b[f] != pins[n][f]]
    said = moved + [f"{n}: not in the lock" for n in sorted(set(built) - set(pins))]
    said += [f"{n}: locked, no recipe" for n in sorted(set(pins) - set(built))]
    if said:
        raise ValueError("the sources lock does not hold; review, then --update-lock: " + "; ".join(said))
    logos.held(marks, pins)


def write(path: Path, built: dict, marks: dict) -> None:
    pins = {n: {**built[n], "G": marks[n][0], "R": marks[n][1]} for n in sorted(built)}
    rows = ",\n".join(f"  {json.dumps(n)}: {json.dumps(p)}" for n, p in pins.items())
    path.write_text('{"format": ' + json.dumps(FORMAT) + ', "sources": {\n' + rows + "\n}}\n")
