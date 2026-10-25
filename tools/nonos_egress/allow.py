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
"""The allow-list: the names the image may send in the clear, and the
(address, port) pairs it pins, versioned. The committed file is the one a
reviewer read; scrape.py fails the run when the sources say otherwise."""

import json
from dataclasses import dataclass, field

FORMAT = "nonos-egress-allow/1"


class BadAllowList(Exception):
    pass


@dataclass(frozen=True)
class Allow:
    names: frozenset = frozenset()
    peers: frozenset = frozenset()  # {(ip, port)} the image pins
    authorities: frozenset = frozenset()  # the peers whose consensus names relays
    what: dict = field(default_factory=dict, compare=False)


def parse(doc):
    if not isinstance(doc, dict) or doc.get("format") != FORMAT:
        raise BadAllowList(f"not {FORMAT}")
    try:
        names = frozenset(str(n) for n in doc["names"])
        rows = [(str(p["ip"]), int(p["port"]), str(p["what"]), bool(p.get("authority"))) for p in doc["peers"]]
    except (KeyError, TypeError, ValueError) as e:
        raise BadAllowList(f"malformed: {e}") from e
    peers = frozenset((ip, port) for ip, port, _, _ in rows)
    auth = frozenset((ip, port) for ip, port, _, a in rows if a)
    return Allow(names, peers, auth, {(ip, port): w for ip, port, w, _ in rows})


def load(path):
    with open(path) as f:
        return parse(json.load(f))
