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

"""Every source recipe's content made present and held to its pin: the
order it is looked for in, a tar off its pin never taken from anywhere,
the cache filled and reused, the content branch read through git alone,
and a package nothing gives named with how it is built, or, for a
development build alone, said in one line and left absent."""

import io
import json
import subprocess
import tarfile
import tempfile
import unittest
from pathlib import Path
from unittest import mock

from nonos_market_catalogue import source_content
from nonos_market_catalogue.digest import blake3

from . import asset, cache, ensure, pins, store


def content(body: bytes) -> bytes:
    buf = io.BytesIO()
    with tarfile.open(fileobj=buf, mode="w") as t:
        info = tarfile.TarInfo("usr/bin/demo")
        info.size = len(body)
        t.addfile(info, io.BytesIO(body))
    return buf.getvalue()


def pin_of(tar: bytes, package: str) -> tuple:
    with tempfile.TemporaryDirectory() as d:
        (Path(d) / f"{package}.tar").write_bytes(tar)
        b = source_content.bundled(Path(d), package)
    return blake3(b), len(b)


def git(repo: Path, *args):
    subprocess.run(["git", "-C", str(repo), "-c", "user.name=t", "-c", "user.email=t@t", *args],
                   check=True, capture_output=True)


class World:
    """A source recipe, its pin, an empty cache and a repo whose origin holds
    a content branch."""

    def __init__(self, d: Path, branch_body: bytes = None, recipe: dict = None):
        self.good = content(b"\x7fELF the pinned program")
        self.package = "demo"
        self.pin = pin_of(self.good, self.package)
        self.recipe = recipe or {"kind": "source", "package": self.package}
        self.out, self.cache = d / "out", d / "cache"
        origin, self.repo = d / "origin", d / "repo"
        git(d, "init", "-q", str(origin))
        (origin / store.DIR).mkdir()
        (origin / store.DIR / f"{self.package}.tar").write_bytes(branch_body or self.good)
        git(origin, "checkout", "-q", "-b", store.BRANCH)
        git(origin, "add", "-A")
        git(origin, "commit", "-q", "-m", "content")
        git(d, "init", "-q", str(self.repo))
        git(self.repo, "remote", "add", "origin", str(origin))

    def run(self, absent_ok=False):
        with mock.patch.object(ensure, "recipes", lambda: [self.recipe]), \
                mock.patch.object(ensure, "pins", lambda: {self.package: self.pin}):
            return ensure.ensure(self.out, self.cache, self.repo, absent_ok)


class Sources(unittest.TestCase):
    def setUp(self):
        self.dir = tempfile.TemporaryDirectory()
        self.d = Path(self.dir.name)

    def tearDown(self):
        self.dir.cleanup()

    def test_the_branch_gives_it_the_cache_keeps_it_and_the_next_run_needs_neither(self):
        w = World(self.d)
        self.assertEqual(w.run(), [("demo", store.BRANCH)])
        self.assertTrue(pins.holds(w.out / "demo.tar", "demo", w.pin))
        self.assertTrue((w.cache / f"{w.pin[0]}.tar").is_file())
        self.assertEqual(w.run(), [("demo", "present")])
        (w.out / "demo.tar").unlink()
        git(w.repo, "remote", "remove", "origin")
        self.assertEqual(w.run(), [("demo", "the cache")], "no network, no branch: the cache")

    def test_a_tar_off_its_pin_is_never_taken(self):
        w = World(self.d, branch_body=content(b"\x7fELF another program"))
        (w.out).mkdir()
        (w.out / "demo.tar").write_bytes(content(b"\x7fELF a stale build"))
        with self.assertRaises(ValueError) as e:
            w.run()
        self.assertIn("darkfi-build.sh", str(e.exception), "it names how the content is built")
        self.assertFalse((w.out / "demo.tar").exists(), "nothing off its pin is left behind")
        self.assertFalse(w.cache.exists() and any(w.cache.iterdir()))

    def test_a_cached_file_off_its_pin_is_dropped(self):
        w = World(self.d)
        w.cache.mkdir()
        (w.cache / f"{w.pin[0]}.tar").write_bytes(content(b"\x7fELF tampered"))
        self.assertEqual(w.run(), [("demo", store.BRANCH)])
        self.assertTrue(pins.holds(w.cache / f"{w.pin[0]}.tar", "demo", w.pin))

    def test_an_asset_recipe_falls_to_upstream_and_is_held_to_its_pin_there_too(self):
        w = World(self.d, branch_body=content(b"\x7fELF not it"), recipe={"kind": "source", "package": "demo",
                                                                        "asset": "https://example/demo"})
        good = w.good

        def made(recipe, out_dir):
            (out_dir / "demo.tar").write_bytes(good)
            return True

        with mock.patch.object(asset, "make", made):
            self.assertEqual(w.run(), [("demo", "upstream's release")])

        def wrong(recipe, out_dir):
            (out_dir / "demo.tar").write_bytes(content(b"\x7fELF wrong"))
            return True

        (w.out / "demo.tar").unlink()
        for f in w.cache.iterdir():
            f.unlink()
        with mock.patch.object(asset, "make", wrong), self.assertRaises(ValueError):
            w.run()

    def test_an_asset_recipe_with_a_tool_not_here_is_said_by_the_tool_and_never_made(self):
        w = World(self.d, branch_body=content(b"\x7fELF not it"), recipe={"kind": "source", "package": "demo",
                                                                        "asset": "https://example/demo"})
        made = mock.Mock(return_value=True)
        here = {"gpg": "/bin/gpg", "gpgv": "/bin/gpgv"}
        with mock.patch.object(asset, "make", made), mock.patch.object(asset.shutil, "which", here.get):
            with self.assertRaisesRegex(ValueError, "demo: b3sum not found here"):
                w.run()
            with mock.patch("sys.stderr", new_callable=io.StringIO) as said:
                self.assertEqual(w.run(absent_ok=True), [])
        lines = [x for x in said.getvalue().splitlines() if "left out" in x]
        self.assertEqual(len(lines), 1)
        self.assertIn("demo: b3sum not found here", lines[0])
        self.assertNotIn("did not give", lines[0], "the release is not said to be wrong")
        made.assert_not_called()
        self.assertEqual(asset.missing(), [t for t in asset.TOOLS if not asset.shutil.which(t)])

    def test_a_tree_in_place_of_the_tar_is_refused_by_name(self):
        w = World(self.d)
        (w.out / "demo").mkdir(parents=True)
        for absent_ok in (False, True):
            with self.assertRaises(ValueError) as e:
                w.run(absent_ok)
            self.assertIn("is a tree", str(e.exception))

    def test_content_nothing_gives_is_left_absent_only_when_a_development_build_asks(self):
        w = World(self.d, branch_body=content(b"\x7fELF another program"))
        with self.assertRaisesRegex(ValueError, "demo: no cache, no "):
            w.run()
        with mock.patch("sys.stderr", new_callable=io.StringIO) as said:
            self.assertEqual(w.run(absent_ok=True), [])
        lines = [x for x in said.getvalue().splitlines() if "left out" in x]
        self.assertEqual(len(lines), 1)
        self.assertIn("demo: no cache", lines[0])
        self.assertFalse((w.out / "demo.tar").exists() or (w.out / "demo").exists(),
                         "nothing is left for the catalogue to list")

    def test_a_recipe_with_no_pin_still_stops_a_development_build(self):
        w = World(self.d)
        with mock.patch.object(ensure, "recipes", lambda: [w.recipe]), mock.patch.object(ensure, "pins", dict), \
                self.assertRaisesRegex(ValueError, "pins no content"):
            ensure.ensure(w.out, w.cache, w.repo, True)

    def test_every_package_the_tree_lists_has_a_pin(self):
        listed = {r["package"] for r in pins.recipes()}
        self.assertTrue(listed)
        self.assertEqual(listed, set(pins.pins()))
        self.assertEqual(json.loads(pins.CATALOGUE.read_text())["format"], "nonos.linux-catalogue.v1")
