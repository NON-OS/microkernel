#!/usr/bin/env python3
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
"""What a guest put on the wire, read from a QEMU filter-dump capture, and
whether any of it names the machine or the person outside the anonymity
networks.

The capture holds both directions. A frame from a user-net host MAC (QEMU's
slirp answers as 52:55:0a:00:02:02, and as 52:56:00:00:00:02 for IPv6) is the
host's and is not judged. A frame from the NIC's own assigned MAC (--nic-mac,
QEMU's 52:54:00:12:34:56 by default) before the guest has sent from any other
address is the firmware's: UEFI brings the NIC up with it before NONOS runs.
Those are counted and listed, not judged. NONOS replaces that address with a
random one, so a frame from it once another guest address has appeared
fails as `mac`. Every other frame is the guest's, is classified, and these
fail the run:

  dns        any UDP or TCP to port 53, or 853 (DoT): a name in the clear
  ntp        UDP to 123: the clock, and the machine, to a time server
  discovery  mDNS (5353), LLMNR (5355), SSDP (1900), NetBIOS (137/138)
  ipv6       any IPv6 frame: the image has no IPv6, so one is a stack nobody audited
  mac        a source MAC without the locally administered bit, or with the
             multicast bit: the card's burned-in address, which names the device
  dhcp-id    a DHCP client option that names the machine: hostname (12),
             vendor class (60), user class (77), FQDN (81)
  sni        a TLS ClientHello whose server name is not in --allow-name
  http       a plaintext HTTP request whose Host is not in --allow-name, to a
             peer not in --allow (a directory authority, pinned by address,
             is asked without a Host)

  peer       a TCP or UDP peer the image did not choose: not given with --allow
             (the bootstrap the capsules pin), not a relay the consensus a
             pinned authority sent in this same capture names, and not the
             DHCP server's broadcast

The relays are read from the capture itself: every reply a peer given with
--allow sent on a TCP stream is gathered, its HTTP body inflated when it is
zlib, and each `r` line's address taken, which is how an onion network's
consensus names its relays. A consensus only an allowed authority sent can
widen the set, so a host that is not one cannot vouch for itself. Each
peer, DHCP parameter list, TLS server name and HTTP host is listed too.

The verdict and every finding go to --json as well. Exit 0 pass, 1 fail,
2 when the capture cannot be read.
"""

import argparse
import json
import re
import struct
import sys
import zlib
from collections import Counter, defaultdict

SLIRP_HOST_MAC = bytes.fromhex("52550a000202")
SLIRP_HOST_MACS = (SLIRP_HOST_MAC, bytes.fromhex("525600000002"))
QEMU_NIC_MAC = bytes.fromhex("525400123456")

DNS_PORTS = {53, 853}
NTP_PORTS = {123}
DISCOVERY_PORTS = {5353, 5355, 1900, 137, 138}
DHCP_NAMING = {12: "hostname", 60: "vendor class", 77: "user class", 81: "client FQDN"}


class Unreadable(Exception):
    pass


def frames(data):
    """Each frame of a classic pcap, link type Ethernet. Truncated records end
    the walk; a header that is not pcap is refused."""
    if len(data) < 24:
        raise Unreadable("shorter than a pcap header")
    magic = struct.unpack_from("<I", data)[0]
    if magic in (0xA1B2C3D4, 0xA1B23C4D):
        endian = "<"
    elif magic in (0xD4C3B2A1, 0x4D3CB2A1):
        endian = ">"
    else:
        raise Unreadable("not a pcap file")
    linktype = struct.unpack_from(endian + "I", data, 20)[0]
    if linktype != 1:
        raise Unreadable(f"link type {linktype}, not Ethernet")
    off = 24
    while off + 16 <= len(data):
        incl = struct.unpack_from(endian + "I", data, off + 8)[0]
        if incl > 0x40000 or off + 16 + incl > len(data):
            return
        yield data[off + 16 : off + 16 + incl]
        off += 16 + incl


def mac_text(m):
    return ":".join(f"{b:02x}" for b in m)


def ip_text(b):
    return ".".join(str(x) for x in b)


def dhcp_options(udp_payload):
    """(message type, {code: bytes}) of a BOOTP/DHCP payload, or None."""
    p = udp_payload
    if len(p) < 240 or p[236:240] != b"\x63\x82\x53\x63":
        return None
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
    return opts


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


def http_host(payload):
    """The Host of a plaintext HTTP request at the start of `payload`, or None."""
    head = payload[:2048]
    if not any(head.startswith(m) for m in (b"GET ", b"POST ", b"HEAD ", b"PUT ", b"CONNECT ")):
        return None
    for line in head.split(b"\r\n")[1:]:
        if line.lower().startswith(b"host:"):
            return line[5:].strip().decode("ascii", "replace")
    return ""


RELAY_LINE = re.compile(rb"^r \S+ \S+ (?:\S+ )?\S+ \S+ (\d{1,3}\.\d{1,3}\.\d{1,3}\.\d{1,3}) \d+ \d+", re.M)


def relays(data, authorities):
    """The relay addresses every consensus a host in `authorities` sent on a
    TCP stream in the capture names. Each stream's bytes are joined in
    capture order; the HTTP body is inflated when it is zlib."""
    streams = defaultdict(bytearray)
    for f in frames(data):
        if len(f) < 34 or f[12:14] != b"\x08\x00" or f[23] != 6:
            continue
        src = ip_text(f[26:30])
        if src not in authorities:
            continue
        l4 = f[14 + (f[14] & 0x0F) * 4 :]
        if len(l4) < 20:
            continue
        sport, dport = struct.unpack_from(">HH", l4)
        streams[(src, sport, dport)] += l4[(l4[12] >> 4) * 4 :]
    found = set()
    for raw in streams.values():
        body = bytes(raw).split(b"\r\n\r\n", 1)[-1]
        for text in (body, _inflate(body)):
            found.update(m.group(1).decode() for m in RELAY_LINE.finditer(text))
    return found


def _inflate(body):
    try:
        return zlib.decompressobj().decompress(body)
    except zlib.error:
        return b""


def judge(data, allow_ips=(), allow_names=(), host_macs=SLIRP_HOST_MACS, nic_mac=QEMU_NIC_MAC):
    """The report for a capture's bytes: every guest frame classified."""
    allow_names = set(allow_names)
    pinned = set(allow_ips)
    learned = relays(data, pinned)
    allow_ips = pinned | learned
    failures, peers = [], Counter()
    seen = {"frames": 0, "guest_frames": 0, "firmware_frames": 0, "macs": set(), "dhcp_params": [],
            "sni": Counter(), "http": Counter()}
    os_started = False

    def fail(kind, what):
        failures.append({"kind": kind, "what": what})

    for f in frames(data):
        seen["frames"] += 1
        if len(f) < 14:
            continue
        dst_mac, src_mac, ethertype = f[0:6], f[6:12], struct.unpack_from(">H", f, 12)[0]
        if src_mac in host_macs:
            continue
        if src_mac == nic_mac and not os_started:
            seen["firmware_frames"] += 1
            continue
        if src_mac == nic_mac:
            fail("mac", f"a frame from the NIC's own address {mac_text(src_mac)} after the guest had another")
        else:
            os_started = True
        seen["guest_frames"] += 1
        if src_mac not in seen["macs"]:
            seen["macs"].add(src_mac)
            if not src_mac[0] & 0x02 or src_mac[0] & 0x01:
                fail("mac", f"source {mac_text(src_mac)} is not a locally administered unicast address")
        if ethertype == 0x86DD:
            fail("ipv6", f"an IPv6 frame from {mac_text(src_mac)}")
            continue
        if ethertype != 0x0800 or len(f) < 34:
            continue
        ihl = (f[14] & 0x0F) * 4
        proto, dst = f[23], f[30:34]
        l4 = f[14 + ihl :]
        if proto not in (6, 17) or len(l4) < 8:
            continue
        sport, dport = struct.unpack_from(">HH", l4)
        where = f"{ip_text(dst)}:{dport}"
        name = "udp" if proto == 17 else "tcp"
        if dport in DNS_PORTS:
            fail("dns", f"{name} to {where}")
        elif proto == 17 and dport in NTP_PORTS:
            fail("ntp", f"udp to {where}")
        elif dport in DISCOVERY_PORTS:
            fail("discovery", f"{name} to {where}")
        if proto == 17:
            payload = l4[8:]
            if sport == 68 and dport == 67 and ip_text(dst) == "255.255.255.255":
                opts = dhcp_options(payload)
                if opts is not None:
                    for code, label in DHCP_NAMING.items():
                        if code in opts:
                            fail("dhcp-id", f"DHCP option {code} ({label}) = {opts[code][:64]!r}")
                    params = list(opts.get(55, b""))
                    if params not in seen["dhcp_params"]:
                        seen["dhcp_params"].append(params)
                continue
        else:
            doff = (l4[12] >> 4) * 4 if len(l4) >= 13 else 20
            payload = l4[doff:]
            if sni := tls_sni(payload):
                seen["sni"][sni] += 1
                if sni not in allow_names:
                    fail("sni", f"TLS server name {sni} to {where}")
            host = http_host(payload)
            if host is not None:
                seen["http"][host] += 1
                if host.split(":")[0] not in allow_names and ip_text(dst) not in allow_ips:
                    fail("http", f"plaintext HTTP Host {host!r} to {where}")
        peers[(name, ip_text(dst), dport)] += 1

    listed = [
        {"proto": p, "ip": ip, "port": port, "frames": n, "allowed": ip in allow_ips,
         "why": "pinned" if ip in pinned else "relay" if ip in learned else "unchosen"}
        for (p, ip, port), n in sorted(peers.items())
    ]
    for p in listed:
        if not p["allowed"]:
            fail("peer", f"{p['proto']} to {p['ip']}:{p['port']}, a peer the image did not choose")
    return {
        # A capture the guest sent nothing in shows no path was clean, only
        # that none was used: it is empty, not a pass.
        "status": "fail" if failures else "pass" if seen["guest_frames"] else "empty",
        "frames": seen["frames"],
        "guest_frames": seen["guest_frames"],
        "firmware_frames": seen["firmware_frames"],
        "guest_macs": sorted(mac_text(m) for m in seen["macs"]),
        "dhcp_parameter_lists": seen["dhcp_params"],
        "tls_server_names": dict(seen["sni"]),
        "http_hosts": dict(seen["http"]),
        "relays_learned": len(learned),
        "peers": listed,
        "failures": failures,
    }


# ---------------------------------------------------------------------------
# Self-test: captures built here, one per rule, and a fuzz over damaged ones.

GUEST = bytes.fromhex("0a1b2c3d4e5f")  # locally administered, unicast
BURNED = bytes.fromhex("001b2c3d4e5f")  # a vendor's burned-in address


def _pcap(frames_):
    out = struct.pack("<IHHiIII", 0xA1B2C3D4, 2, 4, 0, 0, 65535, 1)
    for f in frames_:
        out += struct.pack("<IIII", 0, 0, len(f), len(f)) + f
    return out


def _ip(proto, dst, l4, src=(10, 0, 2, 15)):
    total = 20 + len(l4)
    return struct.pack(">BBHHHBBH4s4s", 0x45, 0, total, 0, 0, 64, proto, 0, bytes(src), bytes(dst)) + l4


def _eth(payload, src=GUEST, ethertype=0x0800):
    return SLIRP_HOST_MAC + src + struct.pack(">H", ethertype) + payload


def _udp(dst, dport, body, sport=40000, src=GUEST):
    return _eth(_ip(17, dst, struct.pack(">HHHH", sport, dport, 8 + len(body), 0) + body), src)


def _tcp(dst, dport, body, src=GUEST):
    return _eth(_ip(6, dst, struct.pack(">HHIIBBHHH", 41000, dport, 1, 0, 0x50, 0x18, 1024, 0, 0) + body), src)


def _dhcp(options):
    body = bytearray(240)
    body[0] = 1
    body[28:34] = GUEST
    body[236:240] = b"\x63\x82\x53\x63"
    for code, val in options:
        body += bytes([code, len(val)]) + val
    return _udp((255, 255, 255, 255), 67, bytes(body) + b"\xff", sport=68)


def _hello(name):
    sni = name.encode()
    ext = struct.pack(">HHHBH", 0, len(sni) + 5, len(sni) + 3, 0, len(sni)) + sni
    hello = b"\x03\x03" + bytes(32) + b"\x00" + b"\x00\x02\x13\x01" + b"\x01\x00" + struct.pack(">H", len(ext)) + ext
    hs = b"\x01" + len(hello).to_bytes(3, "big") + hello
    return b"\x16\x03\x01" + struct.pack(">H", len(hs)) + hs


def _from(src_ip, sport, body):
    """A TCP segment from `src_ip` to the guest: a server's reply."""
    tcp = struct.pack(">HHIIBBHHH", sport, 41000, 1, 1, 0x50, 0x18, 1024, 0, 0) + body
    return GUEST + SLIRP_HOST_MAC + b"\x08\x00" + _ip(6, (10, 0, 2, 15), tcp, src=src_ip)


def _consensus(*addrs):
    lines = b"network-status-version 3 microdesc\n" + b"".join(
        b"r relay%d AAAA 2026-10-24 12:00:00 %s 9001 0\nm abc\n" % (i, a.encode())
        for i, a in enumerate(addrs))
    return b"HTTP/1.0 200 OK\r\nContent-Encoding: deflate\r\n\r\n" + zlib.compress(lines)


def _inbound():
    return SLIRP_HOST_MAC[:0] + GUEST + SLIRP_HOST_MAC + b"\x08\x00" + _ip(17, (10, 0, 2, 15), struct.pack(">HHHH", 53, 40000, 8, 0), src=(10, 0, 2, 3))


def self_test():
    clean = [
        _dhcp([(53, b"\x01"), (55, b"\x01\x03\x06"), (61, b"\x01" + GUEST)]),
        _tcp((92, 39, 63, 14), 443, _hello("validator.nymtech.net")),
        _tcp((188, 172, 228, 15), 9000, b"\x00" * 40),
        _inbound(),
        _tcp((49, 13, 145, 234), 9230, b"GET /tor/status-vote/current/consensus-microdesc.z HTTP/1.0\r\n\r\n"),
        _from((49, 13, 145, 234), 9230, _consensus("5.6.7.8", "9.9.9.9")),
        _tcp((5, 6, 7, 8), 9001, _hello("www.abcdefgh.net")),
    ]
    allow = {"allow_names": ["validator.nymtech.net", "www.abcdefgh.net"],
             "allow_ips": ["92.39.63.14", "188.172.228.15", "49.13.145.234"]}
    r = judge(_pcap(clean), **allow)
    assert r["status"] == "pass", r["failures"]
    assert r["guest_frames"] == 5, "the hosts' replies to the guest are not judged"
    assert r["tls_server_names"] == {"validator.nymtech.net": 1, "www.abcdefgh.net": 1}
    assert r["dhcp_parameter_lists"] == [[1, 3, 6]]
    assert r["relays_learned"] == 2
    assert {p["ip"]: p["why"] for p in r["peers"]} == {
        "188.172.228.15": "pinned", "49.13.145.234": "pinned", "5.6.7.8": "relay", "92.39.63.14": "pinned"}, r["peers"]

    cases = {
        "dns": _udp((10, 0, 2, 3), 53, b"\x00" * 20),
        "ntp": _udp((162, 159, 200, 1), 123, b"\x23" + bytes(47)),
        "discovery": _udp((224, 0, 0, 251), 5353, b"\x00" * 12),
        "ipv6": _eth(b"\x60" + bytes(39), ethertype=0x86DD),
        "mac": _udp((92, 39, 63, 14), 9999, b"x", src=BURNED),
        "dhcp-id": _dhcp([(53, b"\x01"), (12, b"nonos-laptop")]),
        "sni": _tcp((1, 2, 3, 4), 443, _hello("tracker.example")),
        "http": _tcp((1, 2, 3, 4), 80, b"GET / HTTP/1.1\r\nHost: telemetry.example\r\n\r\n"),
        "peer": _tcp((203, 0, 113, 9), 443, b"\x00" * 40),
    }
    for kind, frame in cases.items():
        r = judge(_pcap(clean + [frame]), **allow)
        kinds = {f["kind"] for f in r["failures"]}
        assert kind in kinds, f"{kind} not caught: {r['failures']}"
        assert r["status"] == "fail"
    # A host nobody pinned cannot vouch for itself with a consensus of its own.
    rogue = [_from((203, 0, 113, 9), 80, _consensus("203.0.113.9")), _tcp((203, 0, 113, 9), 80, b"\x00" * 8)]
    assert {f["kind"] for f in judge(_pcap(rogue), **allow)["failures"]} == {"peer"}, "self-vouching"
    dirauth = _tcp((92, 39, 63, 14), 9230, b"GET /tor/status-vote/current/consensus HTTP/1.0\r\n\r\n")
    assert judge(_pcap([dirauth]), **allow)["status"] == "pass", "a pinned authority, asked without a Host"
    assert judge(_pcap([dirauth]))["status"] == "fail", "the same request to a peer nobody pinned"
    tcp_dns = _tcp((10, 0, 2, 3), 853, b"\x00" * 10)
    assert {f["kind"] for f in judge(_pcap([tcp_dns]))["failures"]} >= {"dns"}, "DNS over TLS"

    # The firmware's frames from the NIC's own address, before the OS sends,
    # are counted, not judged; the same address once the OS has sent fails.
    fw = _eth(b"\x60" + bytes(39), src=QEMU_NIC_MAC, ethertype=0x86DD)
    r = judge(_pcap([fw, fw] + clean), **allow)
    assert r["status"] == "pass" and r["firmware_frames"] == 2, r
    r = judge(_pcap(clean + [_udp((92, 39, 63, 14), 9999, b"x", src=QEMU_NIC_MAC)]), **allow)
    assert "mac" in {f["kind"] for f in r["failures"]}, "the hardware address after the OS started"
    v6router = _eth(b"\x60" + bytes(39), src=bytes.fromhex("525600000002"), ethertype=0x86DD)
    assert judge(_pcap([v6router] + clean), **allow)["status"] == "pass", "slirp's IPv6 router"

    for bad in (b"", b"\x00" * 10, b"not a pcap at all, not even close!!"):
        try:
            judge(bad)
        except Unreadable:
            continue
        raise AssertionError(f"{bad!r} was read as a capture")

    import random

    rng = random.Random(0x5EED)
    base = _pcap(clean + list(cases.values()))
    for _ in range(3000):
        b = bytearray(base)
        for _ in range(rng.randint(1, 12)):
            b[rng.randrange(24, len(b))] = rng.randrange(256)
        b = bytes(b[: rng.randrange(24, len(b) + 1)])
        r = judge(b)
        assert r["status"] in ("pass", "fail", "empty")
    assert judge(_pcap([]))["status"] == "empty", "a capture the guest sent nothing in"
    print("nonos-pcap-egress: self-test passed")
    return 0


def main(argv=None):
    ap = argparse.ArgumentParser(description=__doc__.split("\n", 1)[0])
    ap.add_argument("pcap", nargs="?")
    ap.add_argument("--self-test", action="store_true", help="check the rules on captures built here")
    ap.add_argument("--allow", action="append", default=[], help="an IPv4 peer that is expected")
    ap.add_argument("--allow-name", action="append", default=[], help="a TLS server name or HTTP host that is expected")
    ap.add_argument("--host-mac", action="append", default=[],
                    help="a user-net host MAC, whose frames are not the guest's (default: QEMU slirp's two)")
    ap.add_argument("--nic-mac", default=mac_text(QEMU_NIC_MAC),
                    help="the NIC's own assigned MAC, which only the firmware may send from")
    ap.add_argument("--expect-silence", action="store_true",
                    help="a capture the guest sent nothing in is the claim (an airgapped image), not a gap")
    ap.add_argument("--json", help="write the report here as well")
    a = ap.parse_args(argv)
    if a.self_test:
        return self_test()
    if a.pcap is None:
        ap.error("a capture is required")
    try:
        with open(a.pcap, "rb") as fh:
            hosts = tuple(bytes.fromhex(m.replace(":", "")) for m in a.host_mac) or SLIRP_HOST_MACS
            report = judge(fh.read(), a.allow, a.allow_name, hosts, bytes.fromhex(a.nic_mac.replace(":", "")))
    except (OSError, Unreadable, ValueError) as e:
        print(f"[egress] cannot read {a.pcap}: {e}", file=sys.stderr)
        return 2
    for p in report["peers"]:
        mark = f"  {p['why']}"
        print(f"[egress] {p['proto']} {p['ip']:15} {p['port']:5} {p['frames']:6} frames{mark}")
    for name, n in report["tls_server_names"].items():
        print(f"[egress] tls {name} x{n}")
    for host, n in report["http_hosts"].items():
        print(f"[egress] http {host} x{n}")
    for params in report["dhcp_parameter_lists"]:
        print(f"[egress] dhcp parameter request list {params}")
    print(f"[egress] guest MACs {', '.join(report['guest_macs']) or 'none'}")
    for f in report["failures"]:
        print(f"[egress] FAIL {f['kind']}: {f['what']}")
    print(f"[egress] {report['status'].upper()}: {report['guest_frames']} guest frames, "
          f"{report['firmware_frames']} firmware frames, of {report['frames']}")
    if a.json:
        with open(a.json, "w") as fh:
            json.dump(report, fh, indent=1)
    if report["status"] == "empty" and not a.expect_silence:
        print("[egress] the guest sent nothing: the capture shows no path, so it proves none clean")
        return 3
    return 1 if report["failures"] else 0
