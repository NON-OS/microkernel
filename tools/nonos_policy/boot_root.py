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
"""The boot-root record: which bootloader tree the release vouches for, at which epoch.

104 bytes, as nonos-boot-measure's record module reads them: the root (32),
the epoch as u64 little-endian, then ECDSA P-256 r and s, 32 bytes each,
big-endian, over SHA-256("NONOS-BOOT-ROOT-v1" || root || epoch). The epoch is the
release's rollback index; the kernel refuses a record below the TPM floor."""

import hashlib
import struct
import sys

DOMAIN = b"NONOS-BOOT-ROOT-v1"
P = 0xFFFFFFFF00000001
RECORD_LEN = 104


def canonical(root):
    """Every little-endian word of the root below the field modulus."""
    return all(w < P for w in struct.unpack("<4Q", root))


def message(root, epoch):
    return hashlib.sha256(DOMAIN + root + struct.pack("<Q", epoch)).digest()


def sign(key, root, epoch):
    """The record for `root` at `epoch`, signed prehashed and checked before return."""
    from cryptography.hazmat.primitives import hashes
    from cryptography.hazmat.primitives.asymmetric import ec, utils

    if not canonical(root):
        sys.exit("the root is not four canonical words; no gate would admit it")
    if not 0 <= epoch < 1 << 64:
        sys.exit(f"epoch {epoch} is not a u64")
    digest = message(root, epoch)
    algo = ec.ECDSA(utils.Prehashed(hashes.SHA256()))
    der = key.sign(digest, algo)
    key.public_key().verify(der, digest, algo)
    r, s = utils.decode_dss_signature(der)
    return root + struct.pack("<Q", epoch) + r.to_bytes(32, "big") + s.to_bytes(32, "big")


def verify(pub, record):
    """(root, epoch) of a record that verifies under `pub`, or an exit naming why not."""
    from cryptography.exceptions import InvalidSignature
    from cryptography.hazmat.primitives import hashes
    from cryptography.hazmat.primitives.asymmetric import ec, utils

    if len(record) != RECORD_LEN:
        sys.exit(f"a record is {RECORD_LEN} bytes, this is {len(record)}")
    root, (epoch,) = record[:32], struct.unpack("<Q", record[32:40])
    if not canonical(root):
        sys.exit("the record's root is not four canonical words")
    r, s = int.from_bytes(record[40:72], "big"), int.from_bytes(record[72:104], "big")
    der = utils.encode_dss_signature(r, s)
    try:
        pub.verify(der, message(root, epoch), ec.ECDSA(utils.Prehashed(hashes.SHA256())))
    except InvalidSignature:
        sys.exit("the record's signature does not verify under this key")
    return root, epoch
