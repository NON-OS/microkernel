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
"""The signed catalogue of the NONOS model repository, built from the pins.

It lists every tier the signed personality pins and every part of a split
tier, each with its pin's length and SHA-256. The bytes are decoded again
and compared with the pins, and nothing is written unless the two agree
byte for byte. A file names the NONOS repository first when the build is
given one (--nonos-mirror), then its upstream file. The catalogue is signed
with the marketplace operator's Ed25519 seed, the key the market's index is
signed with, and verified under the operator's public key before it is kept.
"""

import sys

from .ed25519 import public, sign, verify
from .layout import mirrors
from .shapes import memory
from .wire import encode
from .wire_read import decode


def entries(table, base):
    return [(tier, memory(tier, sum(s for _, s, _ in files)),
             [(n, s, d, mirrors(base, tier, n)) for n, s, d in files])
            for tier, files in table.items()]


def agree(tiers, table):
    """Whether `tiers` names exactly the pinned files, in the pins' order."""
    listed = [(t, [(n, s, d) for n, s, d, _ in files]) for t, _, files in tiers]
    return listed == [(t, [tuple(f) for f in files]) for t, files in table.items()]


def body(table, base, serial):
    raw = encode(serial, base, entries(table, base))
    if not agree(decode(raw + bytes(64))[2], table):
        sys.exit("catalogue: it and the pins disagree; nothing written")
    return raw


def signed(raw, seed, key):
    if len(key) != 32 or public(seed) != key:
        sys.exit("catalogue: the operator seed does not belong to the operator public key")
    sig = sign(seed, raw)
    if not verify(key, raw, sig):
        sys.exit("catalogue: the signature does not verify; nothing written")
    return raw + sig
