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
"""A content's Debian pins are read whole or refused, by the rules the
device reads them by (install/deb/pinned.rs; capsule_linux_proofs
tools_tests/debs.rs holds the same lines there). Run:
python3 -m unittest discover -s tools/nonos_market_catalogue -t tools"""

import unittest

from . import asset_needs
from .asset_needs import carried, lines, parse
from .tarwalk import Entry

SUM = "b1c7d1a0f00a5d3c9e2f2b6f2a6e1f0c4d1e9b8a7c6d5e4f3a2b1c0d9e8f7a6b"
GMP = "libgmp10 2:6.3.0+dfsg-3 pool/main/g/gmp/libgmp10_6.3.0+dfsg-3_amd64.deb"


class AssetNeeds(unittest.TestCase):
    def test_lines_read_back_as_written(self):
        text = f"{GMP} {SUM}\nlibc6 2.41-12 pool/main/g/glibc/libc6_2.41-12_amd64.deb {SUM}\n"
        got = parse(text)
        self.assertEqual([p[0] for p in got], ["libgmp10", "libc6"])
        self.assertEqual(lines(got), text)

    def test_one_bad_line_refuses_the_whole_file(self):
        for bad in ("", f"{GMP} {SUM[1:]}", f"{GMP} {SUM} extra", f"{GMP}  {SUM}", f"{GMP} {SUM}\n{GMP} {SUM}",
                    f"Libgmp10 1 pool/main/g/gmp/x.deb {SUM}", f"-gmp 1 pool/main/g/gmp/x.deb {SUM}",
                    f"g 1 pool/main/g/gmp/x.deb {SUM}", f"libgmp10 1 pool/../../etc/x.deb {SUM}",
                    f"libgmp10 1 pool//x.deb {SUM}", f"libgmp10 1 pool/main/g/gmp/x.tar {SUM}",
                    f"libgmp10 1\x7f pool/main/g/gmp/x.deb {SUM}", f"{GMP} {SUM.upper()}"):
            with self.assertRaises(ValueError, msg=bad):
                parse(bad)

    def test_past_the_bound_the_file_is_refused(self):
        line = lambda i: f"lib{i:03} 1 pool/main/l/lib/lib{i}.deb {SUM}\n"  # noqa: E731
        self.assertEqual(len(parse("".join(map(line, range(asset_needs.MAX_NEEDS))))), asset_needs.MAX_NEEDS)
        with self.assertRaises(ValueError):
            parse("".join(map(line, range(asset_needs.MAX_NEEDS + 1))))

    def test_the_pins_are_found_at_the_packages_own_path_and_last_copy(self):
        at = (asset_needs.DIR + "sx").encode()
        files = [Entry(at, "file", b"junk"), Entry(at, "file", f"{GMP} {SUM}\n".encode())]
        self.assertEqual(carried(files, "sx")[0][0], "libgmp10")
        self.assertIsNone(carried(files, "other"))
        with self.assertRaises(ValueError):
            carried([Entry(at, "file", "é".encode())], "sx")
