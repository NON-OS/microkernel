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
"""The model repository catalogue as bytes, as the fetcher reads it.

    "NXQWEN01" | serial u64 | NONOS mirror base: u16 length, bytes |
    tiers u16 | each tier: word (u8 length, bytes), memory u64, files u8 |
    each file: name (u8 length, bytes), length u64, SHA-256 (32 bytes),
    mirrors u8, each mirror: u16 length, URL | Ed25519 signature (64)

Integers are little-endian; the signature covers every byte before it.
"""

import struct

MAGIC = b"NXQWEN01"
SIG = 64


def _field(text, width):
    b = text.encode("ascii")
    return struct.pack("<B" if width == 1 else "<H", len(b)) + b


def encode(serial, base, tiers):
    """The unsigned body; `tiers` is [(word, memory, [(name, length, sha, urls)])]."""
    out = [MAGIC, struct.pack("<Q", serial), _field(base, 2), struct.pack("<H", len(tiers))]
    for word, memory, files in tiers:
        out += [_field(word, 1), struct.pack("<QB", memory, len(files))]
        for name, length, sha, urls in files:
            out += [_field(name, 1), struct.pack("<Q", length), bytes.fromhex(sha)]
            out += [struct.pack("<B", len(urls))] + [_field(u, 2) for u in urls]
    return b"".join(out)
