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
"""The catalogue signer against the RFC 8032 section 7.1 test vectors.

    python3 -m unittest discover -s tools/nonos_qwen_tier -t tools
"""

import unittest

from .ed25519 import public, sign, verify
from .ed25519_point import Q

VECTORS = [
    ("9d61b19deffd5a60ba844af492ec2cc44449c5697b326919703bac031cae7f60",
     "d75a980182b10ab7d54bfed3c964073a0ee172f3daa62325af021a68f707511a", "",
     "e5564300c360ac729086e2cc806e828a84877f1eb8e5d974d873e06522490155"
     "5fb8821590a33bacc61e39701cf9b46bd25bf5f0595bbe24655141438e7a100b"),
    ("4ccd089b28ff96da9db6c346ec114e0f5b8a319f35aba624da8cf6ed4fb8a6fb",
     "3d4017c3e843895a92b70aa74d1b7ebc9c982ccf2ec4968cc0cd55f12af4660c", "72",
     "92a009a9f0d4cab8720e820b5f642540a2b27b5416503f8fb3762223ebdb69da"
     "085ac1e43e15996e458f3613d0f11d8c387b2eaeb4302aeeb00d291612bb0c00"),
    ("c5aa8df43f9f837bedb7442f31dcb7b166d38535076f094b85ce3a2e0b4458f7",
     "fc51cd8e6218a1a38da47ed00230f0580816ed13ba3303ac5deb911548908025", "af82",
     "6291d657deec24024827e69c3abe01a30ce548a284743a445e3680d7db5ac3ac"
     "18ff9b538d16f290ae67f760984dc6594a7c15e9716ed28dc027beceea1ec40a"),
]


class Rfc8032(unittest.TestCase):
    def test_vectors(self):
        for seed, key, msg, sig in VECTORS:
            seed, key, msg, sig = map(bytes.fromhex, (seed, key, msg, sig))
            self.assertEqual(public(seed), key)
            self.assertEqual(sign(seed, msg), sig)
            self.assertTrue(verify(key, msg, sig))

    def test_refuses_what_was_not_signed(self):
        seed, key, msg, sig = map(bytes.fromhex, VECTORS[2])
        self.assertFalse(verify(key, msg + b"\0", sig))
        self.assertFalse(verify(key, msg, sig[:5] + bytes([sig[5] ^ 1]) + sig[6:]))
        self.assertFalse(verify(public(bytes(32)), msg, sig))
        high = sig[:32] + int.to_bytes(int.from_bytes(sig[32:], "little") + Q, 32, "little")
        self.assertFalse(verify(key, msg, high))


if __name__ == "__main__":
    unittest.main()
