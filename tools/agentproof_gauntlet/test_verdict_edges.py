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
"""The verdict's edges: one box report a run, never two, never one beside a
missing one; and the refusal limit taken exactly."""

import unittest

from .test_verdict import BOX, DONE, IOMMU, log
from .verdict import judge, read


class Edges(unittest.TestCase):
    def test_two_reports_fail(self):
        clean = BOX.format(r=0, w=0, n=0)
        self.assertTrue(judge(read(log(IOMMU, clean, clean, DONE))))
        self.assertTrue(judge(read(log(IOMMU, BOX.format(r=2, w=2, n=0), clean, DONE))))

    def test_a_report_beside_a_missing_one_fails(self):
        missing = "[BOX-MISSING] v=1 report=incomplete capsule_failed=no exit=0 source=terminal"
        self.assertTrue(judge(read(log(IOMMU, BOX.format(r=0, w=0, n=0), missing, DONE))))

    def test_the_limit_is_exact(self):
        r = read(log(IOMMU, BOX.format(r=3, w=1, n=2), DONE))
        self.assertEqual(judge(r, max_refused=3), [])
        self.assertEqual(len(judge(r, max_refused=2)), 1)

    def test_a_learn_run_never_passes(self):
        learn = BOX.format(r=0, w=0, n=0).replace("preset=strict", "preset=learn enforced=no")
        self.assertIn("preset=learn", learn)
        self.assertEqual(len(judge(read(log(IOMMU, learn, DONE)))), 1)
        alone = BOX.format(r=0, w=0, n=0).replace("preset=strict", "preset=strict enforced=no")
        self.assertEqual(len(judge(read(log(IOMMU, alone, DONE)))), 1)

    def test_the_net_line_is_kept_for_the_result(self):
        net = "[BOX-NET] v=1 allowed=10.0.2.2:80 connected=none"
        r = read(log(IOMMU, net, BOX.format(r=0, w=0, n=0), DONE))
        self.assertEqual(r["net"], {"v": "1", "allowed": "10.0.2.2:80", "connected": "none"})
        self.assertEqual(judge(r), [])
