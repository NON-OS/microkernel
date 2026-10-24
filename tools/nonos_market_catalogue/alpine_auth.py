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
"""Whether Alpine published a package, checked as the installer checks it
(install/auth/package.rs): its control member hashes to the signed index's
`C:`, and its data, from the first data member to the end, hashes to the
`datahash` the control records. Only the data members are its files."""

import base64
import hashlib

from . import gz, tarwalk
from .rust_text import lines, trim


def checksum(field) -> bytes:
    """The 20 bytes of an index `C:` field, `Q1` then base64, or ValueError."""
    if not isinstance(field, str) or not field.startswith("Q1"):
        raise ValueError("the index gives no Q1 checksum")
    raw = base64.b64decode(field[2:], validate=True)
    if len(raw) != 20:
        raise ValueError("the index checksum is not 20 bytes")
    return raw


def datahash(control_tar: bytes) -> bytes:
    """pkginfo.rs's datahash: strict UTF-8, Rust's lines, the first line with
    the prefix, Rust's trim, 64 lowercase hex digits."""
    found = [e for e in tarwalk.files(control_tar) if e.name == b".PKGINFO"]
    body = lines(found[0].body.decode("utf-8")) if found else []
    text = next((trim(x[len("datahash = "):]) for x in body if x.startswith("datahash = ")), "")
    if len(text) != 64 or any(c not in "0123456789abcdef" for c in text):
        raise ValueError("the package records no datahash")
    return bytes.fromhex(text)


def package_files(apk: bytes, want: bytes) -> bytes:
    """The data members' tar, joined, if the chain holds; else ValueError."""
    parts = gz.members(apk)
    if len(parts) < 3:
        raise ValueError("a package is a signature, a control and data")
    control, data = parts[1], parts[2:]
    if hashlib.sha1(apk[control.start:control.end]).digest() != want:
        raise ValueError("its control member is not the one the signed index names")
    if hashlib.sha256(apk[data[0].start:]).digest() != datahash(control.body):
        raise ValueError("its data is not the data its control member records")
    return b"".join(p.body for p in data)

