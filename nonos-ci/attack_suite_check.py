#!/usr/bin/env python3
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

"""Judge a self-test run's serial logs: every attack attempted and refused,
none ESCAPED, every isolation matrix row with a refusal of its own.

  attack_suite_check.py --log capsule.log --log attest.log --log linux.log
  attack_suite_check.py --log ... --report report.txt --json report.json
  attack_suite_check.py --log capsule.log --boot capsule   one boot's attacks only
  attack_suite_check.py --log ... --mutants   every mutant of these logs fails
  attack_suite_check.py --list                the attacks, rows and dropped rows
  attack_suite_check.py --self-test           the unit tests and mutants, no boot

Exit 0 when the suite passes, 1 when it fails, 2 when a file cannot be read
or written or the arguments are wrong. The report's first line names its
format and version (attack_suite/report.py)."""

import os
import sys

sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))

from attack_suite import report  # noqa: E402
from attack_suite.files import read, write  # noqa: E402
from attack_suite.judge import judge  # noqa: E402
from attack_suite.options import parser  # noqa: E402
from attack_suite.selfcheck import list_rules, mutant_run, self_test  # noqa: E402


def main(argv=None):
    ap = parser(__doc__)
    a = ap.parse_args(argv)
    if a.self_test:
        return self_test()
    if a.list:
        return list_rules()
    if not a.log:
        ap.error("--log is required")
    logs = read(a.log)
    texts, named = [t for _, t, _ in logs], [(p, sha) for p, _, sha in logs]
    if a.mutants:
        return mutant_run(texts, f"{len(logs)} log(s)", a.boot)
    result = judge(texts, a.boot)
    body = report.text(result, named)
    sys.stdout.write(body)
    if a.report:
        write(a.report, body)
    if a.json:
        write(a.json, report.as_json(result, named))
    return 1 if result.bad else 0


if __name__ == "__main__":
    sys.exit(main())
