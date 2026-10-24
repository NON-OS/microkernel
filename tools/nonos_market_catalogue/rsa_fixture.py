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
"""A small RSA key made here, deterministically, so a test can sign: the
verifier's padding and length rules are then tested against signatures no
real key would ever have made."""

import hashlib
import random

from .rsa import PREFIX


def probable_prime(r: random.Random, bits: int) -> int:
    while True:
        n = r.getrandbits(bits) | (1 << (bits - 1)) | 1
        if all(pow(a, n - 1, n) == 1 for a in (2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37)):
            return n


def key(seed: int = 1, bits: int = 1024):
    """(n, e, d) of a test key: two 512-bit primes from a fixed seed."""
    r = random.Random(seed)
    e = 65537
    while True:
        p, q = probable_prime(r, bits // 2), probable_prime(r, bits // 2)
        phi = (p - 1) * (q - 1)
        if p != q and phi % e and (p * q).bit_length() == bits:
            return p * q, e, pow(e, -1, phi)


def sign_block(n: int, d: int, block: bytes) -> bytes:
    k = (n.bit_length() + 7) // 8
    return pow(int.from_bytes(block, "big"), d, n).to_bytes(k, "big")


def block(n: int, digest: bytes, kind: bytes = b"\x01", hashname: str = "sha1") -> bytes:
    """The padded block, with `kind` as its block type (01 is signing)."""
    k, info = (n.bit_length() + 7) // 8, PREFIX[hashname] + digest
    return b"\x00" + kind + b"\xff" * (k - 3 - len(info)) + b"\x00" + info


DIGEST = hashlib.sha1(b"covered").digest()
