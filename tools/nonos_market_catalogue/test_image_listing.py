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

"""An app image's listing: the release pins the measurement and the chunk
list's URL, and the policy names the box's program in the recipe's box,
with the same set and services as the app's listing from Alpine."""

import json
import unittest

from .image_listing import ROOT, listing, policy, recipe_for
from .recipes import checked

KEY = "bbeaa6c15c496575134e0c72d8d75d7d6128c9fdf3723761f44026a0adf522f2"
IMAGES = ROOT / "nonos-data/market/images"
RECIPES = [ROOT / "userland/capsule_market/linux-catalogue.json", ROOT / "userland/capsule_market/linux-images.json"]


def image(name):
    return json.loads((IMAGES / f"linux.nonos-image-{name}.json").read_text())


class ImageListing(unittest.TestCase):
    def test_telegrams_image_listing_names_the_program_its_alpine_listing_does(self):
        img = image("telegram-desktop")
        e = listing(recipe_for("telegram-desktop", RECIPES), img, KEY, 1)
        rel = e["releases"][0]
        self.assertEqual(e["listing_id"], "linux.nonos-image-telegram-desktop")
        self.assertEqual(rel["package_hash"], img["measurement"])
        self.assertTrue(rel["package_url"].endswith(img["measurement"] + ".chunks"))
        alpine = json.loads((ROOT / "nonos-data/market/catalogue/linux.telegram-desktop.json").read_text())
        theirs = alpine["entries"][0]["releases"][0]["ext"]["guest_policy"]
        ours = rel["ext"]["guest_policy"]
        program = img["box"]["program_blake3"]
        self.assertIn(program, theirs)
        self.assertIn(program, ours)
        self.assertEqual(len(bytes.fromhex(ours)), len(bytes.fromhex(theirs)))

    def test_libreoffices_listing_binds_soffice_bin(self):
        img = image("libreoffice")
        e = listing(recipe_for("libreoffice", RECIPES), img, KEY, 1)
        self.assertEqual(img["box"]["program"], "/usr/lib/libreoffice/program/soffice.bin")
        self.assertIn(img["box"]["program_blake3"], e["releases"][0]["ext"]["guest_policy"])

    def test_a_box_off_the_recipe_or_a_recipe_with_extras_is_refused(self):
        img = image("libreoffice")
        item = dict(recipe_for("libreoffice", RECIPES), mask="WAYLAND FILES_HOME")
        with self.assertRaises(ValueError):
            listing(item, img, KEY, 1)
        with self.assertRaises(ValueError):
            listing(dict(recipe_for("libreoffice", RECIPES), **{"with": ["font-noto"]}), img, KEY, 1)
        no_box = {k: v for k, v in img.items() if k != "box"}
        with self.assertRaises(ValueError):
            listing(recipe_for("libreoffice", RECIPES), no_box, KEY, 1)
        with self.assertRaises(ValueError):
            recipe_for("not-an-app", RECIPES)

    def test_the_policy_is_the_same_bytes_for_the_same_inputs(self):
        img = image("telegram-desktop")
        recipe = checked(recipe_for("telegram-desktop", RECIPES), ROOT)
        self.assertEqual(policy(recipe, img, bytes.fromhex(KEY)), policy(recipe, img, bytes.fromhex(KEY)))
