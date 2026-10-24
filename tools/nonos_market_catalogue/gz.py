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
"""A gzip file cut into its members, as the installer's inflater cuts it
(userland/inflate/src/members.rs, members_within): at most 64 members, all
of them together inflating to at most 128 MiB (install/unpacked.rs,
MAX_INFLATED), and no byte that belongs to no member."""

import zlib
from typing import NamedTuple

MAX_MEMBERS = 64
MAX_INFLATED = 128 << 20


class Member(NamedTuple):
    start: int
    end: int
    body: bytes


def members(raw: bytes, limit: int = MAX_INFLATED) -> list:
    """Every member of `raw`, or ValueError. Output is bounded before it is
    made: each member may inflate only to what the limit has left."""
    view, out, at, total = memoryview(raw), [], 0, 0
    while at < len(raw):
        if len(out) == MAX_MEMBERS:
            raise ValueError(f"more than {MAX_MEMBERS} gzip members")
        z = zlib.decompressobj(31)
        try:
            body = z.decompress(view[at:], limit + 1 - total)
        except zlib.error as why:
            raise ValueError(f"not gzip at byte {at}: {why}") from None
        total += len(body)
        if total > limit or not z.eof:
            raise ValueError("a member is cut short or inflates past the limit")
        end = len(raw) - len(z.unused_data)
        out.append(Member(at, end, body))
        at = end
    if not out:
        raise ValueError("no gzip member")
    return out
