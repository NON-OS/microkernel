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
"""Classic pcap, link type Ethernet, read whole and strictly: a record cut
short, a record longer than the snap length, 256 KiB or its own original
length, or bytes left over that do not make a record header, make the file
Unreadable. A damaged capture is never judged on the part that parsed."""

import struct

MAX_RECORD = 0x40000


class Unreadable(Exception):
    pass


def frames(data):
    """Every frame of the capture, or Unreadable."""
    if len(data) < 24:
        raise Unreadable("shorter than a pcap header")
    magic = struct.unpack_from("<I", data)[0]
    if magic in (0xA1B2C3D4, 0xA1B23C4D):
        endian = "<"
    elif magic in (0xD4C3B2A1, 0x4D3CB2A1):
        endian = ">"
    else:
        raise Unreadable("not a pcap file")
    snaplen, linktype = struct.unpack_from(endian + "II", data, 16)
    if linktype != 1:
        raise Unreadable(f"link type {linktype}, not Ethernet")
    out, off = [], 24
    while off < len(data):
        if off + 16 > len(data):
            raise Unreadable(f"a record header cut short at byte {off}")
        incl, orig = struct.unpack_from(endian + "II", data, off + 8)
        if incl > MAX_RECORD or (snaplen and incl > snaplen) or incl > orig:
            raise Unreadable(f"record at byte {off} claims {incl} bytes")
        if off + 16 + incl > len(data):
            raise Unreadable(f"record at byte {off} runs past the end")
        out.append(data[off + 16 : off + 16 + incl])
        off += 16 + incl
    return out


def mac_text(m):
    return ":".join(f"{b:02x}" for b in m)


def ip_text(b):
    return ".".join(str(x) for x in b)
