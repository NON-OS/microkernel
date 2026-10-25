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
"""What a frame's payload says: a DHCP message's hardware address and
options, a TLS ClientHello's server name, a plaintext HTTP request's Host."""

import struct

MAGIC = b"\x63\x82\x53\x63"


def dhcp(udp_payload):
    """(chaddr, {code: bytes}) of a BOOTP/DHCP message, or None."""
    p = udp_payload
    if len(p) < 240 or p[236:240] != MAGIC:
        return None
    chaddr = p[28:34] if p[2] == 6 else p[28 : 28 + min(p[2], 16)]
    opts, i = {}, 240
    while i < len(p):
        code = p[i]
        if code == 255:
            break
        if code == 0:
            i += 1
            continue
        if i + 1 >= len(p):
            break
        n = p[i + 1]
        opts[code] = p[i + 2 : i + 2 + n]
        i += 2 + n
    return chaddr, opts


def tls_sni(payload):
    """The server name of a TLS ClientHello at the start of `payload`, or None."""
    p = payload
    if len(p) < 43 or p[0] != 0x16 or p[5] != 0x01:
        return None
    try:
        i = 5 + 4 + 2 + 32
        i += 1 + p[i]
        i += 2 + struct.unpack_from(">H", p, i)[0]
        i += 1 + p[i]
        end = i + 2 + struct.unpack_from(">H", p, i)[0]
        i += 2
        while i + 4 <= min(end, len(p)):
            kind, n = struct.unpack_from(">HH", p, i)
            if kind == 0 and i + 9 <= len(p):
                ln = struct.unpack_from(">H", p, i + 7)[0]
                return p[i + 9 : i + 9 + ln].decode("ascii", "replace")
            i += 4 + n
    except (IndexError, struct.error):
        return None
    return None


METHODS = (b"GET ", b"POST ", b"HEAD ", b"PUT ", b"CONNECT ", b"OPTIONS ", b"DELETE ", b"PATCH ")


def http_host(payload):
    """The Host of a plaintext HTTP request at the start of `payload`: None
    when it is not one, "" when it names no Host."""
    head = payload[:2048]
    if not any(head.startswith(m) for m in METHODS):
        return None
    for line in head.split(b"\r\n")[1:]:
        if line.lower().startswith(b"host:"):
            return line[5:].strip().decode("ascii", "replace")
    return ""
