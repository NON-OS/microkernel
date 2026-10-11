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
"""A destination list in nonos_linux_policy::dests's canonical form,
from the items policies.txt's dests= takes, refusing what that crate refuses;
test_policy.py holds the bytes to the crate's own."""

import ipaddress

from .blake3 import blake3
from .dests_name import name_ok

DOMAIN = b"NONOS.linux.netdests.v1"
MAX_ENTRIES = 64


class Refused(ValueError):
    """A list the canonical form refuses."""


def entry(item: str) -> bytes:
    what, _, ports = item.rpartition(":")
    first, _, last = ports.partition("-")
    if not what or not first.isdigit() or not (last or first).isdigit():
        raise Refused(f"{item!r}: no ports")
    lo, hi = int(first), int(last or first)
    if not 1 <= lo <= hi <= 0xFFFF:
        raise Refused(f"{item!r}: ports")
    if "/" not in what:
        if not name_ok(what):
            raise Refused(f"{item!r}: not a name a list holds")
        kind, prefix, body = b"N", 0, what.encode()
    else:
        addr, _, plen = what.partition("/")
        v6 = addr.startswith("[") and addr.endswith("]")
        try:
            net = ipaddress.ip_network(f"{addr.strip('[]')}/{int(plen)}", strict=True)
        except ValueError as why:
            raise Refused(f"{item!r}: {why}") from None
        if net.version != (6 if v6 else 4):
            raise Refused(f"{item!r}: IPv6 goes in brackets")
        if v6 and net.prefixlen >= 96 and net.network_address.ipv4_mapped is not None:
            raise Refused(f"{item!r}: an IPv4 address is written as IPv4")
        kind, prefix, body = (b"\x06" if v6 else b"\x04"), net.prefixlen, net.network_address.packed
    return kind + bytes([prefix]) + lo.to_bytes(2, "big") + hi.to_bytes(2, "big") + bytes([len(body)]) + body


def dests_list(items: list) -> bytes:
    entries = sorted(entry(i.strip()) for i in items if i.strip())
    if not 1 <= len(entries) <= MAX_ENTRIES:
        raise Refused(f"{len(entries)} destinations: a list holds 1 to {MAX_ENTRIES}")
    if any(a == b for a, b in zip(entries, entries[1:])):
        raise Refused("a destination is named twice")
    return len(entries).to_bytes(2, "big") + b"".join(entries)


def dests_hash(listing: bytes) -> bytes:
    return blake3(DOMAIN + listing)
