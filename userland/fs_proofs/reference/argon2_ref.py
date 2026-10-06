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
"""An Argon2 (RFC 9106) written apart from the kernel's, over hashlib's
BLAKE2b, for the vectors in src/crypto/argon2_vectors.rs: G, P and H'.
`python3 argon2_fill.py` prints the three section 5 tags, then the vectors
the Rust tests pin."""

import hashlib
import struct

M64 = (1 << 64) - 1


def h(n, data):
    return hashlib.blake2b(data, digest_size=n).digest()


def hp(t, data):
    if t <= 64:
        return h(t, struct.pack("<I", t) + data)
    r = -(-t // 32) - 2
    v = h(64, struct.pack("<I", t) + data)
    out = v[:32]
    for _ in range(1, r):
        v = h(64, v)
        out += v[:32]
    return out + h(t - 32 * r, v)


def gb(v, a, b, c, d):
    f = lambda x, y: (x + y + 2 * (x & 0xFFFFFFFF) * (y & 0xFFFFFFFF)) & M64
    rot = lambda x, n: ((x >> n) | (x << (64 - n))) & M64
    for (x, y, z, n1, n2) in ((a, b, d, 32, 24), (a, b, d, 16, 63)):
        v[x] = f(v[x], v[y]); v[z] = rot(v[z] ^ v[x], n1)
        v[c] = f(v[c], v[z]); v[y] = rot(v[y] ^ v[c], n2)


def g(x, y):
    r = [a ^ b for a, b in zip(x, y)]
    q = list(r)
    quads = ((0, 4, 8, 12), (1, 5, 9, 13), (2, 6, 10, 14), (3, 7, 11, 15),
             (0, 5, 10, 15), (1, 6, 11, 12), (2, 7, 8, 13), (3, 4, 9, 14))
    for idx in [[16 * i + j for j in range(16)] for i in range(8)] + \
               [[2 * i + (j // 2) * 16 + j % 2 for j in range(16)] for i in range(8)]:
        for a, b, c, d in quads:
            gb(q, idx[a], idx[b], idx[c], idx[d])
    return [a ^ b for a, b in zip(q, r)]
