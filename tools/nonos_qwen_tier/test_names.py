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
"""Every pinned name is one the data volume keeps, with its record and its
mark, and each is downloaded by the name the Qwen team publishes it under."""

import unittest

from .download import repo, url
from .layout import mirrors
from .pins import ENTRY_BYTES, STREAM_NAME_MAX, keepable, pins
from .upstream import PUBLISHED, upstream

SPLIT = len("-00001-of-00002.gguf")


class Names(unittest.TestCase):
    def test_the_limits_are_the_kernels(self):
        self.assertEqual((ENTRY_BYTES, STREAM_NAME_MAX), (56, 49))

    def test_every_pin_its_record_and_its_mark_fit(self):
        for tier, files in pins().items():
            for name, _, _ in files:
                self.assertLessEqual(len("/" + name), 49, name)
                self.assertLessEqual(len(name + ".sha256"), 56, name)
                self.assertLessEqual(len(name + ".partial"), 56, name)

    def test_one_byte_past_the_limit_is_refused(self):
        self.assertTrue(keepable("a" * 48))
        self.assertFalse(keepable("a" * 49))
        self.assertFalse(keepable(""))

    def test_renamed_files_needed_it_and_stay_one_split_model(self):
        table = {n: t for t, files in pins().items() for n, _, _ in files}
        for kept, published in PUBLISHED.items():
            self.assertIn(kept, table)
            self.assertFalse(keepable(published), published)
            self.assertEqual(kept[-SPLIT:], published[-SPLIT:])
            self.assertEqual(published.replace("-instruct", ""), kept)
            self.assertTrue(repo(published).startswith("Qwen2.5-Coder-"))

    def test_downloads_ask_for_the_published_name(self):
        name = "qwen2.5-coder-14b-q4_k_m-00002-of-00002.gguf"
        long = "qwen2.5-coder-14b-instruct-q4_k_m-00002-of-00002.gguf"
        self.assertEqual(url(name), url(long))
        self.assertTrue(url(name).endswith("/Qwen2.5-Coder-14B-Instruct-GGUF/resolve/main/" + long))
        self.assertEqual(mirrors("https://r.test/q", "coder-14b", name),
                         [f"https://r.test/q/coder-14b/{long}", url(long)])

    def test_the_small_and_4b_tiers_keep_their_own_names(self):
        table = pins()
        for tier, name in (("small", "qwen2.5-0.5b-instruct-q4_k_m.gguf"),
                           ("qwen3-0.6b", "Qwen3-0.6B-Q8_0.gguf"),
                           ("qwen3-4b", "Qwen3-4B-Q4_K_M.gguf")):
            self.assertEqual([n for n, _, _ in table[tier]], [name])
            self.assertEqual(upstream(name), name)
            self.assertTrue(url(name).endswith("/resolve/main/" + name))


if __name__ == "__main__":
    unittest.main()
