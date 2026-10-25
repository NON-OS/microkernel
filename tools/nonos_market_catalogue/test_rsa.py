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
"""rsa.py on a real Alpine signature: it verifies under the pinned key and
fails on a changed digest, signature, hash name, length or key. Run:
python3 -m unittest discover -s tools/nonos_market_catalogue -t tools"""

import base64
import hashlib
import unittest

from . import alpine_index, gz, rsa, tarwalk
from .rsa_fixture import DIGEST, block, key, sign_block
from .vector_fixture import APK, flip


class Rsa(unittest.TestCase):
    def test_alpine_s_own_signature_verifies_and_nothing_else_does(self):
        parts = gz.members(APK)
        (sig,) = tarwalk.files(parts[0].body)
        key_pem = (alpine_index.KEYS / sig.name.decode()[len(".SIGN.RSA."):]).read_bytes()
        digest = hashlib.sha1(APK[parts[1].start:parts[1].end]).digest()
        self.assertTrue(rsa.verify(key_pem, sig.body, "sha1", digest))
        self.assertFalse(rsa.verify(key_pem, sig.body, "sha1", flip(digest, 0)))
        self.assertFalse(rsa.verify(key_pem, flip(sig.body, 100), "sha1", digest))
        self.assertFalse(rsa.verify(key_pem, sig.body, "sha256", hashlib.sha256(b"").digest()))
        self.assertFalse(rsa.verify(key_pem, sig.body[:-1], "sha1", digest))
        other = (alpine_index.KEYS / alpine_index.NAMES[0]).read_bytes()
        self.assertFalse(rsa.verify(other, sig.body, "sha1", digest))
        self.assertFalse(rsa.verify(key_pem, b"\x00" + sig.body, "sha1", digest))
        der = base64.b64decode(b"".join(key_pem.splitlines()[1:-1]))
        oid = bytes.fromhex("2a864886f70d010101")  # rsaEncryption; ...0105 is sha1WithRSA
        renamed = b"-----BEGIN PUBLIC KEY-----\n" + base64.b64encode(der.replace(oid, oid[:-1] + b"\x05"))
        with self.assertRaises(ValueError):
            rsa.public_key(renamed + b"\n-----END PUBLIC KEY-----\n")


class Padding(unittest.TestCase):
    def test_only_the_exact_signing_block_verifies(self):
        n, e, d = key()
        self.assertTrue(rsa.verify_key(n, e, sign_block(n, d, block(n, DIGEST)), "sha1", DIGEST))
        for bad in (block(n, DIGEST, kind=b"\x02"), block(n, DIGEST, hashname="sha256")[:-32] + DIGEST + bytes(12),
                    b"\x00\x01" + bytes(len(block(n, DIGEST)) - 22) + DIGEST):
            self.assertFalse(rsa.verify_key(n, e, sign_block(n, d, bad), "sha1", DIGEST))


if __name__ == "__main__":
    unittest.main()
