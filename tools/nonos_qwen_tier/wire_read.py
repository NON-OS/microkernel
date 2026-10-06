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
"""Reading a catalogue back, every length checked; see wire.py for the layout."""

import struct

from .wire import MAGIC, SIG


class _Reader:
    def __init__(self, blob):
        self.blob, self.at = blob, 0

    def take(self, n):
        if self.at + n > len(self.blob):
            raise ValueError("the catalogue ends early")
        self.at += n
        return self.blob[self.at - n:self.at]

    def int(self, fmt):
        return struct.unpack(fmt, self.take(struct.calcsize(fmt)))[0]

    def text(self, fmt):
        return self.take(self.int(fmt)).decode("ascii")


def decode(blob):
    """(serial, base, tiers, body, signature) of a catalogue; ValueError otherwise."""
    r = _Reader(blob[:-SIG] if len(blob) > SIG else b"")
    if r.take(8) != MAGIC:
        raise ValueError("not a NONOS model catalogue")
    serial, base, tiers = r.int("<Q"), r.text("<H"), []
    for _ in range(r.int("<H")):
        word, memory, files = r.text("<B"), r.int("<Q"), []
        for _ in range(r.int("<B")):
            name, length, sha = r.text("<B"), r.int("<Q"), r.take(32).hex()
            files.append((name, length, sha, [r.text("<H") for _ in range(r.int("<B"))]))
        tiers.append((word, memory, files))
    if r.at != len(r.blob):
        raise ValueError("bytes after the last tier")
    return serial, base, tiers, r.blob, blob[-SIG:]
