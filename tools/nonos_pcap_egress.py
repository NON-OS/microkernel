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

The capture holds both directions. A frame is the guest's when its source MAC
is not the user-net host's (52:55:0a:00:02:02 under QEMU's slirp); only those
are judged. Each one is classified, and these fail the run:

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

Everything else is listed: each TCP and UDP peer with a count, the DHCP
parameter list, every TLS server name and HTTP host. A TCP or UDP peer outside
--allow is marked `listed`, not failed: the Nym and Anyone clients reach
gateways and relays they learn from a directory at run time, which no fixed
list can name, and the checks above are what tells a relay from a leak.

The verdict and every finding go to --json as well. Exit 0 pass, 1 fail,
2 when the capture cannot be read.
"""

import argparse
import json
import struct
import sys
from collections import Counter

SLIRP_HOST_MAC = bytes.fromhex("52550a000202")

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


def judge(data, allow_ips=(), allow_names=(), host_mac=SLIRP_HOST_MAC):
    """The report for a capture's bytes: every guest frame classified."""
    allow_ips, allow_names = set(allow_ips), set(allow_names)
    failures, peers = [], Counter()
    seen = {"frames": 0, "guest_frames": 0, "macs": set(), "dhcp_params": [], "sni": Counter(), "http": Counter()}

    def fail(kind, what):
        failures.append({"kind": kind, "what": what})

    for f in frames(data):
        seen["frames"] += 1
        if len(f) < 14:
            continue
        dst_mac, src_mac, ethertype = f[0:6], f[6:12], struct.unpack_from(">H", f, 12)[0]
        if src_mac == host_mac:
            continue
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
            if sport == 68 and dport == 67:
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
        {"proto": p, "ip": ip, "port": port, "frames": n, "allowed": ip in allow_ips}
        for (p, ip, port), n in sorted(peers.items())
    ]
    return {
        "status": "fail" if failures else "pass",
        "frames": seen["frames"],
        "guest_frames": seen["guest_frames"],
        "guest_macs": sorted(mac_text(m) for m in seen["macs"]),
        "dhcp_parameter_lists": seen["dhcp_params"],
        "tls_server_names": dict(seen["sni"]),
        "http_hosts": dict(seen["http"]),
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


def _inbound():
    return SLIRP_HOST_MAC[:0] + GUEST + SLIRP_HOST_MAC + b"\x08\x00" + _ip(17, (10, 0, 2, 15), struct.pack(">HHHH", 53, 40000, 8, 0), src=(10, 0, 2, 3))


def self_test():
    clean = [
        _dhcp([(53, b"\x01"), (55, b"\x01\x03\x06"), (61, b"\x01" + GUEST)]),
        _tcp((92, 39, 63, 14), 443, _hello("validator.nymtech.net")),
        _tcp((188, 172, 228, 15), 9000, b"\x00" * 40),
        _inbound(),
    ]
    allow = {"allow_names": ["validator.nymtech.net"], "allow_ips": ["92.39.63.14"]}
    r = judge(_pcap(clean), **allow)
    assert r["status"] == "pass", r["failures"]
    assert r["guest_frames"] == 3, "the host's reply to the guest is not judged"
    assert r["tls_server_names"] == {"validator.nymtech.net": 1}
    assert r["dhcp_parameter_lists"] == [[1, 3, 6]]
    assert [p["allowed"] for p in r["peers"]] == [False, True], r["peers"]

    cases = {
        "dns": _udp((10, 0, 2, 3), 53, b"\x00" * 20),
        "ntp": _udp((162, 159, 200, 1), 123, b"\x23" + bytes(47)),
        "discovery": _udp((224, 0, 0, 251), 5353, b"\x00" * 12),
        "ipv6": _eth(b"\x60" + bytes(39), ethertype=0x86DD),
        "mac": _udp((92, 39, 63, 14), 9999, b"x", src=BURNED),
        "dhcp-id": _dhcp([(53, b"\x01"), (12, b"nonos-laptop")]),
        "sni": _tcp((1, 2, 3, 4), 443, _hello("tracker.example")),
        "http": _tcp((1, 2, 3, 4), 80, b"GET / HTTP/1.1\r\nHost: telemetry.example\r\n\r\n"),
    }
    for kind, frame in cases.items():
        r = judge(_pcap(clean + [frame]), **allow)
        kinds = {f["kind"] for f in r["failures"]}
        assert kind in kinds, f"{kind} not caught: {r['failures']}"
        assert r["status"] == "fail"
    dirauth = _tcp((92, 39, 63, 14), 9230, b"GET /tor/status-vote/current/consensus HTTP/1.0\r\n\r\n")
    assert judge(_pcap([dirauth]), **allow)["status"] == "pass", "a pinned authority, asked without a Host"
    assert judge(_pcap([dirauth]))["status"] == "fail", "the same request to a peer nobody pinned"
    tcp_dns = _tcp((10, 0, 2, 3), 853, b"\x00" * 10)
    assert {f["kind"] for f in judge(_pcap([tcp_dns]))["failures"]} == {"dns"}, "DNS over TLS"

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
        assert r["status"] in ("pass", "fail")
    print("nonos-pcap-egress: self-test passed")
    return 0


def main(argv=None):
    ap = argparse.ArgumentParser(description=__doc__.split("\n", 1)[0])
    ap.add_argument("pcap", nargs="?")
    ap.add_argument("--self-test", action="store_true", help="check the rules on captures built here")
    ap.add_argument("--allow", action="append", default=[], help="an IPv4 peer that is expected")
    ap.add_argument("--allow-name", action="append", default=[], help="a TLS server name or HTTP host that is expected")
    ap.add_argument("--host-mac", default=mac_text(SLIRP_HOST_MAC), help="the user-net host's MAC, whose frames are not the guest's")
    ap.add_argument("--json", help="write the report here as well")
    a = ap.parse_args(argv)
    if a.self_test:
        return self_test()
    if a.pcap is None:
        ap.error("a capture is required")
    try:
        with open(a.pcap, "rb") as fh:
            report = judge(fh.read(), a.allow, a.allow_name, bytes.fromhex(a.host_mac.replace(":", "")))
    except (OSError, Unreadable, ValueError) as e:
        print(f"[egress] cannot read {a.pcap}: {e}", file=sys.stderr)
        return 2
    for p in report["peers"]:
        mark = "" if p["allowed"] else "  listed"
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
    print(f"[egress] {report['status'].upper()}: {report['guest_frames']} guest frames of {report['frames']}")
    if a.json:
        with open(a.json, "w") as fh:
            json.dump(report, fh, indent=1)
    return 1 if report["failures"] else 0
