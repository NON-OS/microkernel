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
"""A public key read as strict DER, for rsa.py: exactly a SubjectPublicKey-
Info of rsaEncryption with NULL parameters, a bit string with no unused
bits, and two minimal positive integers; minimal lengths; nothing left
over at any level. Anything else is ValueError."""

import base64

ALGORITHM = bytes.fromhex("06092a864886f70d0101010500")


def tlv(data: bytes, at: int, tag: int):
    """(body start, body end) of the TLV of `tag` at `at`, minimal lengths."""
    if at + 2 > len(data) or data[at] != tag:
        raise ValueError("not the DER expected")
    size, at = data[at + 1], at + 2
    if size & 0x80:
        count = size & 0x7F
        if count == 0 or count > 4 or at + count > len(data) or data[at] == 0:
            raise ValueError("a DER length is malformed")
        size, at = int.from_bytes(data[at:at + count], "big"), at + count
        if size < 0x80:
            raise ValueError("a DER length is not minimal")
    return at, at + size  # a length past the data fails the whole-key checks


def integer(data: bytes, at: int):
    """(value, end) of a minimal, positive INTEGER at `at`."""
    start, end = tlv(data, at, 0x02)
    body = data[start:end]
    if not body or body[0] & 0x80 or (len(body) > 1 and body[0] == 0 and not body[1] & 0x80):
        raise ValueError("an INTEGER is negative or not minimal")
    return int.from_bytes(body, "big"), end


def public_key(pem: bytes):
    """(n, e) of a PEM SubjectPublicKeyInfo holding an RSA key."""
    lines = [x for x in pem.splitlines() if x and not x.startswith(b"-----")]
    spki = base64.b64decode(b"".join(lines), validate=True)
    outer, outer_end = tlv(spki, 0, 0x30)
    alg, alg_end = tlv(spki, outer, 0x30)
    if outer_end != len(spki) or spki[alg:alg_end] != ALGORITHM:
        raise ValueError("not exactly an rsaEncryption key")
    bits, bits_end = tlv(spki, alg_end, 0x03)
    if bits_end != outer_end or spki[bits:bits + 1] != b"\x00":
        raise ValueError("the key's bit string is not whole")
    seq, seq_end = tlv(spki, bits + 1, 0x30)
    n, n_end = integer(spki, seq)
    e, e_end = integer(spki, n_end)
    if seq_end != bits_end or e_end != seq_end:
        raise ValueError("the key holds more than n and e")
    return n, e
