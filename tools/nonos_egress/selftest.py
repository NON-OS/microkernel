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
"""The judge on captures built here: a clean boot, one capture per rule, the
evasions an image under test could try, and damaged files."""

from .allow import Allow
from .judge import judge
from .selftest_frames import GUEST, arp, consensus, dhcp, hello, offer, pcap, reply, tcp

AUTH = ("49.13.145.234", 9230)
ALLOW = Allow(names=frozenset({"validator.nymtech.net", "www.abcdefgh.net"}),
              peers=frozenset({("92.39.63.14", 443), ("188.172.228.15", 9000), AUTH}),
              authorities=frozenset({AUTH}))
GET = b"GET /tor/status-vote/current/consensus-microdesc.z HTTP/1.0\r\n\r\n"
CLEAN = [
    dhcp([(53, b"\x01"), (61, b"\x01" + GUEST), (55, b"\x01\x03\x06")]),
    arp(GUEST),
    tcp((92, 39, 63, 14), 443, hello("validator.nymtech.net")),
    tcp((188, 172, 228, 15), 9000, b"\x00" * 40),
    tcp((49, 13, 145, 234), 9230, GET),
    tcp((5, 6, 7, 8), 9001, hello("www.abcdefgh.net")),
]
INGRESS = [offer(), reply((49, 13, 145, 234), 9230, consensus("5.6.7.8", "9.9.9.9"))]


def run(egress, ingress=INGRESS, allow=ALLOW):
    return judge(pcap(egress), pcap(ingress) if ingress is not None else b"", allow)


def kinds(r):
    return {f["kind"] for f in r["failures"]}


def caught(kind, frames_, why, ingress=INGRESS, naming=""):
    """`kind` fails the capture, on a finding that names `naming`."""
    r = run(CLEAN + frames_, ingress)
    hit = [f for f in r["failures"] if f["kind"] == kind and naming in f["what"]]
    assert hit and r["status"] == "fail", f"{why}: {kind} {naming} not caught: {r['failures']}"


def clean():
    r = run(CLEAN)
    assert r["status"] == "pass", r["failures"]
    assert r["guest_frames"] == 6 and r["relays_learned"] == 2, r
    assert r["tls_server_names"] == {"validator.nymtech.net": 1, "www.abcdefgh.net": 1}
    assert r["dhcp_parameter_lists"] == [[1, 3, 6]]
    want = {"188.172.228.15": "pinned", "49.13.145.234": "pinned", "5.6.7.8": "relay", "92.39.63.14": "pinned"}
    assert {p["ip"]: p["why"] for p in r["peers"]} == want, r["peers"]
