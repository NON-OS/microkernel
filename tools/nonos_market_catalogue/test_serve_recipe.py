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
"""A source or asset recipe's `serve`: SERVE_ONE's bit, version 2 and its field at 217, as
nonos_linux_policy decodes them; never with WAYLAND or in an Alpine recipe; no other listing's
bytes move. Run: python3 -m unittest discover -s tools/nonos_market_catalogue -t tools"""

import json
import unittest
from pathlib import Path

from nonos_linux_pack.policy import market_ready

from .catalogue import guest_policy
from .recipes import UNMEASURED, checked as alpine
from .recipes_file import load
from .source_recipe import checked
from .test_source_recipe import SOURCE

ROOT = Path(__file__).resolve().parents[2]
DOCS, CORE_DOC = ROOT / "nonos-data/market/catalogue", "linux.kali.nonos-simplex-core.json"
FIELD = (5225).to_bytes(2, "little") + b"app.simplex".ljust(32, b"\0")
CORE = SOURCE | {"serve": "app.simplex:5225", "mask": "FILES_HOME NET_ANON"}


def policy(recipe: dict) -> bytes:
    return guest_policy({"dests": None} | recipe, b"", bytes(32), "linux.x", "x@1")[:256]


class ServeRecipe(unittest.TestCase):
    def test_serve_sets_the_bit_and_writes_version_2s_field(self):
        b = policy(checked(CORE, ROOT))
        self.assertEqual((b[8:10], b[48:56], b[217:251], b[251:]), (b"\2\0", bytes([10, 2]) + bytes(6), FIELD, bytes(5)))
        row = [r for r in load(ROOT / "userland/capsule_market/linux-catalogue.json", ROOT) if r.get("serve")]
        self.assertEqual([(r["package"], r["serve"], r["mask"]) for r in row], [("simplex-core", (5225, "app.simplex"), 0x20A)])

    def test_a_bad_serve_wayland_or_an_alpine_recipe_is_refused(self):
        for bad in ("app.simplex:0", "app.simplex:65536", "0app:1", "q" * 33 + ":1", "App:1", "app.simplex"):
            with self.assertRaises(ValueError, msg=bad):
                checked(CORE | {"serve": bad}, ROOT)
        with self.assertRaisesRegex(ValueError, "SERVE_ONE with WAYLAND"):
            checked(CORE | {"mask": "WAYLAND NET_ANON"}, ROOT)
        with self.assertRaisesRegex(ValueError, "set by serve"):
            checked(SOURCE | {"mask": "NET_ANON SERVE_ONE"}, ROOT)
        with self.assertRaisesRegex(ValueError, "keys"):
            alpine({k: CORE[k] for k in ("package", "licence", "mask", "name", "text", "serve")}, ROOT)

    def test_the_encoder_holds_the_bit_to_the_field_and_off_wayland(self):
        with self.assertRaisesRegex(ValueError, "SERVE_ONE needs"):
            policy({"mask": 0x20A, "calls": UNMEASURED})
        with self.assertRaisesRegex(ValueError, "SERVE_ONE with WAYLAND"):
            policy({"mask": 0x20B, "calls": UNMEASURED, "serve": (5225, "app.simplex")})
        with self.assertRaisesRegex(ValueError, "crate refuses"):
            policy({"mask": 0x20A, "calls": UNMEASURED, "serve": (0, "app.simplex")})
        self.assertEqual([market_ready(UNMEASURED, m) for m in (0x201, 0x200, 1)], [False, True, True])

    def test_only_the_cores_document_is_version_2(self):
        for doc in sorted(DOCS.glob("*.json")):
            for b in (bytes.fromhex(r["ext"]["guest_policy"]) for e in json.loads(doc.read_text())["entries"]
                      for r in e["releases"]):
                want = (b"\2\0", 2, True) if doc.name == CORE_DOC else (b"\1\0", 0, False)
                self.assertEqual((b[8:10], b[49] & 2, b[217:251] == FIELD), want, doc.name)
