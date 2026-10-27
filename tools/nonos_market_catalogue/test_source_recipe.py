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
"""A source recipe names its upstream by https URL and full commit, a nix
build, and its program under usr/bin; it is checked as an Alpine recipe
besides, and keeps out of the namespaces the generator adds. Run:
python3 -m unittest discover -s tools/nonos_market_catalogue -t tools"""

import json
import tempfile
import unittest
from pathlib import Path

from .recipes_file import FORMAT, load
from .source_recipe import checked

ROOT = Path(__file__).resolve().parents[2]
SOURCE = {"kind": "source", "package": "darkfi-drk", "licence": "AGPL-3.0-only", "mask": "TTY FILES_HOME",
          "name": "drk", "text": "x", "program": "usr/bin/drk", "build": "darkfi-drk",
          "source": "https://github.com/darkrenaissance/darkfi", "commit": "1cd71ea" + "0" * 33}


class SourceRecipe(unittest.TestCase):
    def test_a_good_recipe_keeps_its_upstream_and_its_box(self):
        got = checked(SOURCE, ROOT)
        self.assertEqual((got["kind"], got["commit"], got["build"]), ("source", SOURCE["commit"], "darkfi-drk"))
        self.assertEqual((got["mask"], got["program"], got["dests"]), (0x108, "usr/bin/drk", None))

    def test_the_upstream_is_an_https_url_at_a_full_commit(self):
        for bad in ("http://github.com/x/y", "https://github.com", "https://user@github.com/x",
                    "https://github.com:443/x", "https://github.com/x?y", "https://github.com/../x",
                    "https://GitHub.com/x", "git://github.com/x", "https://github.com/x/.git"):
            with self.assertRaises(ValueError, msg=bad):
                checked(dict(SOURCE, source=bad), ROOT)
        for key, bad in [("commit", b) for b in ("1cd71ea", "1CD71EA" + "0" * 33, "g" * 40, "0" * 41, "0" * 64)] \
                + [("build", b) for b in ("", "Darkfi", "../x", "x/y")]:
            with self.assertRaises(ValueError, msg=bad):
                checked(dict(SOURCE, **{key: bad}), ROOT)

    def test_its_keys_are_its_own_and_its_program_is_required(self):
        less = [{k: v for k, v in SOURCE.items() if k != gone} for gone in ("program", "commit", "build")]
        for bad in less + [dict(SOURCE, index_licence="MIT", licence_source="https://x.org"),
                           dict(SOURCE, url="x"), dict(SOURCE, program="usr/lib/drk"),
                           dict(SOURCE, licence="Proprietary"), dict(SOURCE, mask="TTY FILES_USER"),
                           dict(SOURCE, commit=1)]:
            with self.assertRaises(ValueError, msg=bad):
                checked(bad, ROOT)

    def test_a_reserved_namespace_or_a_name_the_device_refuses_is_refused(self):
        for bad in ("nonos-drk", "qwen-x", "kali.x", "blackarch.x", "drk_cli", "drk+", "-drk", "a" * 59):
            with self.assertRaises(ValueError, msg=bad):
                checked(dict(SOURCE, package=bad), ROOT)

    def test_the_file_reads_both_kinds_and_a_package_once(self):
        alpine = {"package": "darkfi-drk", "licence": "MIT", "mask": "TTY", "name": "X", "text": "x"}
        with tempfile.TemporaryDirectory() as d:
            path = Path(d) / "c.json"
            path.write_text(json.dumps({"format": FORMAT, "apps": [SOURCE, dict(alpine, package="y")]}))
            self.assertEqual([r.get("kind") for r in load(path, ROOT)], ["source", None])
            path.write_text(json.dumps({"format": FORMAT, "apps": [SOURCE, alpine]}))
            with self.assertRaises(ValueError):
                load(path, ROOT)
