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
"""Edwards25519 arithmetic for the catalogue signer, after the reference
code in RFC 8032 section 6: extended coordinates, a double-and-add ladder.
It runs on the build host over public inputs and one seed, and is not
constant time; the capsule verifies with its own code (nonos_ed25519).
"""

P = 2**255 - 19
Q = 2**252 + 27742317777372353535851937790883648493
D = -121665 * pow(121666, P - 2, P) % P
IDENTITY = (0, 1, 1, 0)


def inv(x):
    return pow(x, P - 2, P)


def add(a, b):
    x1, y1, z1, t1 = a
    x2, y2, z2, t2 = b
    e, h = (y1 + x1) * (y2 + x2) % P, 2 * z1 * z2 % P
    f, g = (y1 - x1) * (y2 - x2) % P, 2 * t1 * t2 * D % P
    e, f, g, h = e - f, h - g, h + g, e + f
    return (e * f % P, g * h % P, f * g % P, e * h % P)


def mul(s, point):
    out = IDENTITY
    while s > 0:
        if s & 1:
            out = add(out, point)
        point, s = add(point, point), s >> 1
    return out


def recover_x(y, sign):
    """x for y on the curve with the given low bit, or None when there is none."""
    if y >= P:
        return None
    x2 = (y * y - 1) * inv(D * y * y + 1) % P
    x = pow(x2, (P + 3) // 8, P)
    if (x * x - x2) % P:
        x = x * pow(2, (P - 1) // 4, P) % P
    if (x * x - x2) % P or (x == 0 and sign):
        return None
    return P - x if x & 1 != sign else x


def compress(point):
    x, y, z, _ = point
    zi = inv(z)
    return int.to_bytes((y * zi % P) | ((x * zi % P) & 1) << 255, 32, "little")


BASE_X, BASE_Y = recover_x(4 * inv(5) % P, 0), 4 * inv(5) % P
BASE = (BASE_X, BASE_Y, 1, BASE_X * BASE_Y % P)
