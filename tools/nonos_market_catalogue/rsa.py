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
"""RSA PKCS#1 v1.5 signature verification for Alpine's keys, with no
dependency: the key read as strict DER (rsa_key.py) and held to the bounds
the device's RSA accepts, the signature raised to e mod n, and the result
equal to the one padded DigestInfo, byte for byte."""

from .rsa_key import public_key

# DigestInfo prefixes (RFC 8017, 9.2 note 1).
PREFIX = {
    "sha1": bytes.fromhex("3021300906052b0e03021a05000414"),
    "sha256": bytes.fromhex("3031300d060960864801650304020105000420"),
}


def verify(pem: bytes, signature: bytes, hashname: str, digest: bytes) -> bool:
    """True when `signature` is the key's PKCS#1 v1.5 signature over `digest`."""
    return verify_key(*public_key(pem), signature, hashname, digest)


def verify_key(n: int, e: int, signature: bytes, hashname: str, digest: bytes) -> bool:
    """False for a key outside 2048 to 4096 bits or an exponent outside
    3 to 2^33, odd, as well as for any signature that is not exact."""
    if not (2048 <= n.bit_length() <= 4096 and 3 <= e < 1 << 33 and e & 1):
        return False
    k = (n.bit_length() + 7) // 8
    if len(signature) != k or hashname not in PREFIX:
        return False
    s = int.from_bytes(signature, "big")
    if s >= n:
        return False
    info = PREFIX[hashname] + digest
    want = b"\x00\x01" + b"\xff" * (k - 3 - len(info)) + b"\x00" + info
    return pow(s, e, n).to_bytes(k, "big") == want
