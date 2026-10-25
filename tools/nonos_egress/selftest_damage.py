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
"""Damaged captures, minimums, and the self-test's entry."""

import json
import random
import struct
from pathlib import Path

from .allow import BadAllowList, load, parse
from .judge import judge
from .pcap import Unreadable
from .scrape import scrape
from .selftest import ALLOW, CLEAN, INGRESS, clean, run
from .selftest_frames import GUEST, dhcp, pcap, tcp
from .selftest_dhcp import dhcp_rules
from .selftest_rules import evasions, identity, rules


def unreadable(b, why):
    try:
        judge(b, b"", ALLOW)
    except Unreadable:
        return
    raise AssertionError(f"{why}: read as a capture")


def damage():
    for bad in (b"", b"\x00" * 10, b"not a pcap at all, not even close!!"):
        unreadable(bad, repr(bad))
    whole = pcap(CLEAN)
    unreadable(whole + b"\x00" * 7, "bytes left over after the last record")
    lie = bytearray(pcap(CLEAN[:1]))
    struct.pack_into("<I", lie, 24 + 12, 10)  # orig_len below incl_len
    unreadable(bytes(lie), "a record longer than its own original length")
    rng = random.Random(0x5EED)
    ends, off = [], 24
    while off < len(whole):
        n = struct.unpack_from("<I", whole, off + 8)[0]
        ends.append((off, off + 16 + n))
        off += 16 + n
    for _ in range(2000):
        start, end = ends[rng.randrange(len(ends))]
        unreadable(whole[: rng.randrange(start + 1, end)], "a capture cut inside a record")
    for _ in range(2000):
        b = bytearray(whole)
        for _ in range(rng.randint(1, 12)):
            b[rng.randrange(24, len(b))] = rng.randrange(256)
        try:
            r = judge(bytes(b), pcap(INGRESS), ALLOW)
        except Unreadable:
            continue
        assert r["status"] in ("pass", "fail", "empty", "incomplete")


def minimums():
    assert run([])["status"] == "empty", "a capture the guest sent nothing in"
    only_dhcp = [dhcp([(53, b"\x01"), (61, b"\x01" + GUEST)])]
    assert run(only_dhcp)["status"] == "incomplete", "DHCP and nothing pinned reached"
    assert run(CLEAN[2:])["status"] == "incomplete", "a pinned peer and no DHCP"


def allowlist():
    here = Path(__file__).resolve().parent
    assert scrape(here.parent.parent) == json.loads((here / "allowlist.json").read_text()), \
        "allowlist.json differs from what the sources pin"
    load(here / "allowlist.json")
    for bad in ({}, {"format": "nonos-egress-allow/0", "names": [], "peers": []},
                {"format": "nonos-egress-allow/1", "names": [], "peers": [{"ip": "1.2.3.4"}]}):
        try:
            parse(bad)
        except BadAllowList:
            continue
        raise AssertionError(f"{bad} was taken as an allow-list")


def self_test():
    clean()
    rules()
    evasions()
    identity()
    dhcp_rules()
    damage()
    minimums()
    allowlist()
    print("nonos-pcap-egress: self-test passed")
    return 0
