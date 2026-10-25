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
"""One capture per rule, and the evasions."""

from .selftest import CLEAN, INGRESS, caught, kinds, run
from .selftest_frames import BURNED, GUEST, NIC, SLIRP, arp, consensus, dhcp, eth, hello, ip, ipv6_nd
from .selftest_frames import reply, tcp, udp


def rules():
    caught("dns", [udp((10, 0, 2, 3), 53, b"\x00" * 20)], "DNS")
    caught("dns", [tcp((10, 0, 2, 3), 853, b"\x00" * 10)], "DNS over TLS")
    caught("ntp", [udp((162, 159, 200, 1), 123, b"\x23" + bytes(47))], "NTP")
    caught("discovery", [udp((224, 0, 0, 251), 5353, b"\x00" * 12)], "mDNS")
    caught("ipv6", [ipv6_nd(GUEST)], "IPv6 from the guest")
    caught("mac", [udp((92, 39, 63, 14), 9999, b"x", src=BURNED)], "a burned-in address")
    caught("dhcp-id", [dhcp([(53, b"\x01"), (12, b"nonos-laptop")])], "a hostname")
    caught("sni", [tcp((92, 39, 63, 14), 443, hello("tracker.example"))], "SNI")
    caught("http", [tcp((92, 39, 63, 14), 443, b"GET / HTTP/1.1\r\nHost: telemetry.example\r\n\r\n")],
           "a Host not allowed, to a pinned address")
    caught("http", [tcp((203, 0, 113, 7), 80, b"GET / HTTP/1.0\r\n\r\n")], "no Host, to a peer not pinned")
    caught("peer", [tcp((203, 0, 113, 9), 443, b"\x00" * 40)], "an unchosen peer")
    caught("peer", [tcp((92, 39, 63, 14), 9999, b"\x00" * 40)], "a pinned address on another port")


def evasions():
    # (a) The guest picks its source MAC: QEMU's own host address exempts nothing.
    caught("dns", [udp((10, 0, 2, 3), 53, b"\x00" * 20, src=SLIRP)], "DNS from slirp's MAC")
    # (b) The NIC's address before the OS sends is the firmware's only for ARP,
    # DHCP and neighbour discovery; anything else from it is judged.
    r = run([arp(NIC, src=NIC), ipv6_nd(NIC), dhcp([(53, b"\x01")], chaddr=NIC, src=NIC)] + CLEAN)
    assert r["status"] == "pass" and r["firmware_frames"] == 3, r
    early = [udp((10, 0, 2, 3), 53, b"\x00" * 20, src=NIC)]
    assert {"dns", "mac"} <= kinds(run(early + CLEAN)), "DNS from the NIC's address before the OS"
    caught("mac", [arp(NIC, src=NIC)], "the NIC's address once the OS has sent")
    # (c) A consensus the guest sent itself, or one from an authority's address
    # on another port, names no relay.
    forged = reply((49, 13, 145, 234), 9230, consensus("203.0.113.9"), dport=42000)
    rogue = tcp((203, 0, 113, 9), 9001, b"\x00" * 8)
    caught("peer", [forged, rogue], "a consensus in the egress", naming="203.0.113.9:9001")
    other = INGRESS + [reply((49, 13, 145, 234), 80, consensus("203.0.113.9"), dport=42000)]
    caught("peer", [rogue], "an authority off its port", ingress=other, naming="203.0.113.9:9001")
    # (d) Ethertypes and IP protocols the judge would not have read.
    tagged = eth(b"\x00\x01\x08\x00" + ip(17, (10, 0, 2, 3), b"\x9c\x40\x00\x35\x00\x08\x00\x00"), ethertype=0x8100)
    caught("ethertype", [tagged], "802.1Q-tagged DNS")
    caught("ethertype", [eth(b"\x02\x07\x04" + bytes(30), ethertype=0x88CC, dst=bytes.fromhex("0180c200000e"))], "LLDP")
    caught("ip-proto", [eth(ip(1, (1, 2, 3, 4), b"\x08\x00\x00\x00" + b"exfiltrated bytes"))], "ICMP with a payload")
    caught("ip-proto", [eth(ip(47, (1, 2, 3, 4), bytes(8)))], "GRE")


def identity():
    caught("dhcp-id", [dhcp([(53, b"\x03"), (12, b"host")], dst=(10, 0, 2, 2))], "a unicast renewal")
    caught("dhcp-id", [dhcp([(53, b"\x01")], chaddr=BURNED)], "a chaddr not the frame's")
    caught("dhcp-id", [dhcp([(53, b"\x01"), (61, b"\x01" + BURNED)])], "a client id not the frame's")
    caught("dhcp-id", [dhcp([(53, b"\x01"), (61, b"\xff" + bytes(12))])], "a DUID client id")
    caught("arp", [arp(BURNED)], "an ARP sender not the frame's")
