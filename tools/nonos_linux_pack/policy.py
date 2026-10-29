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
"""A release's guest_policy: the 256 bytes of nonos_linux_policy's layout
(src/layout.rs), held to market_ready (src/market.rs), its ids by
marketplace_abi's rule, version 2 with SERVE_ONE's field (serve.py).
test_policy.py holds the bytes to the crate's."""

from .keccak import keccak256
from .serve import VERSION_SERVE, field

MAGIC, VERSION, LEN, SET_LEN = b"NLXPOL01", 1, 256, 65
MASK = {"WAYLAND": 1 << 0, "NET_ANON": 1 << 1, "NET_LISTEN_LO": 1 << 2,
        "FILES_HOME": 1 << 3, "FILES_USER": 1 << 4, "MODELS_RO": 1 << 5,
        "EXEC": 1 << 6, "JIT": 1 << 7, "TTY": 1 << 8, "SERVE_ONE": 1 << 9}
ALWAYS_SERVED, NATIVE_BUCKET, SERVED = (60, 231), 512, MASK["SERVE_ONE"] | MASK["WAYLAND"]


def listing_id(publisher: bytes, app: str) -> bytes:
    """keccak256(abi.encode(LISTING_TAG, publisher, keccak256(app)))."""
    tag = keccak256(b"NONOS.nox.listing.v1")
    return keccak256(tag + publisher + keccak256(app.encode()))


def release_id(listing: bytes, release: str) -> bytes:
    """keccak256(abi.encode(RELEASE_TAG, listing_id, keccak256(release)))."""
    tag = keccak256(b"NONOS.nox.release.v1")
    return keccak256(tag + listing + keccak256(release.encode()))


def mask_of(names: str) -> int:
    words = [w for w in names.replace(",", " ").split() if w]
    unknown = [w for w in words if w not in MASK]
    if unknown or "SERVE_ONE" in words:
        raise ValueError(f"no service is named {unknown[0]}" if unknown else "SERVE_ONE is set by serve, not named")
    return sum({MASK[w] for w in words})


def market_ready(calls: bytes, mask: int) -> bool:
    has = lambda b: calls[b // 8] >> (b % 8) & 1
    sets = all(has(b) for b in ALWAYS_SERVED) and not has(NATIVE_BUCKET)
    return sets and not mask & MASK["FILES_USER"] and mask & SERVED != SERVED


def encode(program: bytes, mask: int, listing: bytes, release: bytes,
           dests_hash: bytes, calls: bytes, serve: tuple = None) -> bytes:
    if len(calls) != SET_LEN or calls[-1] & 0xFE:
        raise ValueError("calls is not a 65-byte set of buckets 0 to 512")
    if mask & MASK["FILES_USER"]:
        raise ValueError("FILES_USER is reserved: nothing serves the person's files yet")
    if bool(mask & MASK["SERVE_ONE"]) != (serve is not None):
        raise ValueError("SERVE_ONE needs version 2's port and name, and they need SERVE_ONE")
    if mask & SERVED == SERVED:
        raise ValueError("SERVE_ONE with WAYLAND is not market ready")
    if not market_ready(calls, mask):
        raise ValueError("calls must name exit (60) and exit_group (231) and leave bucket 512 clear")
    for name, got in (("program", program), ("listing", listing), ("release", release), ("dests", dests_hash)):
        if len(got) != 32:
            raise ValueError(f"{name} is not 32 bytes")
    version = (VERSION_SERVE if serve else VERSION).to_bytes(2, "little")
    out = MAGIC + version + bytes(6) + program + mask.to_bytes(8, "little")
    out += listing + release + dests_hash + calls + (field(serve) if serve else b"")
    return out + bytes(LEN - len(out))
