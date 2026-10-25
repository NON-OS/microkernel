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
"""The records of a signed index, read as the device reads them
(install/get/index.rs, install/index.rs): every APKINDEX entry of both
branches joined, main first; lines as Rust's `str::lines`; a record kept
only with a name and a version; the listed package the first of its exact
name, never one that only provides it."""

from . import tarwalk
from .rust_text import lines


def text(index_tar: bytes) -> bytes:
    """Every entry whose name ends in APKINDEX, joined, as get/index.rs does."""
    return b"".join(e.body for e in tarwalk.files(index_tar) if e.name.endswith(b"APKINDEX"))


def rest(line: str) -> str:
    """`line.get(2..)`: past the second byte, "" where that is no boundary."""
    b = line.encode()
    try:
        return b[2:].decode() if len(b) >= 2 and b[:2].decode() else ""
    except UnicodeDecodeError:
        return ""


def records(raw: bytes) -> list:
    """The records, in order, as Index::parse reads them: a blank line ends
    one, and one without both a name and a version is dropped."""
    out, rec = [], {}
    for line in lines(raw.decode("utf-8", "replace")) + [""]:
        if not line:
            if rec.get("P") and rec.get("V"):
                out.append(rec)
            rec = {}
        elif line[0] in "CPVDpLS":
            rec[line[0]] = rest(line)
    return out


def listed(recs: list, name: str):
    """The first record of exactly this name (Index::listed), or None."""
    return next((r for r in recs if r["P"] == name), None)
