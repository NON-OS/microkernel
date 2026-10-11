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
"""BLAKE3-256, the reference algorithm without its parallelism: the policy's
dests hash, and a program's hash when b3sum is not installed. Checked against
the official test vectors in test_hashes.py."""

IV = [0x6A09E667, 0xBB67AE85, 0x3C6EF372, 0xA54FF53A, 0x510E527F, 0x9B05688C, 0x1F83D9AB, 0x5BE0CD19]
PERM = [2, 6, 3, 10, 7, 0, 4, 13, 1, 11, 12, 5, 9, 14, 15, 8]
CHUNK_START, CHUNK_END, PARENT, ROOT = 1, 2, 4, 8
M32 = 0xFFFFFFFF


def g(s, a, b, c, d, x, y):
    s[a] = (s[a] + s[b] + x) & M32
    s[d] = ((s[d] ^ s[a]) >> 16 | (s[d] ^ s[a]) << 16) & M32
    s[c] = (s[c] + s[d]) & M32
    s[b] = ((s[b] ^ s[c]) >> 12 | (s[b] ^ s[c]) << 20) & M32
    s[a] = (s[a] + s[b] + y) & M32
    s[d] = ((s[d] ^ s[a]) >> 8 | (s[d] ^ s[a]) << 24) & M32
    s[c] = (s[c] + s[d]) & M32
    s[b] = ((s[b] ^ s[c]) >> 7 | (s[b] ^ s[c]) << 25) & M32


def compress(cv, block: bytes, counter: int, length: int, flags: int) -> list:
    m = [int.from_bytes(block[4 * i:4 * i + 4], "little") for i in range(16)]
    s = cv[:] + IV[:4] + [counter & M32, counter >> 32, length, flags]
    for r in range(7):
        for i, (a, b, c, d) in enumerate([(0, 4, 8, 12), (1, 5, 9, 13), (2, 6, 10, 14), (3, 7, 11, 15),
                                          (0, 5, 10, 15), (1, 6, 11, 12), (2, 7, 8, 13), (3, 4, 9, 14)]):
            g(s, a, b, c, d, m[2 * i], m[2 * i + 1])
        if r < 6:
            m = [m[p] for p in PERM]
    return [s[i] ^ s[i + 8] for i in range(8)]


def chunk_cv(chunk: bytes, counter: int, last_flags: int) -> list:
    """The chaining value of one chunk; last_flags is ROOT for a lone chunk."""
    cv, blocks = IV[:], [chunk[i:i + 64] for i in range(0, max(len(chunk), 1), 64)]
    for i, block in enumerate(blocks):
        flags = (CHUNK_START if i == 0 else 0) | (CHUNK_END | last_flags if i == len(blocks) - 1 else 0)
        cv = compress(cv, block.ljust(64, b"\0"), counter, len(block), flags)
    return cv


def blake3(data: bytes) -> bytes:
    chunks = [data[i:i + 1024] for i in range(0, max(len(data), 1), 1024)]
    if len(chunks) == 1:
        cv = chunk_cv(chunks[0], 0, ROOT)
        return b"".join(w.to_bytes(4, "little") for w in cv)
    stack = []
    for n, chunk in enumerate(chunks[:-1]):
        cv = chunk_cv(chunk, n, 0)
        total = n + 1
        while total & 1 == 0:
            cv = compress(IV, b"".join(w.to_bytes(4, "little") for w in stack.pop() + cv), 0, 64, PARENT)
            total >>= 1
        stack.append(cv)
    cv = chunk_cv(chunks[-1], len(chunks) - 1, 0)
    while stack:
        flags = PARENT | (ROOT if len(stack) == 1 else 0)
        cv = compress(IV, b"".join(w.to_bytes(4, "little") for w in stack.pop() + cv), 0, 64, flags)
    return b"".join(w.to_bytes(4, "little") for w in cv)
