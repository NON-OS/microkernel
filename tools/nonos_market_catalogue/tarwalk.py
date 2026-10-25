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
"""A tar walked exactly as the installer walks one (install/tar.rs,
tar_kind.rs, tar_field.rs, tar_path.rs): zero blocks skipped, a size that
is not octal or runs past the end stops the walk, no checksum read, pax and
GNU long names honoured, `./` and a trailing `/` cut from every name."""

from typing import NamedTuple

from . import tarpax
from .tarfield import cstr, member, name_of, octal

BLOCK = 512


class Entry(NamedTuple):
    name: bytes
    kind: str
    body: bytes


KINDS = {0x30: "file", 0: "file", 0x37: "file", 0x31: "hardlink", 0x32: "symlink", 0x35: "dir"}


def walk(data: bytes) -> list:
    """Every entry, in archive order; records and dropped kinds are not."""
    out, nxt, at = [], tarpax.Overrides(), 0
    while at + BLOCK <= len(data):
        head = data[at:at + BLOCK]
        if head.count(0) == BLOCK:  # a zero block ends an archive; an apk is several
            at += BLOCK
            continue
        size = octal(head[124:136])
        if size is None or at + BLOCK + size > len(data):
            break
        body, flag = data[at + BLOCK:at + BLOCK + size], head[156]
        if flag == 0x78:
            tarpax.read(body, nxt)
        elif flag in (0x4C, 0x4B):
            setattr(nxt, "path" if flag == 0x4C else "link", cstr(body))
        elif flag in KINDS:
            kind = KINDS[flag]
            out.append(Entry(member(nxt.path if nxt.path is not None else name_of(head)), kind,
                             body if kind == "file" else b""))
            nxt = tarpax.Overrides()
        elif flag != 0x67:
            nxt = tarpax.Overrides()
        at += BLOCK + -(-size // BLOCK) * BLOCK
    return out


def files(data: bytes) -> list:
    """Regular files only, as tar::entries gives them."""
    return [e for e in walk(data) if e.kind == "file"]
