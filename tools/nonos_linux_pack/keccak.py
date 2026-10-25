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
"""Keccak-256, as Ethereum and marketplace_abi use it: the original Keccak
padding (0x01), not SHA3-256's (0x06), so Python's hashlib.sha3_256 is not
it. Slow and plain, for hashing a few names."""

RATE = 136
ROUNDS = [
    0x0000000000000001, 0x0000000000008082, 0x800000000000808A, 0x8000000080008000,
    0x000000000000808B, 0x0000000080000001, 0x8000000080008081, 0x8000000000008009,
    0x000000000000008A, 0x0000000000000088, 0x0000000080008009, 0x000000008000000A,
    0x000000008000808B, 0x800000000000008B, 0x8000000000008089, 0x8000000000008003,
    0x8000000000008002, 0x8000000000000080, 0x000000000000800A, 0x800000008000000A,
    0x8000000080008081, 0x8000000000008080, 0x0000000080000001, 0x8000000080008008,
]
ROT = [[0, 36, 3, 41, 18], [1, 44, 10, 45, 2], [62, 6, 43, 15, 61],
       [28, 55, 25, 21, 56], [27, 20, 39, 8, 14]]
MASK = (1 << 64) - 1


def rol(v: int, n: int) -> int:
    return ((v << n) | (v >> (64 - n))) & MASK if n else v


def permute(a: list) -> None:
    """keccak-f[1600] on a[x][y], in place."""
    for rc in ROUNDS:
        c = [a[x][0] ^ a[x][1] ^ a[x][2] ^ a[x][3] ^ a[x][4] for x in range(5)]
        d = [c[(x - 1) % 5] ^ rol(c[(x + 1) % 5], 1) for x in range(5)]
        for x in range(5):
            for y in range(5):
                a[x][y] ^= d[x]
        b = [[0] * 5 for _ in range(5)]
        for x in range(5):
            for y in range(5):
                b[y][(2 * x + 3 * y) % 5] = rol(a[x][y], ROT[x][y])
        for x in range(5):
            for y in range(5):
                a[x][y] = b[x][y] ^ (~b[(x + 1) % 5][y] & b[(x + 2) % 5][y])
        a[0][0] ^= rc


def keccak256(data: bytes) -> bytes:
    padded = bytearray(data) + b"\x01"
    padded += bytes(-len(padded) % RATE)
    padded[-1] |= 0x80
    a = [[0] * 5 for _ in range(5)]
    for at in range(0, len(padded), RATE):
        block = padded[at:at + RATE]
        for i in range(RATE // 8):
            a[i % 5][i // 5] ^= int.from_bytes(block[8 * i:8 * i + 8], "little")
        permute(a)
    return b"".join(a[i % 5][i // 5].to_bytes(8, "little") for i in range(4))
