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


"""The report's shape: a versioned first line, one record a line, and the
same records in the text and the JSON."""

import json
import unittest

from . import report
from .judge import judge
from .rules import REPORT_VERSION
from .sample import passing
from .test_judge import with_capsule

LOGS = [("attest.log", "a" * 64), ("capsule.log", "b" * 64), ("linux.log", "c" * 64)]


class Report(unittest.TestCase):
    def setUp(self):
        self.result = judge(passing())

    def test_the_first_lines_name_the_format_version_and_scope(self):
        first, scope = report.text(self.result, LOGS).splitlines()[:2]
        self.assertEqual(first, f"nonos-attack-suite-report v{REPORT_VERSION}")
        self.assertEqual(scope, "boots attest capsule linux")

    def test_json_and_text_carry_the_same_records(self):
        records = json.loads(report.as_json(self.result, LOGS))
        self.assertEqual(records[0], {"record": "format", "name": report.FORMAT, "version": REPORT_VERSION})
        self.assertEqual(records[-1], {"record": "result", "pass": True})
        kinds = [r["record"] for r in records]
        self.assertEqual(kinds.count("attempt"), len(self.result.attempts))
        self.assertEqual([r["sha256"] for r in records if r["record"] == "log"], [s for _, s in LOGS])
        lines = report.text(self.result, LOGS).splitlines()
        self.assertEqual(lines[-1], "attack suite: PASS")
        # Each attempt is two text lines, every other record one.
        self.assertEqual(len(lines), len(records) + kinds.count("attempt"))

    def test_no_sample_or_reference_claim_in_shipped_output(self):
        body = report.text(self.result, LOGS).lower()
        for word in ("sample", "synthetic", "fixture", "reference"):
            self.assertNotIn(word, body)

    def test_a_failing_run_says_why_and_fails(self):
        r = judge(with_capsule(lambda t: t + "[ATTACK] cap-escape ESCAPED: x\n"))
        lines = report.text(r, LOGS).splitlines()
        self.assertIn("FAIL cap-escape: ESCAPED: x", lines)
        self.assertEqual(lines[-1], "attack suite: FAIL")


if __name__ == "__main__":
    unittest.main()
