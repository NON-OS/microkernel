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
"""The two hashes the policy command computes, against vectors the Rust
crates nonos_linux_policy builds with gave for the same inputs (blake3 =1,
sha3 0.10.9), which are BLAKE3's published test vectors."""

import unittest

from .blake3 import blake3
from .keccak import keccak256

BLAKE3 = {
    0: "af1349b9f5f9a1a6a0404dea36dcc9499bcb25c9adc112b7cc9a93cae41f3262",
    1: "2d3adedff11b61f14c886e35afa036736dcd87a74d27b5c1510225d0f592e213",
    1024: "42214739f095a406f3fc83deb889744ac00df831c10daa55189b5d121c855af7",
    1025: "d00278ae47eb27b34faecf67b4fe263f82d5412916c1ffd97c8cb7fb814b8444",
    2049: "5f4d72f40d7a5f82b15ca2b2e44b1de3c2ef86c426c95c1af0b6879522563030",
    8193: "bab6c09cb8ce8cf459261398d2e7aef35700bf488116ceb94a36d0f5f1b7bc3b",
}


class Hashes(unittest.TestCase):
    def test_blake3_over_one_chunk_and_a_tree(self):
        for n, want in BLAKE3.items():
            self.assertEqual(blake3(bytes(i % 251 for i in range(n))).hex(), want, n)

    def test_keccak_is_not_sha3(self):
        self.assertEqual(keccak256(b"").hex(),
                         "c5d2460186f7233c927e7db2dcc703c0e500b653ca82273b7bfad8045d85a470")
        self.assertEqual(keccak256(b"NONOS.nox.listing.v1").hex(),
                         "fc28c341928084c4d621b5d5ca20ad5fe5f9626b525ba55ca86203e2b12545b4")
        import hashlib
        self.assertNotEqual(keccak256(b"").hex(), hashlib.sha3_256(b"").hexdigest())

    def test_a_block_boundary_pads_into_a_second_block(self):
        self.assertEqual(len({keccak256(bytes(n)) for n in (135, 136, 137)}), 3)


if __name__ == "__main__":
    unittest.main()
