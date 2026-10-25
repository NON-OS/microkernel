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
"""A hostile index record never reaches the fetch, a bad publisher key is
refused, and no document name can leave its directory. Run:
python3 -m unittest discover -s tools/nonos_market_catalogue -t tools"""

import unittest

from .apk_fixture import ELF, checksum, make
from .catalogue import catalogue_entries
from .documents import split
from .recipes import UNMEASURED

BASE = {"package": "x", "licence": "MIT", "mask": "TTY", "name": "X", "text": "x"}
RECIPE = dict(BASE, mask=0x100, dests=None, calls=UNMEASURED)
RAW = make([("usr/bin/x", ELF)])
REC = {"P": "x", "V": "1-r0", "L": "MIT", "base": "https://example.org", "C": checksum(RAW), "S": str(len(RAW))}


class Builders(unittest.TestCase):
    def test_a_hostile_index_record_never_reaches_the_fetch(self):
        asked = []
        for rec in (dict(REC, V="../../evil"), dict(REC, V=None), dict(REC, L=None), dict(REC, S="0"),
                    dict(REC, S="-1"), dict(REC, S=str((128 << 20) + 1)), dict(REC, S=None)):
            got = catalogue_entries([RECIPE], {"x": rec}, lambda r: asked.append(r) or b"", "11" * 32, 1)
            self.assertEqual(got, [])
        self.assertEqual(asked, [])
        self.assertEqual(len(catalogue_entries([RECIPE], {"x": REC}, lambda r: RAW, "11" * 32, 1)), 1)

    def test_a_publisher_key_that_is_not_32_bytes_of_hex_is_refused(self):
        for key in ("11" * 31, "zz" * 32, ""):
            with self.assertRaises(ValueError, msg=key):
                catalogue_entries([RECIPE], {}, lambda r: b"", key, 1)

    def test_a_document_name_that_could_leave_the_directory_is_refused(self):
        for entries in ([{"listing_id": "linux../../x"}], [{"listing_id": "/etc/x"}], ["x"],
                        [{"listing_id": "linux.a"}, {"listing_id": "linux.a"}]):
            with self.assertRaises(ValueError, msg=entries):
                split({"schema_version": 3, "entries": entries})
        for index in ({"entries": []}, {"schema_version": 2, "entries": []}, [], {"schema_version": 3}):
            with self.assertRaises((ValueError, TypeError), msg=index):
                split(index)


if __name__ == "__main__":
    unittest.main()
