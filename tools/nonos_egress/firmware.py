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
"""What the firmware may send before NONOS runs. UEFI brings the NIC up with
its assigned address and may send ARP, DHCP, and IPv6 neighbour discovery
and multicast listener messages from it. Only those, only from that address,
and only before NONOS has sent a frame of its own, are the firmware's; any
other frame is the guest's and is judged."""

import struct

# ICMPv6 MLD (130-132, 143) and neighbour discovery (133-137).
ND_MLD = {130, 131, 132, 133, 134, 135, 136, 137, 143}


def kind(f):
    """'arp', 'dhcp' or 'nd' for a frame the firmware may send, else None."""
    if len(f) < 14:
        return None
    ethertype = struct.unpack_from(">H", f, 12)[0]
    if ethertype == 0x0806:
        return "arp"
    if ethertype == 0x0800 and len(f) >= 34 and f[23] == 17:
        l4 = f[14 + (f[14] & 0x0F) * 4 :]
        if len(l4) >= 4 and struct.unpack_from(">HH", l4) == (68, 67):
            return "dhcp"
        return None
    if ethertype == 0x86DD and len(f) >= 55:
        nh, at = f[20], 54
        if nh == 0 and len(f) >= at + 2:  # hop-by-hop options, as MLDv2 carries
            nh, at = f[at], at + (f[at + 1] + 1) * 8
        if nh == 58 and len(f) > at and f[at] in ND_MLD:
            return "nd"
    return None
