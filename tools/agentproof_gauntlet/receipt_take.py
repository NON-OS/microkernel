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
"""A receipt's rows in a serial log, as receipt_log.rs writes them:

    [AGENTPROOF-RECEIPT] v=1 bytes=<n> seal=<16 hex>
    [AGENTPROOF-RECEIPT] <the receipt's next bytes, as hex>
    [AGENTPROOF-RECEIPT] v=1 end
    [AGENTPROOF-RECEIPT] v=1 rule again=same other=-16 after-ack=-2
    [AGENTPROOF-RECEIPT] v=1 status=kept seal=<16 hex> | status=none why=<why>

Taken only whole: every row after the head and before the end, exactly as
many bytes as the head says. A later head starts again.
"""

import re

HEAD = re.compile(r"\[AGENTPROOF-RECEIPT\] v=1 bytes=(\d+) seal=([0-9a-f]{16})")
ROW = re.compile(r"\[AGENTPROOF-RECEIPT\] ((?:[0-9a-f]{2})+)")
END, SAID = "[AGENTPROOF-RECEIPT] v=1 end", "[AGENTPROOF-RECEIPT] v=1 status="
RULE = "[AGENTPROOF-RECEIPT] v=1 rule "


def take(log: str):
    """The receipt the log carries, whole or None, what the Terminal said of it,
    and what the job's rule did on the machine."""
    rows, length, done, said, rule = None, -1, False, "no [AGENTPROOF-RECEIPT] status line", {}
    for line in log.splitlines():
        if m := HEAD.fullmatch(line):
            rows, length, done = [], int(m.group(1)), False
        elif line == END:
            done = rows is not None
        elif line.startswith(RULE):
            rule = dict(t.split("=", 1) for t in line[len(RULE):].split() if "=" in t)
        elif line.startswith(SAID):
            said = line[len(SAID):]
        elif rows is not None and not done and (m := ROW.fullmatch(line)):
            rows.append(bytes.fromhex(m.group(1)))
    body = b"".join(rows or [])
    return (body if done and len(body) == length else None), said, rule
