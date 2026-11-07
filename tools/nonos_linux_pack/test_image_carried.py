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

"""The image leaves out what the device refuses as the NONOS image's own
files (image_carried.py, capsule_linux member/image.rs), and nothing else:
the rest of a package's bytes stay in their order, and a package with none
is its own tar, byte for byte."""

import io
import tarfile
import unittest

from nonos_market_catalogue.tarwalk import walk

from . import image_carried, image_split

IMAGE = ["/etc/ssl/cert.pem", "/usr/bin/python3", "/usr/bin/sqlite3"]


def tar(items, fmt=tarfile.USTAR_FORMAT) -> bytes:
    """A tar of (name, kind, body or link target), as Alpine packs one."""
    buf = io.BytesIO()
    with tarfile.open(fileobj=buf, mode="w", format=fmt) as t:
        for name, kind, value in items:
            info = tarfile.TarInfo(name)
            info.mtime, info.mode = 0, 0o755
            if kind == "file":
                info.size = len(value)
                t.addfile(info, io.BytesIO(value))
                continue
            info.type = {"dir": tarfile.DIRTYPE, "symlink": tarfile.SYMTYPE, "hardlink": tarfile.LNKTYPE}[kind]
            info.linkname = value or ""
            t.addfile(info)
    return buf.getvalue()


def names(data: bytes) -> list:
    return [e.name.decode() for e in walk(data)]


class Carried(unittest.TestCase):
    def test_the_list_is_the_store_catalogues_linux_group_from_the_root(self):
        files = image_carried.carried()
        self.assertIn("/usr/bin/python3", files)
        self.assertIn("/etc/ssl/cert.pem", files)
        self.assertEqual(files, sorted(set(files)))
        self.assertTrue(all(f.startswith("/") and not f.startswith("/linux/") for f in files))

    def test_a_file_the_image_carries_is_left_out_and_the_rest_kept_in_order(self):
        python3 = tar([(".PKGINFO", "file", b"pkgname = python3\n"),
                       ("usr/bin/", "dir", None),
                       ("usr/bin/python3", "symlink", "python3.12"),
                       ("usr/bin/python3.12", "file", b"\x7fELF python"),
                       ("usr/lib/libpython3.12.so.1.0", "file", b"\x7fELF lib")])
        out, gone = image_carried.without("python3", python3, IMAGE)
        self.assertEqual(gone, ["/usr/bin/python3"])
        self.assertEqual(names(out), [".PKGINFO", "usr/bin", "usr/bin/python3.12", "usr/lib/libpython3.12.so.1.0"])
        kept = [e for e in image_split.entries(python3) if b"usr/bin/python3\0" not in e[:100]]
        self.assertEqual(out, b"".join(kept) + image_split.END, "the package's own bytes, one entry fewer")
        self.assertEqual(len(out) % 512, 0)

    def test_a_package_with_none_of_them_is_its_own_tar(self):
        other = tar([("usr/bin/python3.12", "file", b"x"), ("usr/share/doc/sqlite3", "file", b"y")])
        self.assertEqual(image_carried.without("other", other, IMAGE), (other, []))

    def test_a_link_at_a_directory_above_one_is_left_out_but_a_directory_is_not(self):
        linked = tar([("etc/ssl", "symlink", "../usr/share/ssl"), ("usr/bin/", "dir", None)])
        out, gone = image_carried.without("ssl", linked, IMAGE)
        self.assertEqual(gone, ["/etc/ssl"])
        self.assertEqual(names(out), ["usr/bin"])

    def test_a_long_name_goes_with_the_record_before_it(self):
        long = "usr/share/" + "d" * 120 + "/x"
        image = IMAGE + ["/" + long]
        data = tar([(long, "file", b"long"), ("usr/bin/env", "file", b"env")], fmt=tarfile.PAX_FORMAT)
        out, gone = image_carried.without("long", data, sorted(image))
        self.assertEqual(gone, ["/" + long])
        self.assertEqual(names(out), ["usr/bin/env"])
        self.assertNotIn(b"d" * 120, out, "no pax record is left without its entry")

    def test_a_hard_link_kept_to_an_entry_left_out_is_refused(self):
        data = tar([("usr/bin/sqlite3", "file", b"db"), ("usr/bin/sqlite3-again", "hardlink", "usr/bin/sqlite3")])
        with self.assertRaises(ValueError):
            image_carried.without("sqlite", data, IMAGE)


if __name__ == "__main__":
    unittest.main()
