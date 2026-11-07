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

"""A package's tar cut into members the device can land: each member's
files make one pack (userland/nonos_linux_pack MAX_PACK) and its tar stays
within PART_RAW, so the install role decompresses and packs it inside its
heap. A cut falls only between entries, and an entry keeps the pax or
long-name records before it, so every member is the package's own bytes
in its own order, and the device reads each as it reads the package."""

from nonos_market_catalogue import pack_size

BLOCK = 512
END = b"\0" * (2 * BLOCK)
# The most one member's tar holds: what the device inflates one package to
# today (capsule_linux install/unpacked.rs MAX_INFLATED).
PART_RAW = 128 << 20
# Records that belong to the entry after them: pax, GNU long name and link.
CARRIED = (0x78, 0x67, 0x4C, 0x4B)


def entries(tar: bytes) -> list:
    """Each entry's bytes with the records before it, in archive order;
    ValueError for a tar the walk would stop in."""
    out, at, start = [], 0, None
    while at + BLOCK <= len(tar):
        head = tar[at:at + BLOCK]
        if head.count(0) == BLOCK:
            at += BLOCK
            continue
        field = head[124:136].split(b"\0")[0].strip()
        if not field or any(c not in b"01234567" for c in field):
            raise ValueError("a tar size that is not octal")
        size = int(field, 8)
        end = at + BLOCK + -(-size // BLOCK) * BLOCK
        if end > len(tar):
            raise ValueError("a tar entry runs past the end")
        start = at if start is None else start
        if head[156] not in CARRIED:
            out.append(tar[start:end])
            start = None
        at = end
    if start is not None:
        raise ValueError("a tar ends in a record with no entry after it")
    return out


def fits(tar: bytes, held: bool) -> bool:
    return pack_size.pack_len(tar, held) <= pack_size.MAX_PACK


def split(tar: bytes, held: bool) -> list:
    """The tar whole when one member holds it, else member tars that each
    end in two zero blocks.
    `held` says the package is the listed one, whose programs land beside
    its pack. ValueError when one entry alone cannot be packed."""
    parts, cur = [], b""
    for e in entries(tar):
        grown = cur + e
        if cur and (len(grown) + len(END) > PART_RAW or not fits(grown + END, held)):
            parts.append(cur + END)
            grown = e
        if len(grown) + len(END) > PART_RAW or not fits(grown + END, held):
            raise ValueError("one of its files is larger than a member holds")
        cur = grown
    if not parts:
        # Uncut, a package is its own tar, the bytes the device reads today.
        return [tar]
    parts.append(cur + END)
    if held:
        # The device records the listed package's program from one member.
        raise ValueError("the listed package needs more than one pack")
    return parts


def names(package: str, count: int) -> list:
    """`package`, then `package.p2`, `package.p3`... for its further parts."""
    return [package] + [f"{package}.p{i}" for i in range(2, count + 1)]
