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

"""The image builder's steps: the device's closure order (the same cases
capsule_linux_proofs' install_closure_tests.rs holds the device to), the
cut between entries, the members, the catalogue entry, and a whole build
from Alpine's own signed package (the catalogue's vector)."""

import json
import os
import tempfile
import unittest
from pathlib import Path

from nonos_market_catalogue import alpine_auth
from nonos_market_catalogue.tarwalk import walk
from nonos_market_catalogue.vector_fixture import APK, REC, flip

from . import image_build, image_entry, image_format, image_members, image_split, image_vector
from .image_closure import closure
from .image_index import Index

TEXT = (b"P:app\nV:1\nS:1000\nD:liba so:libb.so.1\n\nP:liba\nV:1\nD:libc\n\n"
        b"P:libb\nV:1\np:so:libb.so.1=1\nD:libc\n\nP:libc\nV:1\n\n")


def index(*texts) -> Index:
    ix = Index()
    for t in texts:
        ix.add(t, 0)
    return ix


def names(ix, name, most=10) -> list:
    return [p.name for p in closure(ix, name, (), most)]


class Closure(unittest.TestCase):
    def test_the_listed_package_first_then_each_need_as_popped_once(self):
        ix = index(TEXT)
        self.assertEqual(names(ix, "app"), ["app", "libb", "libc", "liba"])
        self.assertEqual(ix.by_name("app").size, 1000)

    def test_a_need_nothing_provides_or_too_many_ends_it(self):
        with self.assertRaises(ValueError):
            names(index(b"P:app\nV:1\nD:ghost\n\n"), "app")
        with self.assertRaises(ValueError):
            names(index(TEXT), "missing")
        with self.assertRaises(ValueError):
            names(index(TEXT), "app", 3)
        self.assertEqual(len(names(index(TEXT), "app", 4)), 4)

    def test_a_provider_named_later_and_install_ifs_package_are_in_it(self):
        ix = index(b"P:app\nV:1\nD:icu-libs qt5-qtbase\n\nP:icu-libs\nV:1\nD:icu-data\n\n"
                   b"P:icu-data-en\nV:1\np:icu-data=1\n\nP:icu-data-full\nV:1\np:icu-data=1\n\n"
                   b"P:qt5-qtbase\nV:1\nD:icu-data-full\n\n"
                   b"P:qt5-qtwayland\nV:1\ni:qt5-qtbase wayland-libs-server\n\n")
        got = names(ix, "app")
        self.assertIn("icu-data-full", got)
        self.assertIn("qt5-qtwayland", got)
        self.assertNotIn("icu-data-en", got)

    def test_conflicts_paths_and_constraints_are_not_needs(self):
        ix = index(b"P:app\nV:1\nD:!old /bin/sh cmd:ls pc:x libc>=2 so:libz.so.1\n\n"
                   b"P:libc\nV:2\n\nP:zlib\nV:1\np:so:libz.so.1=1.3\n\n")
        self.assertEqual(names(ix, "app"), ["app", "zlib", "libc"])

    def test_extras_come_after_the_listed_packages_needs(self):
        ix = index(TEXT, b"P:plugin\nV:1\n\n")
        self.assertEqual([p.name for p in closure(ix, "app", ("plugin",))][-1], "plugin")


class Split(unittest.TestCase):
    def test_a_small_package_is_one_member_and_every_entry_stays(self):
        _, tar = image_vector.packages()[0]
        parts = image_split.split(tar, held=True)
        self.assertEqual(len(parts), 1)
        self.assertEqual([e.name for e in walk(parts[0])], [e.name for e in walk(tar)])

    def test_a_cut_falls_between_entries_and_keeps_pax_records_with_theirs(self):
        big = {f"usr/share/x/{'f' * 120}{i}": bytes([i]) * (20 << 20) for i in range(5)}
        tar = image_vector.tar(big)
        parts = image_split.split(tar, held=False)
        self.assertGreater(len(parts), 1)
        joined = [e for p in parts for e in walk(p)]
        self.assertEqual([(e.name, e.body) for e in joined], [(e.name, e.body) for e in walk(tar)])
        for p in parts:
            self.assertTrue(image_split.fits(p, False))
            self.assertEqual(len(p) % 512, 0)
        self.assertEqual(image_split.names("x", 3), ["x", "x.p2", "x.p3"])

    def test_the_listed_package_is_never_cut_and_a_file_no_pack_holds_is_refused(self):
        big = {f"usr/share/x/{i}": bytes([i]) * (40 << 20) for i in range(2)}
        with self.assertRaises(ValueError):
            image_split.split(image_vector.tar(big), held=True)
        with self.assertRaises(ValueError):
            image_split.split(image_vector.tar({"usr/share/huge": bytes(70 << 20)}), held=False)
        with self.assertRaises(ValueError):
            image_split.entries(b"x" * 512)


class Members(unittest.TestCase):
    def test_xz_only_when_it_is_smaller(self):
        compressible = image_vector.tar({"a": b"a" * 100000})
        noise = os.urandom(8192)
        self.assertEqual(image_members.stored(compressible)[0], image_format.XZ)
        codec, body = image_members.stored(noise)
        self.assertEqual(codec, image_format.TAR)
        self.assertTrue(body == noise)

    def test_the_same_packages_make_the_same_members(self):
        a = image_members.members(image_vector.packages())
        b = image_members.members(image_vector.packages())
        self.assertEqual(a, b)
        self.assertEqual([p.name for p in a], [b"demo", b"libdemo", b"demo-data"])


class Build(unittest.TestCase):
    def setUp(self):
        self.dir = tempfile.TemporaryDirectory()
        self.cache = Path(self.dir.name)
        self.ix = Index()
        self.ix.add("\n".join(f"{k}:{v}" for k, v in REC.items()).encode() + b"\n\n", 0)
        self.at = self.cache / "main" / f"{REC['P']}-{REC['V']}.apk"
        self.at.parent.mkdir(parents=True)

    def tearDown(self):
        self.dir.cleanup()

    def test_alpines_own_package_builds_an_image_of_its_files(self):
        self.at.write_bytes(APK)
        built = image_build.build(self.ix, "file:///nowhere", self.cache, REC["P"], (), 1 << 16)
        files = alpine_auth.package_files(APK, alpine_auth.checksum(REC["C"]))
        self.assertEqual(built["members"][0].tar, files)
        self.assertEqual(built["measurement"], image_format.measurement(built["list"]))
        e = image_entry.entry(REC["P"], built, "mirror.example:80", 1 << 16)
        self.assertEqual(e["listing_id"], "linux.nonos-image-nitrokey-udev-rules")
        self.assertEqual(e["release_id"], "nonos-image-nitrokey-udev-rules@1.0.0-r2")
        self.assertEqual(e["package_hash"], built["measurement"].hex())
        self.assertEqual(e["package_url"], f"http://mirror.example:80/linux/{e['measurement']}.chunks")
        self.assertEqual(e["publisher_signature"], "")
        json.dumps(e)

    def test_a_package_off_its_chain_is_refused_and_dropped_from_the_cache(self):
        self.at.write_bytes(flip(APK, len(APK) - 40))
        with self.assertRaises(ValueError):
            image_build.build(self.ix, "file:///nowhere", self.cache, REC["P"], (), 1 << 16)
        self.assertFalse(self.at.exists())
