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


"""The attack suite's checker over every boot's log."""

import os
import shlex
import subprocess
import sys

CHECK = "nonos-ci/attack_suite_check.py"


def judge(boots, out, dry):
    """The checker over every boot's log, writing the report beside them."""
    # Judged for the attacks of the boots that ran, and the report says which.
    cmd = [sys.executable, CHECK, *[a for b in boots for a in ("--log", b.log, "--boot", b.name)],
           "--report", os.path.join(out, "report.txt"), "--json", os.path.join(out, "report.json")]
    print(f"selftest: check: {shlex.join(cmd)}", flush=True)
    if dry:
        return 0
    missing = [b.log for b in boots if not os.path.exists(b.log)]
    for m in missing:
        # An empty log stands in, so the checker says what never ran.
        open(m, "w").close()
    return subprocess.run(cmd).returncode
