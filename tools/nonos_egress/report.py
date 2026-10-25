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
"""The judge's state over one capture, and the report it ends in (judge.py
lists the rules)."""

import struct
from collections import Counter

from . import dhcp_rules, firmware
from .payloads import http_host, tls_sni
from .pcap import ip_text, mac_text

QEMU_NIC_MAC = bytes.fromhex("525400123456")
DNS, NTP = {53, 853}, {123}
DISCOVERY = {5353, 5355, 1900, 137, 138}


class Report:
    def __init__(self, allow, learned, nic_mac, dhcp_servers=frozenset()):
        self.allow, self.learned, self.nic_mac = allow, learned, nic_mac
        self.dhcp_servers = dhcp_servers
        self.failures, self.peers = [], Counter()
        self.frames = self.guest = self.firmware = 0
        self.macs, self.dhcp_params, self.dhcp_seen = set(), [], False
        self.sni, self.http = Counter(), Counter()
        self.os_started = False

    def fail(self, kind, what):
        self.failures.append({"kind": kind, "what": what})

    def frame(self, f):
        self.frames += 1
        if len(f) < 14:
            self.fail("ethertype", f"a frame of {len(f)} bytes, short of an Ethernet header")
            return
        src, ethertype = f[6:12], struct.unpack_from(">H", f, 12)[0]
        if src == self.nic_mac and not self.os_started and (kind := firmware.kind(f)):
            self.firmware += 1
            if kind == "dhcp":
                l4 = f[14 + (f[14] & 0x0F) * 4 :]
                dhcp_rules.check(self.fail, l4[8:], src, ip_text(f[30:34]), self.dhcp_servers, "firmware")
            return
        self.guest += 1
        if src == self.nic_mac:
            self.fail("mac", f"the NIC's assigned address {mac_text(src)} on a frame the firmware may not send")
        else:
            self.os_started = True
        if src not in self.macs:
            self.macs.add(src)
            if not src[0] & 0x02 or src[0] & 0x01:
                self.fail("mac", f"source {mac_text(src)} is not a locally administered unicast address")
        if ethertype == 0x0806:
            self.arp(f, src)
        elif ethertype == 0x86DD:
            self.fail("ipv6", f"an IPv6 frame from {mac_text(src)}")
        elif ethertype != 0x0800:
            self.fail("ethertype", f"ethertype {ethertype:#06x} from {mac_text(src)}")
        else:
            self.ipv4(f, src)

    def arp(self, f, src):
        if len(f) < 42 or f[22:28] != src:
            sender = mac_text(f[22:28]) if len(f) >= 28 else "cut short"
            self.fail("arp", f"ARP sender {sender} is not the frame's source {mac_text(src)}")

    def ipv4(self, f, src):
        if len(f) < 34:
            self.fail("ip-proto", "an IPv4 frame cut short of its header")
            return
        proto, dst, l4 = f[23], ip_text(f[30:34]), f[14 + (f[14] & 0x0F) * 4 :]
        if proto not in (6, 17):
            what = f"ICMP type {l4[0]}" if proto == 1 and l4 else f"IP protocol {proto}"
            self.fail("ip-proto", f"{what} to {dst}")
            return
        if len(l4) < 8:
            self.fail("ip-proto", f"a {'TCP' if proto == 6 else 'UDP'} header cut short, to {dst}")
            return
        sport, dport = struct.unpack_from(">HH", l4)
        name, where = ("udp" if proto == 17 else "tcp"), f"{dst}:{dport}"
        if dport in DNS:
            self.fail("dns", f"{name} to {where}")
        elif proto == 17 and dport in NTP:
            self.fail("ntp", f"udp to {where}")
        elif dport in DISCOVERY:
            self.fail("discovery", f"{name} to {where}")
        if proto == 17 and (sport, dport) == (68, 67):
            self.dhcp(l4[8:], src, dst)
            return
        if proto == 6:
            self.tcp(l4, dst, where)
        self.peers[(name, dst, dport)] += 1

    def dhcp(self, payload, src, dst):
        params = dhcp_rules.check(self.fail, payload, src, dst, self.dhcp_servers, "the guest's")
        if params is None:
            return
        self.dhcp_seen = True
        if params not in self.dhcp_params:
            self.dhcp_params.append(params)

    def tcp(self, l4, dst, where):
        payload = l4[(l4[12] >> 4) * 4 :] if len(l4) >= 13 else b""
        if sni := tls_sni(payload):
            self.sni[sni] += 1
            if sni not in self.allow.names:
                self.fail("sni", f"TLS server name {sni} to {where}")
        host = http_host(payload)
        if host is None:
            return
        self.http[host] += 1
        port = int(where.rsplit(":", 1)[1])
        if host and host.split(":")[0] not in self.allow.names:
            self.fail("http", f"plaintext HTTP Host {host!r} to {where}")
        elif not host and (dst, port) not in self.allow.peers:
            self.fail("http", f"plaintext HTTP with no Host to {where}, a peer the image does not pin")

    def done(self):
        listed = []
        for (proto, ip, port), n in sorted(self.peers.items()):
            pinned, relay = (ip, port) in self.allow.peers, (ip, port) in self.learned
            why = "pinned" if pinned else "relay" if relay else "unchosen"
            listed.append({"proto": proto, "ip": ip, "port": port, "frames": n, "why": why})
            if why == "unchosen":
                self.fail("peer", f"{proto} to {ip}:{port}, a peer the image did not choose")
        reached = any(p["why"] == "pinned" for p in listed)
        if self.failures:
            status = "fail"
        elif not self.guest:
            status = "empty"
        elif not (self.dhcp_seen and reached):
            status = "incomplete"
        else:
            status = "pass"
        return {
            "status": status,
            "frames": self.frames,
            "guest_frames": self.guest,
            "firmware_frames": self.firmware,
            "guest_macs": sorted(mac_text(m) for m in self.macs),
            "dhcp_seen": self.dhcp_seen,
            "pinned_reached": reached,
            "dhcp_parameter_lists": self.dhcp_params,
            "tls_server_names": dict(self.sni),
            "http_hosts": dict(self.http),
            "relays_learned": len(self.learned),
            "peers": listed,
            "failures": self.failures,
        }
