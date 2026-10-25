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
"""An RSA key or exponent outside the device's bounds, DER that is not
minimal or whole, a base64 body that is not strict, and a signature not
below n are each refused. Run:
python3 -m unittest discover -s tools/nonos_market_catalogue -t tools"""

import base64
import unittest

from . import rsa, rsa_key
from .rsa_fixture import DIGEST, block, key, pem, sign_block, spki

N, E, D = key()


class Rsa(unittest.TestCase):
    def test_a_well_formed_key_reads_and_anything_more_or_less_is_refused(self):
        self.assertEqual(rsa_key.public_key(pem(spki(N, E))), (N, E))
        good = spki(N, E)
        for bad in (spki(N, E, b"\x05\x00"), good + b"\x00", good[:-1], spki(N, E, e_pad=b"\x00"),
                    spki(N, E, long_alg=True), spki(N, E, unused=b"\x01")):
            with self.assertRaises(ValueError, msg=bad[-8:].hex()):
                rsa_key.public_key(pem(bad))
        text = base64.b64encode(good)
        with self.assertRaises(ValueError):
            rsa_key.public_key(b"-----BEGIN PUBLIC KEY-----\n" + text[:40] + b"!" + text[40:] + b"\n")

    def test_bounds_on_the_key_the_exponent_and_the_signature(self):
        good = sign_block(N, D, block(N, DIGEST))
        self.assertTrue(rsa.verify_key(N, E, good, "sha1", DIGEST))
        for n, e in ((N, 2), (N, 1 << 33 | 1), (N >> 1100, E), (N << 2100, E)):
            self.assertFalse(rsa.verify_key(n, e, good, "sha1", DIGEST), (n.bit_length(), e))
        self.assertFalse(rsa.verify_key(N, 1, block(N, DIGEST), "sha1", DIGEST), "e = 1 signs nothing")
        n, e, d = key(bits=1024)
        self.assertFalse(rsa.verify_key(n, e, sign_block(n, d, block(n, DIGEST)), "sha1", DIGEST), "1024 bits")
        over = (int.from_bytes(good, "big") + N).to_bytes(len(good) + 1, "big")[1:]
        self.assertFalse(rsa.verify_key(N, E, over, "sha1", DIGEST), "s >= n")


if __name__ == "__main__":
    unittest.main()
