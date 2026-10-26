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
mark, and each is downloaded by the name its publisher gives it."""

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
            self.assertTrue(repo(published).startswith("Qwen/Qwen2.5-Coder-"))

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

    def test_a_safetensors_only_model_is_fetched_from_ggml_org(self):
        for name, at in (("Qwen3.5-0.8B-Q8_0.gguf", "ggml-org/Qwen3.5-0.8B-GGUF"),
                         ("Qwen3.6-27B-Q4_K_M.gguf", "ggml-org/Qwen3.6-27B-GGUF")):
            self.assertEqual(repo(name), at)
            self.assertEqual(url(name), f"https://huggingface.co/{at}/resolve/main/{name}")

    def test_the_vision_model_and_its_projector_are_the_qwen_teams(self):
        for name in ("Qwen3VL-2B-Instruct-Q4_K_M.gguf", "mmproj-Qwen3VL-2B-Instruct-Q8_0.gguf"):
            self.assertEqual(repo(name), "Qwen/Qwen3-VL-2B-Instruct-GGUF")
        for name in ("Qwen3VL-2B-Q4_K_M.gguf", "mmproj-Qwen3VL-2B-Instruct.gguf", "xmmproj-Qwen3VL-2B-Instruct-Q8_0.gguf"):
            with self.assertRaises(SystemExit, msg=name):
                repo(name)

    def test_the_qwen_teams_own_files_stay_theirs(self):
        self.assertEqual(repo("Qwen3-30B-A3B-Q4_K_M.gguf"), "Qwen/Qwen3-30B-A3B-GGUF")
        self.assertEqual(repo("qwen2.5-0.5b-instruct-q4_k_m.gguf"), "Qwen/Qwen2.5-0.5B-Instruct-GGUF")

    def test_a_file_no_known_repository_publishes_is_refused(self):
        for name in ("Qwen3.6-27B.gguf", "Qwen3.6-35B-A3B-Q4_K_M.gguf", "Qwen3.5-9B-Q4_K_M.gguf",
                     "Qwen3.5-0.9B-Q8_0.gguf",
                     "Qwen3.7-27B-Q4_K_M.gguf", "evil/Qwen3.6-27B-Q4_K_M.gguf"):
            with self.assertRaises(SystemExit, msg=name):
                repo(name)

    def test_every_pin_has_a_repository(self):
        for files in pins().values():
            for name, _, _ in files:
                self.assertIn(repo(upstream(name)).split("/")[0], ("Qwen", "ggml-org"), name)


if __name__ == "__main__":
    unittest.main()
