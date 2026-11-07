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

"""GTK's caches as a member: the loader cache's paths as the device reads
them, one tar for the same files whatever the order, a name from its
content, and nothing for a closure with no GTK tool."""

import unittest

from . import image_gtk, image_members, image_vector


class Gtk(unittest.TestCase):
    def test_the_loader_cache_names_device_paths_and_no_build_directory(self):
        raw = (b"# LoaderDir = /tmp/x/root/usr/lib/gdk-pixbuf-2.0/2.10.0/loaders\n#\n"
               b'"/tmp/x/root/usr/lib/gdk-pixbuf-2.0/2.10.0/loaders/libpixbufloader-png.so"\n'
               b'"png" 5 "gdk-pixbuf" "PNG" "LGPL"\n')
        out = image_gtk.loaders_text(raw, "/tmp/x/root")
        self.assertNotIn(b"/tmp/x", out)
        self.assertNotIn(b"#", out)
        self.assertIn(b'"/usr/lib/gdk-pixbuf-2.0/2.10.0/loaders/libpixbufloader-png.so"', out)

    def test_the_same_files_make_the_same_tar(self):
        a = image_gtk.tar({"usr/share/nonos-gtk/b": b"2", "usr/share/nonos-gtk/a": b"1"})
        b = image_gtk.tar({"usr/share/nonos-gtk/a": b"1", "usr/share/nonos-gtk/b": b"2"})
        self.assertEqual(a, b)
        self.assertEqual(len(a) % 512, 0)

    def test_a_closure_without_gtk_has_no_caches_member(self):
        self.assertIsNone(image_gtk.caches(image_members.members(image_vector.packages())))
