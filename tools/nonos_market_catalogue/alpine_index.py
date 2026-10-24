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
"""The signed package index (install/auth/signature.rs): its first member
signs its second under one of the three keys pinned in
userland/capsule_linux/keys/alpine/, which the device pins too, and only
then are its records read."""

import hashlib
from pathlib import Path

from . import gz, rsa, tarwalk

KEYS = Path(__file__).resolve().parents[2] / "userland/capsule_linux/keys/alpine"
NAMES = ("alpine-devel@lists.alpinelinux.org-4a6a0840.rsa.pub",
         "alpine-devel@lists.alpinelinux.org-5261cecb.rsa.pub",
         "alpine-devel@lists.alpinelinux.org-6165ee59.rsa.pub")


def signed_index(raw: bytes, verify=rsa.verify) -> bytes:
    """The index tar, if a pinned key signed it; else ValueError."""
    parts = gz.members(raw)
    if len(parts) != 2:
        raise ValueError("an index is exactly a signature and its index")
    covered = raw[parts[1].start:parts[1].end]
    for e in tarwalk.files(parts[0].body):
        for prefix, hashname in ((b".SIGN.RSA256.", "sha256"), (b".SIGN.RSA.", "sha1")):
            key = e.name[len(prefix):].decode("ascii", "replace") if e.name.startswith(prefix) else ""
            if key in NAMES and verify((KEYS / key).read_bytes(), e.body, hashname,
                                       hashlib.new(hashname, covered).digest()):
                return parts[1].body
    raise ValueError("the index is not signed by a key pinned here")


def records(index_tar: bytes) -> dict:
    """name -> record, from the APKINDEX file of a signed index tar."""
    found = [e for e in tarwalk.files(index_tar) if e.name == b"APKINDEX"]
    if not found:
        raise ValueError("the signed index holds no APKINDEX")
    out, rec = {}, {}
    for line in found[0].body.decode("utf-8", "replace").split("\n") + [""]:
        if not line:
            if rec.get("P"):
                out[rec["P"]] = rec
            rec = {}
        elif len(line) > 2 and line[1] == ":":
            rec[line[0]] = line[2:]
    return out
