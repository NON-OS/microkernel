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

"""An in-tree tool's content is the same bytes wherever and whenever it is
made, holds its program and data but never its proofs, and is listed under
`linux.nonos-<package>` pinned by the BLAKE3 of the tar the mirror serves.
Run from the repository root:

    python3 -m unittest discover -s tools/nonos_market_catalogue -t tools
"""

import io
import json
import os
import tarfile
import tempfile
import time
import unittest
from pathlib import Path

from . import packages
from .digest import blake3

ELF = b"\x7fELF" + b"\x02" * 60
CAPSULES = [{"dir": "userland/linux_userland", "target": "x86_64-unknown-linux-musl",
             "bin": "userland_perl", "prebuilt": "target/linux-userland/perl"}]
PROGRAM = "userland/linux_userland/target/x86_64-unknown-linux-musl/release/userland_perl"
FILES = [
    {"path": "/linux/usr/bin/perl", "file": PROGRAM},
    {"path": "/linux/usr/bin/perl.nonos_id_cert.bin", "file": "nonos-data/trust/capsules/userland_perl.nonos_id_cert.bin"},
    {"path": "/linux/usr/bin/perl.manifest.bin", "file": "nonos-data/trust/capsules/userland_perl.manifest.bin"},
    {"path": "/linux/usr/bin/perl.zk_trailer.bin", "file": "nonos-data/trust/capsules/userland_perl.zk_trailer.bin"},
    {"path": "/linux/usr/lib/perl5/strict.pm", "file": "target/linux-userland/perl5/strict.pm"},
    {"path": "/linux/usr/lib/perl5/Data/Dumper.pm", "file": "target/linux-userland/perl5/Data/Dumper.pm"},
]


def tree(root: Path):
    for rel, body in (("target/linux-userland/perl", ELF),
                      ("target/linux-userland/perl5/strict.pm", b"package strict;\n1;\n"),
                      ("target/linux-userland/perl5/Data/Dumper.pm", b"package Data::Dumper;\n1;\n")):
        (root / rel).parent.mkdir(parents=True, exist_ok=True)
        (root / rel).write_bytes(body)


class Content(unittest.TestCase):
    def test_the_content_holds_the_program_and_data_but_no_proof(self):
        files = packages.content_files(FILES, CAPSULES)
        self.assertEqual(files, [
            ("usr/bin/perl", "target/linux-userland/perl"),
            ("usr/lib/perl5/Data/Dumper.pm", "target/linux-userland/perl5/Data/Dumper.pm"),
            ("usr/lib/perl5/strict.pm", "target/linux-userland/perl5/strict.pm"),
        ])

    def test_a_file_outside_the_linux_userland_is_refused(self):
        with self.assertRaises(ValueError):
            packages.content_files([{"path": "/linux/usr/bin/x", "file": "elsewhere/x"}], [])

    def test_the_bundle_is_the_same_bytes_whenever_and_in_whatever_order(self):
        with tempfile.TemporaryDirectory() as d:
            root = Path(d)
            tree(root)
            files = packages.content_files(FILES, CAPSULES)
            first = packages.bundle(files, root)
            time.sleep(1.1)
            for f in root.rglob("*"):
                os.utime(f, (12345, 12345))
            second = packages.bundle(list(reversed(files)), root)
            self.assertEqual(first, second)

    def test_the_bundle_reads_back_as_the_files_with_no_owner_or_time(self):
        with tempfile.TemporaryDirectory() as d:
            root = Path(d)
            tree(root)
            body = packages.bundle(packages.content_files(FILES, CAPSULES), root)
        with tarfile.open(fileobj=io.BytesIO(body)) as tar:
            members = tar.getmembers()
            self.assertEqual([m.name for m in members],
                             ["usr/bin/perl", "usr/lib/perl5/Data/Dumper.pm", "usr/lib/perl5/strict.pm"])
            self.assertTrue(all(m.mtime == 0 and m.uid == 0 and m.gid == 0 and m.uname == "" for m in members))
            self.assertEqual(members[0].mode, 0o755, "a program can be run")
            self.assertEqual(members[1].mode, 0o644, "data cannot")
            self.assertEqual(tar.extractfile(members[0]).read(), ELF)


    def test_a_path_longer_than_the_name_field_is_refused(self):
        with tempfile.TemporaryDirectory() as d:
            root = Path(d)
            long = "target/linux-userland/" + "a" * 120
            (root / long).parent.mkdir(parents=True, exist_ok=True)
            (root / long).write_bytes(b"x")
            with self.assertRaises(ValueError):
                packages.bundle([("usr/lib/" + "a" * 120, long)], root)


    def test_a_link_rides_in_the_content_as_a_symlink_to_a_full_path(self):
        with tempfile.TemporaryDirectory() as d:
            root = Path(d)
            tree(root)
            files = packages.content_files(FILES, CAPSULES)
            body = packages.bundle(files, root, {"usr/bin/perl5.44.0": "/usr/bin/perl"})
            self.assertEqual(body, packages.bundle(files, root, {"usr/bin/perl5.44.0": "/usr/bin/perl"}))
            with self.assertRaises(ValueError):
                packages.bundle(files, root, {"usr/bin/x": "perl"})
        with tarfile.open(fileobj=io.BytesIO(body)) as tar:
            link = tar.getmember("usr/bin/perl5.44.0")
            self.assertTrue(link.issym())
            self.assertEqual(link.linkname, "/usr/bin/perl")


class Listing(unittest.TestCase):
    def test_a_tool_is_listed_pinned_by_the_content_the_mirror_serves(self):
        with tempfile.TemporaryDirectory() as d:
            root = Path(d)
            tree(root)
            store = root / "store.json"
            store.write_text(json.dumps({"packages": {"perl": FILES}}))
            caps = root / "capsules.json"
            caps.write_text(json.dumps(CAPSULES))
            tools = root / "tools.json"
            tools.write_text(json.dumps([
                {"package": "perl", "name": "Perl", "text": "Perl."},
                {"package": "absent", "name": "Absent", "text": "Not a package."},
            ]))
            out = root / "packages"
            found = packages.package_entries(tools, store, caps, root, "mirror.example.org:80",
                                             out, "ab" * 32, 1000)
            self.assertEqual(len(found), 1, "a tool store.json does not have is not listed")
            entry = found[0]
            self.assertEqual(entry["listing_id"], "linux.nonos-perl")
            rel = entry["releases"][0]
            pin = rel["package_hash"]
            served = (out / f"{pin}.tar").read_bytes()
            self.assertEqual(blake3(served), pin)
            self.assertEqual(rel["package_url"], f"http://mirror.example.org:80/linux/{pin}.tar")
            self.assertEqual(rel["release_id"], "nonos-perl@1")
            self.assertNotIn(b".zk_trailer.bin", served)

    def test_a_tool_not_yet_built_is_not_listed(self):
        with tempfile.TemporaryDirectory() as d:
            root = Path(d)
            store = root / "store.json"
            store.write_text(json.dumps({"packages": {"perl": FILES}}))
            caps = root / "capsules.json"
            caps.write_text(json.dumps(CAPSULES))
            tools = root / "tools.json"
            tools.write_text(json.dumps([{"package": "perl", "name": "Perl", "text": "Perl."}]))
            self.assertEqual(packages.package_entries(tools, store, caps, root, "m:80", root / "o",
                                                      "ab" * 32, 1000), [])


class Committed(unittest.TestCase):
    def test_every_listed_tool_is_a_package_of_this_tree(self):
        here = Path(__file__).resolve().parents[2]
        listed = [t["package"] for t in json.loads((here / "userland/capsule_market/linux-tools.json").read_text())]
        store = json.loads((here / "tools/nix/store.json").read_text())["packages"]
        self.assertEqual(sorted(listed), sorted(store))


if __name__ == "__main__":
    unittest.main()
