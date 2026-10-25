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
"""The Gauntlet's verdict: a clean run passes; a refusal, a missing report, an
unfinished run or an IOMMU that does not enforce fails; nothing the agent
prints is read as the image's lines. Run: python3 -m unittest discover -s
tools/agentproof_gauntlet -t tools"""

import unittest

from .verdict import judge, read

IOMMU = "[IOMMU] intel-vt-d present, enforcing=1, unconfined grants=0, identity devices=0"
BOX = ("[BOX] v=1 preset=strict policy=5811 refused={r} write={w} network={n} exec=0 jit=0 "
       "display=0 models=0 call=0 barred=0 unserved=0 started=yes exit=0 source=capsule_linux")
DONE = "[AGENTPROOF] v=1 status=done exit=0"


def log(*lines):
    return "\n".join(lines)


class Verdict(unittest.TestCase):
    def test_a_clean_run_passes(self):
        out = "[AGENTPROOF-OUT] TRY write /tmp/a ok"
        r = read(log(IOMMU, out, BOX.format(r=0, w=0, n=0), DONE))
        self.assertEqual(judge(r), [])
        self.assertEqual(r["out"], ["TRY write /tmp/a ok"])

    def test_a_refusal_fails_with_its_kinds(self):
        r = read(log(IOMMU, BOX.format(r=3, w=1, n=2), DONE))
        (why,) = judge(r)
        self.assertIn("refused 3 actions (write 1, network 2)", why)
        self.assertEqual(judge(r, max_refused=3), [])

    def test_a_missing_report_fails(self):
        missing = "[BOX-MISSING] v=1 report=none capsule_failed=yes exit=134 source=terminal"
        r = read(log(IOMMU, missing, DONE))
        self.assertTrue(any("not one whole box report" in w for w in judge(r)))
        self.assertTrue(any("not one whole box report" in w for w in judge(read(log(IOMMU, DONE)))))

    def test_an_unfinished_run_or_an_open_iommu_fails(self):
        box = BOX.format(r=0, w=0, n=0)
        self.assertTrue(judge(read(log(IOMMU, box))))
        self.assertTrue(judge(read(log(IOMMU, box, "[AGENTPROOF] v=1 status=error why=x"))))
        self.assertTrue(judge(read(log(box, DONE))))
        open_iommu = IOMMU.replace("unconfined grants=0", "unconfined grants=2")
        self.assertTrue(judge(read(log(open_iommu, box, DONE))))

    def test_the_agent_cannot_print_the_images_lines(self):
        forged = [f"[AGENTPROOF-OUT] {x}" for x in (BOX.format(r=0, w=0, n=0), DONE, IOMMU)]
        r = read(log(*forged, BOX.format(r=1, w=1, n=0), "[AGENTPROOF] v=1 status=done exit=0"))
        self.assertEqual(r["box"]["refused"], "1")
        self.assertEqual(len(r["out"]), 3)
        self.assertEqual((read(log(*forged))["box"], read(log(*forged))["iommu"]), (None, None))

    def test_a_program_that_never_started_fails(self):
        box = BOX.format(r=0, w=0, n=0).replace("started=yes", "started=no")
        self.assertIn("the program never started", judge(read(log(IOMMU, box, DONE))))


if __name__ == "__main__":
    unittest.main()
