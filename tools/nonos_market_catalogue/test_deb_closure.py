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
"""The Debian closure tools/nonos-source-asset pins is the one the device
would choose, and the libraries a .deb places are read from its data
member. Run: python3 -m unittest discover -s tools/nonos_market_catalogue -t tools"""

import io
import tarfile
import unittest
from unittest import mock

from . import deb_closure, deb_files


def member(name: str, body: bytes) -> bytes:
    """One ar member, padded to an even length as ar pads it."""
    head = name.ljust(16).encode() + b"0".ljust(12) + b"0".ljust(6) * 2 + b"644".ljust(8) + str(len(body)).ljust(10).encode()
    return head + b"`\n" + body + b"\n" * (len(body) & 1)


class DebClosure(unittest.TestCase):
    def test_the_closure_is_the_one_the_device_would_choose(self):
        table = {"app": {"Package": "app", "Depends": "libx (>= 1) | liby, virt:any"},
                 "libx": {"Package": "libx", "Pre-Depends": "libc6"}, "liby": {"Package": "liby"},
                 "real": {"Package": "real", "Provides": "virt (= 2)", "Depends": "libc6"}, "libc6": {"Package": "libc6"}}
        self.assertEqual([r["Package"] for r in deb_closure.closure(table, ["app"])], ["app", "libx", "libc6", "real"])
        with self.assertRaisesRegex(ValueError, "provides gone"):
            deb_closure.closure(table, ["gone"])
        with mock.patch.object(deb_closure, "MAX_NEEDS", 3), self.assertRaisesRegex(ValueError, "passes 3"):
            deb_closure.closure(table, ["app"])
        self.assertEqual(deb_closure.groups("a (>= 1), b | c:any, , d [amd64]"), [["a"], ["b", "c"], ["d"]])

    def test_a_deb_says_what_it_places_and_which_are_libraries(self):
        for mode in ("xz", "gz"):
            data = io.BytesIO()
            with tarfile.open(fileobj=data, mode=f"w:{mode}") as t:
                for name in ("./usr/lib/x86_64-linux-gnu/libgmp.so.10.5.0", "./usr/share/doc/x"):
                    t.addfile(tarfile.TarInfo(name), io.BytesIO(b""))
                link = tarfile.TarInfo("./usr/lib/x86_64-linux-gnu/libgmp.so.10")
                link.type, link.linkname = tarfile.SYMTYPE, "libgmp.so.10.5.0"
                t.addfile(link)
            deb = b"!<arch>\n" + member("debian-binary", b"2.0\n") + member(f"data.tar.{mode}", data.getvalue())
            self.assertEqual(deb_files.placed(deb), {"usr/lib/x86_64-linux-gnu/libgmp.so.10.5.0", "usr/share/doc/x",
                                                     "usr/lib/x86_64-linux-gnu/libgmp.so.10"})
        paths = {"usr/lib/x86_64-linux-gnu/libgmp.so.10", "usr/lib64/ld-linux-x86-64.so.2", "usr/share/doc/libc.so.6",
                 "usr/lib/x86_64-linux-gnu/gconv/x.so"}
        self.assertEqual(deb_files.libraries(paths), {"libgmp.so.10", "ld-linux-x86-64.so.2"})
        for bad in (b"not ar", b"!<arch>\n", b"!<arch>\n" + member("data.tar.gz", b"xx")):
            with self.assertRaises(ValueError):
                deb_files.data_tar(bad)
