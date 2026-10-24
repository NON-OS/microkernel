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
dependency: a public key's DER is read for n and e, the signature is raised
to e mod n, and the result must equal the one padded DigestInfo exactly."""

import base64

# DigestInfo prefixes (RFC 8017, 9.2 note 1).
PREFIX = {
    "sha1": bytes.fromhex("3021300906052b0e03021a05000414"),
    "sha256": bytes.fromhex("3031300d060960864801650304020105000420"),
}


def der(data: bytes, at: int, tag: int):
    """(body start, body end) of the TLV of `tag` at `at`, or ValueError."""
    if at + 2 > len(data) or data[at] != tag:
        raise ValueError("not the DER expected")
    size, at = data[at + 1], at + 2
    if size & 0x80:
        count = size & 0x7F
        if count == 0 or count > 4 or at + count > len(data):
            raise ValueError("a DER length is malformed")
        size, at = int.from_bytes(data[at:at + count], "big"), at + count
    if at + size > len(data):
        raise ValueError("a DER length runs past its data")
    return at, at + size


def public_key(pem: bytes):
    """(n, e) of a PEM SubjectPublicKeyInfo holding an RSA key."""
    lines = [x for x in pem.splitlines() if x and not x.startswith(b"-----")]
    spki = base64.b64decode(b"".join(lines), validate=True)
    outer, _ = der(spki, 0, 0x30)
    alg_start, alg_end = der(spki, outer, 0x30)
    if b"\x2a\x86\x48\x86\xf7\x0d\x01\x01\x01" not in spki[alg_start:alg_end]:
        raise ValueError("not an RSA key")
    bits, _ = der(spki, alg_end, 0x03)
    if spki[bits:bits + 1] != b"\x00":
        raise ValueError("the key's bit string has unused bits")
    seq, _ = der(spki, bits + 1, 0x30)
    n_start, n_end = der(spki, seq, 0x02)
    e_start, e_end = der(spki, n_end, 0x02)
    return int.from_bytes(spki[n_start:n_end], "big"), int.from_bytes(spki[e_start:e_end], "big")


def verify(pem: bytes, signature: bytes, hashname: str, digest: bytes) -> bool:
    """True when `signature` is the key's PKCS#1 v1.5 signature over `digest`."""
    return verify_key(*public_key(pem), signature, hashname, digest)


def verify_key(n: int, e: int, signature: bytes, hashname: str, digest: bytes) -> bool:
    k = (n.bit_length() + 7) // 8
    if len(signature) != k or hashname not in PREFIX or k < 128:
        return False
    s = int.from_bytes(signature, "big")
    if s >= n:
        return False
    info = PREFIX[hashname] + digest
    want = b"\x00\x01" + b"\xff" * (k - 3 - len(info)) + b"\x00" + info
    return pow(s, e, n).to_bytes(k, "big") == want
