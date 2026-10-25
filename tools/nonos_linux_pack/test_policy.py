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
"""The policy command against the Rust crate it must agree with: the bytes
nonos_linux_policy encodes for the same policy and list, its mask constants,
and the E39 ids, which the project recomputed with cast."""

import re
import unittest
from pathlib import Path

from .policy import MASK, encode, listing_id, market_ready, release_id

CRATE = Path(__file__).parents[2] / "userland/nonos_linux_policy/src"
# From the crate: LinuxAppPolicy { [0x11;32], WAYLAND|NET_ANON|MODELS_RO|TTY,
# [0x22;32], [0x33;32], [0x44;32], calls {0,1,60,231,511} }.encode().
CALLS = "03000000000000100000000000000000000000000000000000000000800000000000" \
        "00000000000000000000000000000000000000000000000000000000008000"
POLICY = ("4e4c58504f4c3031010000000000" + "00" * 2 + "11" * 32 + "2301000000000000" + "22" * 32
          + "33" * 32 + "44" * 32 + CALLS + "00" * 39)


class Policy(unittest.TestCase):
    def test_the_bytes_are_the_crates(self):
        got = encode(bytes([0x11]) * 32, 0b100100011, bytes([0x22]) * 32, bytes([0x33]) * 32,
                     bytes([0x44]) * 32, bytes.fromhex(CALLS))
        self.assertEqual(got.hex(), POLICY)

    def test_the_mask_names_are_the_crates(self):
        consts = dict(re.findall(r"pub const (\w+): u64 = 1 << (\d+);", (CRATE / "mask.rs").read_text()))
        self.assertEqual({k: 1 << int(v) for k, v in consts.items()}, MASK)

    def test_e39_ids(self):
        pub = bytes.fromhex("bbeaa6c15c496575134e0c72d8d75d7d6128c9fdf3723761f44026a0adf522f2")
        lid = listing_id(pub, "linux.qwen-small")
        self.assertEqual(lid.hex(), "ff4399cc04b31c66714c8be18d9b33e33912a83ccbc7ede50fce0a31d8414cd5")
        self.assertEqual(release_id(lid, "qwen-small@1").hex(),
                         "af2e230eabc03a74fb38620601f259a086d9924a715b352c66d09c23694a864f")

    def test_a_set_that_is_not_market_ready_is_refused(self):
        calls = bytearray.fromhex(CALLS)
        for bucket, flip in ((60, 0), (231, 0), (512, 1)):
            bad = bytearray(calls)
            bad[bucket // 8] = bad[bucket // 8] | (1 << bucket % 8) if flip else bad[bucket // 8] & ~(1 << bucket % 8)
            with self.assertRaises(ValueError):
                encode(bytes(32), 0, bytes(32), bytes(32), bytes(32), bytes(bad))

    def test_files_user_is_reserved(self):
        calls = bytes.fromhex(CALLS)
        self.assertTrue(market_ready(calls, MASK["FILES_HOME"]))
        self.assertFalse(market_ready(calls, MASK["FILES_USER"]))
        with self.assertRaisesRegex(ValueError, "FILES_USER is reserved"):
            encode(bytes(32), MASK["FILES_USER"] | MASK["TTY"], bytes(32), bytes(32), bytes(32), calls)


if __name__ == "__main__":
    unittest.main()
