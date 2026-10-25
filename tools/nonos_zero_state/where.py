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

"""Where in guest memory a secret sits, read from the ELF core QEMU's
dump-guest-memory writes, so a failed run names each copy's physical
address and what surrounds it, not only a count."""

import mmap
import struct

from .memscan import forms


def segments(m):
    """Each loaded segment of the core: (file offset, physical address, size)."""
    if m[:4] != b"\x7fELF":
        raise ValueError("not an ELF core")
    phoff = struct.unpack_from("<Q", m, 0x20)[0]
    size, count = struct.unpack_from("<HH", m, 0x36)
    out = []
    for i in range(count):
        kind, _, off, _, phys, fsize = struct.unpack_from("<IIQQQQ", m, phoff + i * size)
        if kind == 1:
            out.append((off, phys, fsize))
    return out


def hits(path, secret, around=32):
    """Each copy of the secret, in any form: (physical address, bytes around)."""
    with open(path, "rb") as f, mmap.mmap(f.fileno(), 0, access=mmap.ACCESS_READ) as m:
        segs = segments(m)
        out = []
        for needle in forms(secret):
            at = m.find(needle)
            while at != -1:
                phys = next((p + at - o for o, p, n in segs if o <= at < o + n), None)
                out.append((phys, bytes(m[max(0, at - around):at + len(needle) + around])))
                at = m.find(needle, at + 1)
        return out
