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
"""Reading the catalogue's lock (lock.py): a versioned object of pins, one
a package, each its index record's version, checksum and size, its
.apk's BLAKE3, and its logo's source and raster BLAKE3s ("" for no logo). A file of any other format, shape or encoding is refused
whole, never read in part."""

import json
from pathlib import Path

from .names import package, version

FORMAT = "nonos.linux-catalogue-lock.v3"
FIELDS = ("V", "C", "S")  # the index record's
PINNED = FIELDS + ("B",)  # and the .apk's BLAKE3, hex
LOGO = ("G", "R")  # the logo's source and raster BLAKE3, hex, or ""
HEX = frozenset("0123456789abcdef")
MAX_BYTES = 1 << 20


def read(path: Path) -> dict:
    """{package: {V, C, S, B, G, R}}, or ValueError (TypeError for a wrong type)."""
    if not path.is_file():
        raise ValueError(f"{path}: no lock; make one with --update-lock and commit it")
    raw = path.read_bytes()
    if len(raw) > MAX_BYTES:
        raise ValueError(f"{path}: larger than {MAX_BYTES} bytes")
    try:
        doc = json.loads(raw.decode("utf-8"))
    except (UnicodeDecodeError, json.JSONDecodeError) as why:
        raise ValueError(f"{path}: not JSON: {why}") from None
    if not isinstance(doc, dict) or doc.get("format") != FORMAT or set(doc) != {"format", "packages"}:
        raise ValueError(f"{path}: not a {FORMAT} file")
    pins = doc["packages"]
    if not isinstance(pins, dict):
        raise TypeError(f"{path}: packages is not an object")
    for name, pin in pins.items():
        if package(name) != name or not isinstance(pin, dict) or set(pin) != set(PINNED + LOGO):
            raise ValueError(f"{path}: {name!r} is not pinned by {', '.join(PINNED + LOGO)}")
        if version(pin["V"]) != pin["V"] or not all(isinstance(pin[f], str) and pin[f] for f in PINNED):
            raise ValueError(f"{path}: {name}: a pin is not a version, checksum and size")
        if not all(isinstance(pin[f], str) and len(pin[f]) in (0, 64) and HEX >= set(pin[f]) for f in LOGO):
            raise ValueError(f"{path}: {name}: a logo pin is not a BLAKE3 in hex, or \"\"")
    return pins
