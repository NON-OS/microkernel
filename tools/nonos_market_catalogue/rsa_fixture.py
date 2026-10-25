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
"""A deterministic RSA key a test can sign with, and hand-built DER."""

import base64
import functools
import hashlib
import random

from .rsa import PREFIX
from .rsa_key import ALGORITHM


def probable_prime(r: random.Random, bits: int) -> int:
    while True:
        n = r.getrandbits(bits) | (1 << (bits - 1)) | 1
        if all(pow(a, n - 1, n) == 1 for a in (2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37)):
            return n


@functools.cache
def key(seed: int = 1, bits: int = 2048):
    """(n, e, d) of a test key: two primes from a fixed seed."""
    r = random.Random(seed)
    e = 65537
    while True:
        p, q = probable_prime(r, bits // 2), probable_prime(r, bits // 2)
        phi = (p - 1) * (q - 1)
        if p != q and phi % e and (p * q).bit_length() == bits:
            return p * q, e, pow(e, -1, phi)


def sign_block(n: int, d: int, block: bytes) -> bytes:
    return pow(int.from_bytes(block, "big"), d, n).to_bytes((n.bit_length() + 7) // 8, "big")


def block(n: int, digest: bytes, kind: bytes = b"\x01", hashname: str = "sha1") -> bytes:
    """The padded block, with `kind` as its block type (01 is signing)."""
    k, info = (n.bit_length() + 7) // 8, PREFIX[hashname] + digest
    return b"\x00" + kind + b"\xff" * (k - 3 - len(info)) + b"\x00" + info

DIGEST = hashlib.sha1(b"covered").digest()


def lv(tag: int, body: bytes, long: bool = False) -> bytes:
    size = len(body)
    count = 0 if size < 0x80 and not long else (size.bit_length() + 7) // 8 or 1
    head = bytes([size]) if not count else bytes([0x80 | count]) + size.to_bytes(count, "big")
    return bytes([tag]) + head + body


def integer(v: int, pad: bytes = b"") -> bytes:
    return lv(0x02, pad + v.to_bytes(v.bit_length() // 8 + 1, "big"))  # one sign byte when needed


def spki(n: int, e: int, extra=b"", e_pad=b"", long_alg=False, unused=b"\x00") -> bytes:
    inner = lv(0x30, integer(n) + integer(e, e_pad))
    return lv(0x30, lv(0x30, ALGORITHM, long_alg) + lv(0x03, unused + inner) + extra)


def pem(der: bytes) -> bytes:
    return b"-----BEGIN PUBLIC KEY-----\n" + base64.b64encode(der) + b"\n-----END PUBLIC KEY-----\n"
