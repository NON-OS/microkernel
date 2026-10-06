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
"""Ed25519 (RFC 8032) signing and verifying for the model catalogue, with
the marketplace operator's 32-byte seed, as the market's index is signed.
"""

import hashlib

from .ed25519_point import BASE, P, Q, add, compress, mul, recover_x


def decompress(b):
    y = int.from_bytes(b, "little")
    x = recover_x(y & ((1 << 255) - 1), y >> 255)
    y &= (1 << 255) - 1
    return None if x is None else (x, y, 1, x * y % P)


def equal(a, b):
    return (a[0] * b[2] - b[0] * a[2]) % P == 0 and (a[1] * b[2] - b[1] * a[2]) % P == 0


def _sha512(*parts):
    return hashlib.sha512(b"".join(parts)).digest()


def _expand(seed):
    if len(seed) != 32:
        raise ValueError("an Ed25519 seed is 32 bytes")
    h = _sha512(seed)
    a = int.from_bytes(h[:32], "little") & ((1 << 254) - 8) | (1 << 254)
    return a, h[32:]


def public(seed):
    return compress(mul(_expand(seed)[0], BASE))


def sign(seed, msg):
    a, prefix = _expand(seed)
    key = compress(mul(a, BASE))
    r = int.from_bytes(_sha512(prefix, msg), "little") % Q
    big_r = compress(mul(r, BASE))
    h = int.from_bytes(_sha512(big_r, key, msg), "little") % Q
    return big_r + int.to_bytes((r + h * a) % Q, 32, "little")


def verify(key, msg, sig):
    if len(key) != 32 or len(sig) != 64:
        return False
    a, r, s = decompress(key), decompress(sig[:32]), int.from_bytes(sig[32:], "little")
    if a is None or r is None or s >= Q:
        return False
    h = int.from_bytes(_sha512(sig[:32], key, msg), "little") % Q
    return equal(mul(s, BASE), add(r, mul(h, a)))
