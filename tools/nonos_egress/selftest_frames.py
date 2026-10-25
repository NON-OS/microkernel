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
"""Frames and captures the self-test builds."""

import struct
import zlib

GUEST = bytes.fromhex("0a1b2c3d4e5f")  # locally administered, unicast
BURNED = bytes.fromhex("001b2c3d4e5f")  # a vendor's burned-in address
SLIRP = bytes.fromhex("52550a000202")
NIC = bytes.fromhex("525400123456")


def pcap(frames_):
    out = struct.pack("<IHHiIII", 0xA1B2C3D4, 2, 4, 0, 0, 65535, 1)
    for f in frames_:
        out += struct.pack("<IIII", 0, 0, len(f), len(f)) + f
    return out


def ip(proto, dst, l4, src=(10, 0, 2, 15)):
    total = 20 + len(l4)
    return struct.pack(">BBHHHBBH4s4s", 0x45, 0, total, 0, 0, 64, proto, 0, bytes(src), bytes(dst)) + l4


def eth(payload, src=GUEST, ethertype=0x0800, dst=SLIRP):
    return dst + src + struct.pack(">H", ethertype) + payload


def udp(dst, dport, body, sport=40000, src=GUEST):
    return eth(ip(17, dst, struct.pack(">HHHH", sport, dport, 8 + len(body), 0) + body), src)


def tcp(dst, dport, body, src=GUEST, src_ip=(10, 0, 2, 15)):
    seg = struct.pack(">HHIIBBHHH", 41000, dport, 1, 0, 0x50, 0x18, 1024, 0, 0) + body
    return eth(ip(6, dst, seg, src=src_ip), src)


def dhcp(options, chaddr=GUEST, src=GUEST, dst=(255, 255, 255, 255), sname=b"", file=b""):
    body = bytearray(240)
    body[0], body[1], body[2] = 1, 1, 6
    body[28:34] = chaddr
    body[44 : 44 + len(sname)], body[108 : 108 + len(file)] = sname, file
    body[236:240] = b"\x63\x82\x53\x63"
    for code, val in options:
        body += bytes([code, len(val)]) + val
    return udp(dst, 67, bytes(body) + b"\xff", sport=68, src=src)


def offer(server=(10, 0, 2, 2), kind=b"\x02"):
    """A DHCP OFFER (or ACK, kind 5) from `server` to the guest, as the ingress holds it."""
    body = bytearray(240)
    body[0], body[1], body[2] = 2, 1, 6
    body[28:34] = GUEST
    body[236:240] = b"\x63\x82\x53\x63"
    body += bytes([53, 1]) + kind + bytes([54, 4]) + bytes(server) + b"\xff"
    seg = struct.pack(">HHHH", 67, 68, 8 + len(body), 0) + bytes(body)
    return GUEST + SLIRP + b"\x08\x00" + ip(17, (255, 255, 255, 255), seg, src=server)


def arp(sender, src=GUEST):
    body = struct.pack(">HHBBH", 1, 0x0800, 6, 4, 1) + sender + bytes([10, 0, 2, 15]) + bytes(6) + bytes([10, 0, 2, 2])
    return eth(body, src=src, ethertype=0x0806, dst=b"\xff" * 6)


def hello(name):
    sni = name.encode()
    ext = struct.pack(">HHHBH", 0, len(sni) + 5, len(sni) + 3, 0, len(sni)) + sni
    body = b"\x03\x03" + bytes(32) + b"\x00" + b"\x00\x02\x13\x01" + b"\x01\x00" + struct.pack(">H", len(ext)) + ext
    hs = b"\x01" + len(body).to_bytes(3, "big") + body
    return b"\x16\x03\x01" + struct.pack(">H", len(hs)) + hs


def reply(src_ip, sport, body, dport=41000):
    """A TCP segment from `src_ip` to the guest, as the ingress holds it."""
    seg = struct.pack(">HHIIBBHHH", sport, dport, 1, 1, 0x50, 0x18, 1024, 0, 0) + body
    return GUEST + SLIRP + b"\x08\x00" + ip(6, (10, 0, 2, 15), seg, src=src_ip)


def consensus(*addrs):
    lines = b"network-status-version 3 microdesc\n" + b"".join(
        b"r relay%d AAAA 2026-10-24 12:00:00 %s 9001 0\nm abc\n" % (i, a.encode()) for i, a in enumerate(addrs))
    return b"HTTP/1.0 200 OK\r\nContent-Encoding: deflate\r\n\r\n" + zlib.compress(lines)


def ipv6_nd(src):
    """An IPv6 neighbour solicitation, as UEFI sends one."""
    return eth(b"\x60" + bytes(5) + b"\x3a\xff" + bytes(32) + b"\x87" + bytes(23), src=src, ethertype=0x86DD)
