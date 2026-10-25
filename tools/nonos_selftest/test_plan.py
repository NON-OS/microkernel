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


"""The plan, and what is refused before anything runs; nothing boots."""

import os
import sys
import tempfile
import unittest

HERE = os.path.dirname(os.path.abspath(__file__))
sys.path.insert(0, os.path.dirname(HERE))

from nonos_selftest import boots, build, guard  # noqa: E402


class Plan(unittest.TestCase):
    def setUp(self):
        d = tempfile.TemporaryDirectory()
        self.addCleanup(d.cleanup)
        self.root = d.name
        self.tree = os.path.join(self.root, boots.TREE)

    def test_every_boot_writes_its_own_log_under_out(self):
        p = boots.plan(None, self.root, "target/selftest")
        self.assertEqual([b.name for b in p], list(boots.NAMES))
        self.assertEqual({os.path.dirname(b.log) for b in p}, {os.path.join(self.root, "target/selftest")})
        self.assertTrue(all(b.log in " ".join(b.cmd) for b in p), "each command is told its log")

    def test_every_make_boot_runs_in_the_copy_with_its_key(self):
        for b in boots.plan(None, self.root, "o"):
            if b.cmd[0] == "make":
                self.assertEqual(b.cwd, self.tree)
                self.assertEqual(b.env["NONOS_SIGNING_KEY"], os.path.join(self.tree, ".keys/signing_key_v1.bin"))
        self.assertIsNone(guard.refusal(boots.plan(None, self.root, "o"), self.root, built=True))

    def test_a_boot_that_would_use_the_checkouts_keys_is_refused(self):
        (a,) = boots.plan(["attest"], self.root, "o")
        here = a._replace(cwd=self.root)
        self.assertIn("would run make", guard.refusal([here], self.root, True))
        key = a._replace(env={"NONOS_SIGNING_KEY": os.path.join(self.root, ".keys/signing_key_v1.bin")})
        self.assertIn("outside the copy", guard.refusal([key], self.root, True))

    def test_no_build_needs_the_copys_keys_and_image(self):
        p = boots.plan(None, self.root, "o")
        self.assertIn("no throwaway keys", guard.refusal(p, self.root, built=False))
        os.makedirs(os.path.join(self.tree, ".keys"))
        open(os.path.join(self.tree, ".keys/signing_key_v1.bin"), "w").close()
        self.assertIn("no selftest image", guard.refusal(p, self.root, built=False))
        self.assertIsNone(guard.refusal(p[1:], self.root, built=False))

    def test_the_build_and_the_options(self):
        self.assertIn("selftest", build.prepare(["capsule", "attest"]))
        self.assertIn("--keys-only", build.prepare(["attest", "linux"]))
        (c,) = boots.plan(["capsule"], self.root, "o", iommu=True)
        self.assertEqual(c.cmd[-2:], ["--lane", "iommu=intel"])
        self.assertIn("--smp", c.cmd)


if __name__ == "__main__":
    unittest.main()
