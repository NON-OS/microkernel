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
"""Argon2's memory fill and tag (RFC 9106 section 3), on argon2_ref.py."""

import struct

from argon2_ref import g, h, hp


def argon2(pw, salt, secret, ad, p, t_len, m, t, y):
    fields = b"".join(struct.pack("<I", len(f)) + f for f in (pw, salt, secret, ad))
    h0 = h(64, struct.pack("<6I", p, t_len, m, t, 0x13, y) + fields)
    q = 4 * p * (m // (4 * p)) // p
    sl, mp = q // 4, q * p
    blk = lambda b: list(struct.unpack("<128Q", b))
    mem = [[blk(hp(1024, h0 + struct.pack("<2I", c, i))) for c in (0, 1)] + [None] * (q - 2)
           for i in range(p)]
    for r in range(t):
        for s in range(4):
            for lane in range(p):
                fill(mem, r, s, lane, (p, q, sl, mp, t, y))
    c = mem[0][q - 1]
    for lane in range(1, p):
        c = [a ^ b for a, b in zip(c, mem[lane][q - 1])]
    return hp(t_len, struct.pack("<128Q", *c))


def fill(mem, r, s, lane, shape):
    p, q, sl, mp, t, y = shape
    indep = y == 1 or (y == 2 and r == 0 and s < 2)
    inp, addr = [r, lane, s, mp, t, y] + [0] * 122, None
    start = 2 if r == 0 and s == 0 else 0
    for idx in range(start, sl):
        if indep and (idx % 128 == 0 or idx == start == 2):
            inp[6] += 1
            addr = g([0] * 128, g([0] * 128, inp))
        j = s * sl + idx
        prev = mem[lane][j - 1] if j else mem[lane][q - 1]
        rnd = addr[idx % 128] if indep else prev[0]
        rl = lane if r == 0 and s == 0 else (rnd >> 32) % p
        done = s * sl if r == 0 else q - sl
        area = done + idx - 1 if rl == lane else done - (idx == 0)
        rel = area - 1 - ((area * (((rnd & 0xFFFFFFFF) ** 2) >> 32)) >> 32)
        st = 0 if r == 0 or s == 3 else (s + 1) * sl
        new = g(prev, mem[rl][(st + rel) % q])
        mem[lane][j] = [a ^ b for a, b in zip(new, mem[lane][j])] if r else new


if __name__ == "__main__":
    k = (b"\x01" * 32, b"\x02" * 16, b"\x03" * 8, b"\x04" * 12)
    for y in (0, 1, 2):
        print(y, argon2(*k, 4, 32, 32, 3, y).hex())
    print(argon2(b"password", b"somesaltsomesalt", b"", b"", 1, 100, 1024, 2, 2).hex())
    print(argon2(b"", b"saltsalt", b"", b"", 2, 4, 64, 1, 2).hex())
    print(argon2(b"correct horse battery staple", bytes(32), b"", b"", 4, 64, 256, 3, 2).hex())
