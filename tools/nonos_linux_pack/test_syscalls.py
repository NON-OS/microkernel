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

"""The measured set: the union of every run, both exits in, bucket 512 out,
with the same two constants the Rust policy crate holds."""

import re
import tempfile
import unittest
import unittest.mock
from pathlib import Path

from .syscalls import ALWAYS_SERVED, NATIVE_BUCKET, apps, buckets, main, market_set

MARKET_RS = Path(__file__).parents[2] / "userland/nonos_linux_policy/src/market.rs"


def line(bits) -> bytes:
    s = bytearray(65)
    for b in bits:
        s[b // 8] |= 1 << (b % 8)
    return b"[PROVE] syscall buckets ok=" + s.hex().encode() + b"\n"


def measured(app: str, bits_on) -> bytes:
    return b"[MEASURE] app=" + app.encode() + b" ok=" + line(bits_on)[27:]


def bits(hexset: str) -> set:
    raw = bytes.fromhex(hexset)
    return {b for b in range(513) if raw[b // 8] >> (b % 8) & 1}


class Syscalls(unittest.TestCase):
    def test_the_runs_unite_with_both_exits_and_no_native_bucket(self):
        log = b"boot\n" + line({0, 1, 512}) + b"noise\n" + line({59, 202})
        got = bits(market_set(buckets(log)))
        self.assertEqual(got, {0, 1, 59, 202, 60, 231})

    def test_a_bit_past_512_or_a_short_line_is_refused(self):
        bad = b"[PROVE] syscall buckets ok=" + b"00" * 64 + b"02"
        self.assertRaises(ValueError, buckets, bad)
        self.assertEqual(buckets(b"[PROVE] syscall buckets ok=" + b"00" * 64), [])

    def test_the_constants_are_the_policy_crates(self):
        rs = MARKET_RS.read_text()
        self.assertIn(f"ALWAYS_SERVED: [u64; 2] = {list(ALWAYS_SERVED)};", rs)
        self.assertIn(f"NATIVE_BUCKET: u64 = {NATIVE_BUCKET};", rs)

    def test_main_prints_the_calls_word(self):
        with tempfile.TemporaryDirectory() as d:
            a, b = Path(d, "qemu.log"), Path(d, "hw.log")
            a.write_bytes(line({0})), b.write_bytes(line({1}))
            with unittest.mock.patch("sys.stdout") as out:
                self.assertEqual(main(["--from-log", str(a), "--from-log", str(b)]), 0)
            printed = "".join(c.args[0] for c in out.write.call_args_list)
            self.assertTrue(re.fullmatch(r"calls=[0-9a-f]{130}\n", printed))
            self.assertEqual(bits(printed[6:136]), {0, 1, 60, 231})


    def test_a_measure_line_counts_only_for_its_app(self):
        qwen, perl = "00112233aabbccdd", "ffeeddcc00000001"
        log = measured(qwen, {0, 9}) + measured(perl, {41}) + measured(qwen, {202}) + line({59})
        self.assertEqual(bits(market_set(buckets(log, qwen))), {0, 9, 202, 60, 231})
        self.assertEqual(bits(market_set(buckets(log))), {59, 60, 231}, "a [MEASURE] line read as [PROVE]")
        self.assertEqual(apps(log), {qwen: 2, perl: 1})

    def test_the_kernel_says_the_line_this_reads(self):
        rs = (MARKET_RS.parents[3] / "src/process/foreign/seal/measure_line.rs").read_text()
        self.assertIn('const HEAD: &[u8] = b"[MEASURE] app=";', rs)
        self.assertIn('const OK: &[u8] = b" ok=";', rs)
        self.assertIn("id[..8].iter()", rs, "the app is named by its first 8 bytes, 16 hex digits")

    def test_main_names_the_apps_when_none_is_chosen_and_refuses_a_bad_one(self):
        with tempfile.TemporaryDirectory() as d:
            a = Path(d, "measure.log")
            a.write_bytes(measured("00112233aabbccdd", {0}))
            with unittest.mock.patch("sys.stderr") as err:
                self.assertEqual(main(["--from-log", str(a)]), 1)
                self.assertEqual(main(["--from-log", str(a), "--app", "XYZ"]), 2)
            said = "".join(c.args[0] for c in err.write.call_args_list)
            self.assertIn("[MEASURE] app=00112233aabbccdd: 1 run(s)", said)
            with unittest.mock.patch("sys.stdout"):
                self.assertEqual(main(["--from-log", str(a), "--app", "00112233aabbccdd"]), 0)
