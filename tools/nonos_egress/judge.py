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
"""Every egress frame classified. These fail the run:

  dns        UDP or TCP to port 53, or 853 (DoT)
  ntp        UDP to 123
  discovery  mDNS (5353), LLMNR (5355), SSDP (1900), NetBIOS (137, 138)
  ipv6       any IPv6 frame that is not the firmware's
  ethertype  any frame neither IPv4 nor ARP: 802.1Q-tagged, LLDP, anything
  ip-proto   IPv4 that is not TCP or UDP, ICMP included
  mac        a source that is not locally administered unicast, or the NIC's
             own assigned address on any frame the firmware may not send
  arp        an ARP sender address that is not the frame's source
  dhcp-id    a DHCP option that names the machine (12, 60, 77, 81, 97), a
             chaddr or client identifier (61) that is not the frame's source
  dhcp-opt   any other option but 50, 53, 54, 55 and 57, or sname or file set
  dhcp-dst   DHCP to an address neither broadcast nor a server the ingress's
             OFFER or ACK named
             (the firmware's DHCP is held to all three: dhcp_rules.py)
  sni        a TLS server name not on the allow-list
  http       a plaintext Host not on the allow-list, or a request with no
             Host to a peer the image does not pin
  peer       a TCP or UDP (address, port) neither pinned nor a relay

Then a capture with no guest frame is empty, and one with no DHCP or no
pinned peer reached is incomplete: neither is a pass."""

from .allow import Allow
from .pcap import frames
from .relays import relays
from .dhcp_rules import servers
from .report import QEMU_NIC_MAC, Report


def judge(egress, ingress=b"", allow=Allow(), nic_mac=QEMU_NIC_MAC):
    """The report for an egress capture's bytes, with the ingress capture's
    relays allowed. Either capture damaged raises Unreadable."""
    out = frames(egress)
    inward = frames(ingress) if ingress else []
    learned = relays(inward, allow.authorities)
    r = Report(allow, learned, nic_mac, servers(inward))
    for f in out:
        r.frame(f)
    return r.done()
