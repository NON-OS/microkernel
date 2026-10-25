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
"""One debug run's log, as the boot session keeps it (HANDOFF/catalogue/05):
whether it shows the app installed in its release's box, its workload
ending well, and exactly one measured set, the app's own family's."""

import re

from nonos_linux_pack.syscalls import buckets

STARTED, REFUSED = b"[LINUX-INSTALL] started, in its release's box linux.", (
    b"[LINUX] refused the market's policy", b"the market does not vouch for its box")
EXITED = re.compile(rb"\[LINUX\] process \d+ exited, status (\d+)")
SIGNALLED = (b"ended by signal", b"ended on a signal")


def run_verdict(app: str, log: bytes):
    """(ok, sets, why) for one run's log."""
    if STARTED + app.encode() not in log:
        return False, [], "the install never started in its release's box"
    if any(r in log for r in REFUSED):
        return False, [], "the install role refused the policy"
    codes = [int(c) for c in EXITED.findall(log)]
    if any(c != 0 for c in codes) or any(s in log for s in SIGNALLED):
        return False, [], "the workload failed: a process exited nonzero or on a signal"
    if not codes:
        return False, [], "the workload never ended: no process exited"
    try:
        sets = buckets(log)
    except ValueError as why:
        return False, [], f"its measured set is malformed: {why}"
    if len(sets) != 1:
        return False, [], f"{len(sets)} measured sets in one run: prove the app's family alone"
    return True, sets, "installed in its box, workload ran, set measured"
