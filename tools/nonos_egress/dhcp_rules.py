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
"""What a DHCP message from this machine may carry and where it may go, the
firmware's as much as the guest's. Only options 53 (type), 50 (requested
address), 54 (server), 55 (parameters), 57 (size) and 61 (client id, type 1
and the frame's source) may be sent. sname and file stay empty, so option 52
cannot move options into them. A message goes to the broadcast address or to
a server the network named in an OFFER or ACK, read from the ingress, never
to an address the sender picked."""

import struct

from .payloads import dhcp
from .pcap import ip_text, mac_text

ALLOWED = {53, 50, 54, 55, 57, 61}
NAMING = {12: "hostname", 60: "vendor class", 77: "user class", 81: "client FQDN", 97: "machine UUID"}
BROADCAST = "255.255.255.255"


def servers(ingress_frames):
    """{ip} of every server whose OFFER or ACK reached the guest: the source
    address and option 54."""
    found = set()
    for f in ingress_frames:
        if len(f) < 34 or f[12:14] != b"\x08\x00" or f[23] != 17:
            continue
        l4 = f[14 + (f[14] & 0x0F) * 4 :]
        if len(l4) < 8 or struct.unpack_from(">HH", l4) != (67, 68):
            continue
        msg = dhcp(l4[8:])
        if msg is None or msg[1].get(53) not in (b"\x02", b"\x05"):
            continue
        found.add(ip_text(f[26:30]))
        if len(msg[1].get(54, b"")) == 4:
            found.add(ip_text(msg[1][54]))
    return found


def check(fail, payload, src, dst, named, who):
    """Every rule on one message to UDP 67, as `fail(kind, what)`; option 55
    as a list, or None when the payload is not DHCP."""
    if dst != BROADCAST and dst not in named:
        fail("dhcp-dst", f"{who} DHCP to {dst}, neither broadcast nor a server the network named")
    msg = dhcp(payload)
    if msg is None:
        fail("dhcp-id", f"{who} frame to a DHCP server from {mac_text(src)} that is not DHCP")
        return None
    chaddr, opts = msg
    if chaddr != src:
        fail("dhcp-id", f"{who} chaddr {mac_text(chaddr)} is not the frame's source {mac_text(src)}")
    if any(payload[44:236]):
        fail("dhcp-opt", f"{who} DHCP with sname or file set")
    for code in sorted(opts):
        if code in NAMING:
            fail("dhcp-id", f"{who} DHCP option {code} ({NAMING[code]}) = {opts[code][:64]!r}")
        elif code not in ALLOWED:
            fail("dhcp-opt", f"{who} DHCP option {code} = {opts[code][:32].hex()}")
    cid = opts.get(61)
    if cid is not None and cid != b"\x01" + src:
        fail("dhcp-id", f"{who} client identifier {cid[:20].hex()} is not type 1 and the frame's source")
    return list(opts.get(55, b""))
