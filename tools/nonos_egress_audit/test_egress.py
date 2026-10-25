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

"""The audit's own proofs: it passes a tree its list matches, and each way a
new path can land, or the list go stale, is caught. Run with
`python3 -m unittest nonos_egress_audit.test_egress` from tools/."""

import unittest

from .__main__ import audit
from .case import REPO, Case, run


class Egress(Case):
    def test_a_tree_its_list_matches_passes(self):
        self.assertEqual(run(self.root), [])

    def test_an_address_in_a_library_a_networked_capsule_builds_from_is_caught(self):
        self.edit("userland/netlib/src/lib.rs", "113, 7", "113, 8")
        self.caught("203.0.113.8", "not listed")

    def test_a_new_host_is_caught(self):
        self.edit("userland/capsule_dial/src/main.rs", "dir.example.net", "dir2.example.net")
        self.caught("dir2.example.net", "not listed")

    def test_a_new_build_time_endpoint_is_caught(self):
        self.edit("userland/capsule_dial/src/main.rs", "NONOS_X_MIRROR", "NONOS_Y_URL")
        self.caught("build:NONOS_Y_URL")

    def test_a_capsule_that_gains_network_is_caught(self):
        self.edit("userland/capsule_quiet/Capsule.mk", "0x18", "0x1C")
        self.caught("capsule_quiet", "holds Network")
        self.caught("quiet.example.com", "not listed")

    def test_a_row_the_code_no_longer_has_is_caught(self):
        self.edit("docs/security/privacy.md", "`203.0.113.7`, ", "`203.0.113.7`, `198.51.100.1`, ")
        self.caught("198.51.100.1", "no networked source")

    def test_comments_tests_and_quiet_capsules_are_not_read(self):
        found = audit(self.root, "")[1]
        for lit in ("docs.example.org", "test.example.io", "10.20.30.40", "quiet.example.com"):
            self.assertNotIn(lit, found)

    def test_other_spellings_of_a_place_are_caught(self):
        main = self.root / "userland/capsule_dial/src/main.rs"
        main.write_text(main.read_text() + 'const A: Ipv4Addr = Ipv4Addr::new(198, 51, 100, 9);\n'
                        'const B: &str = "198.51.100.10:8080";\n'
                        "/* const C: &str = \"https://hidden.example.org\";\n*/\n")
        (main.parent / "fetch.py").write_text("URL = 'https://Upper.Example.NET/x'\n")
        found = audit(self.root, "")[1]
        self.assertIn("198.51.100.9", found)
        self.assertIn("198.51.100.10", found, "an address written as a dotted string, port and all")
        self.assertIn("upper.example.net", found)
        self.assertNotIn("hidden.example.org", found)

    def test_this_repository_agrees_with_its_list(self):
        self.assertEqual(run(REPO), [])


if __name__ == "__main__":
    unittest.main()
