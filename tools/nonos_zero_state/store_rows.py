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

"""The store's table in a disk image, read and edited on the host, for the
tests that need a disk as an earlier boot would have left it: list the
records, or erase one where it lies (payload, then descriptor, all zeros),
as vfs does on a boot that keeps nothing."""

import struct

STORE_LBA, SECTOR = 256, 512
HEADER, ENTRY, NAME = 32, 128, 96


def _table(f):
    f.seek(STORE_LBA * SECTOR)
    head = f.read(HEADER)
    if head[:8] != b"NONOSTR1":
        raise ValueError("no NONOSTR1 store at LBA 256")
    count = struct.unpack_from("<I", head, 12)[0]
    return count, STORE_LBA * SECTOR + HEADER


def rows(image):
    """Each record: (slot, name, payload offset, length)."""
    with open(image, "rb") as f:
        count, base = _table(f)
        f.seek(base)
        table = f.read(ENTRY * count)
    out = []
    for slot in range(count):
        e = table[slot * ENTRY:(slot + 1) * ENTRY]
        if not any(e):
            continue
        name = e[:NAME].split(b"\0", 1)[0].decode("ascii", "replace")
        off, length = struct.unpack_from("<QQ", e, NAME)
        out.append((slot, name, off, length))
    return out


def erase(image, name):
    """Zero the record's payload, then its descriptor. False if absent."""
    hit = [r for r in rows(image) if r[1] == name]
    if not hit:
        return False
    slot, _, off, length = hit[0]
    with open(image, "r+b") as f:
        _, base = _table(f)
        f.seek(off)
        f.write(bytes(length))
        f.seek(base + slot * ENTRY)
        f.write(bytes(ENTRY))
    return True


def payload(image, name):
    """The record's bytes, or None when the store holds no such name."""
    hit = [r for r in rows(image) if r[1] == name]
    if not hit:
        return None
    with open(image, "rb") as f:
        f.seek(hit[0][2])
        return f.read(hit[0][3])
