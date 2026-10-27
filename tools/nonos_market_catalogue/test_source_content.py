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
"""A source's content bundles to the same bytes from a tree or any tar of it,
and holds regular files only: a link (a hard one too), a path out of the tar,
and an empty, missing or doubled content are refused. Run:
python3 -m unittest discover -s tools/nonos_market_catalogue -t tools"""

import io
import tarfile
import tempfile
import unittest
from pathlib import Path

from .apk_fixture import ELF
from .source_content import bundled

FILES = {"usr/bin/drk": ELF + b"drk", "usr/share/drk/config.toml": b"x = 1\n"}


def tar_of(files: dict, links=(), mtime=0, kind=tarfile.SYMTYPE, target="/etc/passwd") -> bytes:
    buf = io.BytesIO()
    with tarfile.open(fileobj=buf, mode="w", format=tarfile.PAX_FORMAT) as t:
        for name, body in reversed(list(files.items())):
            info = tarfile.TarInfo(name)
            info.size, info.mtime, info.uname = len(body), mtime, "builder"
            t.addfile(info, io.BytesIO(body))
        for name in links:
            info = tarfile.TarInfo(name)
            info.type, info.linkname = kind, target
            t.addfile(info)
    return buf.getvalue()


class Content(unittest.TestCase):
    def test_a_tree_and_any_tar_of_it_give_the_same_bytes(self):
        with tempfile.TemporaryDirectory() as d:
            for path, body in FILES.items():
                (Path(d) / "a" / path).parent.mkdir(parents=True, exist_ok=True)
                (Path(d) / "a" / path).write_bytes(body)
            (Path(d) / "b.tar").write_bytes(tar_of(FILES, mtime=1700000000))
            tree, tar = bundled(Path(d), "a"), bundled(Path(d), "b")
            self.assertEqual(tree, tar)
            self.assertEqual([m.name for m in tarfile.open(fileobj=io.BytesIO(tree))], sorted(FILES))

    def test_links_escapes_and_missing_or_doubled_content_are_refused(self):
        with tempfile.TemporaryDirectory() as d:
            at = Path(d)
            (at / "l.tar").write_bytes(tar_of(FILES, links=("usr/bin/sh",)))
            (at / "hard.tar").write_bytes(tar_of(FILES, ("usr/bin/sh",), kind=tarfile.LNKTYPE, target="usr/bin/drk"))
            (at / "up.tar").write_bytes(tar_of({"../x": ELF}))
            (at / "torn.tar").write_bytes(tar_of(FILES)[:700])
            (at / "empty").mkdir()
            (at / "link" / "usr").mkdir(parents=True)
            (at / "link" / "usr" / "passwd").symlink_to("/etc/passwd")
            (at / "both").mkdir()
            (at / "both" / "x").write_bytes(ELF)
            (at / "both.tar").write_bytes(tar_of(FILES))
            for name in ("l", "hard", "up", "torn", "empty", "link", "both", "none"):
                with self.assertRaises(ValueError, msg=name):
                    bundled(at, name)
        with self.assertRaises(ValueError):
            bundled(None, "a")
