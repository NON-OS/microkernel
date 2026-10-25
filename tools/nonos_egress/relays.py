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
"""The relays a pinned authority's consensus names, read from the ingress
capture: every TCP stream an authority sent from its pinned (address, port)
is joined in capture order, its HTTP body inflated when it is zlib, and each
`r` line's address with its ORPort and DirPort taken. The ingress is what
the network sent the guest, so the guest cannot forge an authority in it."""

import re
import struct
import zlib
from collections import defaultdict

from .pcap import ip_text

RELAY_LINE = re.compile(
    rb"^r \S+ \S+ (?:\S+ )?\S+ \S+ (\d{1,3}\.\d{1,3}\.\d{1,3}\.\d{1,3}) (\d+) (\d+)", re.M
)


def relays(ingress_frames, authorities):
    """{(ip, port)} of the relays named, by ORPort and by DirPort when it has one."""
    streams = defaultdict(bytearray)
    for f in ingress_frames:
        if len(f) < 34 or f[12:14] != b"\x08\x00" or f[23] != 6:
            continue
        l4 = f[14 + (f[14] & 0x0F) * 4 :]
        if len(l4) < 20:
            continue
        sport, dport = struct.unpack_from(">HH", l4)
        src = (ip_text(f[26:30]), sport)
        if src not in authorities:
            continue
        streams[(src, dport)] += l4[(l4[12] >> 4) * 4 :]
    found = set()
    for raw in streams.values():
        body = bytes(raw).split(b"\r\n\r\n", 1)[-1]
        for text in (body, _inflate(body)):
            for m in RELAY_LINE.finditer(text):
                ip, orport, dirport = m.group(1).decode(), int(m.group(2)), int(m.group(3))
                found.add((ip, orport))
                if dirport:
                    found.add((ip, dirport))
    return found


def _inflate(body):
    try:
        return zlib.decompressobj().decompress(body)
    except zlib.error:
        return b""
