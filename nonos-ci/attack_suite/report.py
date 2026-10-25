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

"""The report a self-test run writes: one record a line, versioned, so a
reader knows which shape it reads. Text for people, JSON for tools; both
carry the same records."""

import json

from .dropped import DROPPED
from .rules import REPORT_VERSION

FORMAT = "nonos-attack-suite-report"


def records(result, logs):
    """`logs` is (path, sha256) for each log, the digest of the bytes judged."""
    yield {"record": "format", "name": FORMAT, "version": REPORT_VERSION}
    yield {"record": "scope", "boots": result.boots}
    for path, sha in logs:
        yield {"record": "log", "path": path, "sha256": sha}
    for name, (date, why) in sorted(DROPPED.items()):
        yield {"record": "dropped", "attack": name, "since": date, "reason": why}
    for a in result.attempts:
        yield {"record": "attempt", "attack": a.name, "verdict": a.verdict, "how": a.how,
               "kernel": a.kernel}
    for note in result.notes:
        yield {"record": "note", "text": note}
    for row, missing in result.rows:
        yield {"record": "row", "row": row, "proven": not missing}
    for b in result.bad:
        yield {"record": "fail", "reason": b}
    yield {"record": "result", "pass": not result.bad}


# Each record's text lines.
LINES = {
    "format": lambda r: [f"{r['name']} v{r['version']}"],
    "scope": lambda r: [f"boots {' '.join(r['boots'])}"],
    "log": lambda r: [f"log {r['path']} sha256 {r['sha256']}"],
    "dropped": lambda r: [f"dropped {r['attack']} since {r['since']}: {r['reason']}"],
    "attempt": lambda r: [f"attempt {r['attack']} {r['verdict']}: {r['how']}",
                          f"  kernel: {r['kernel'] or 'no line; the refusal is the errno the call returned'}"],
    "note": lambda r: [f"note {r['text']}"],
    "row": lambda r: [f"row {r['row']}: {'proven' if r['proven'] else 'NOT proven'}"],
    "fail": lambda r: [f"FAIL {r['reason']}"],
    "result": lambda r: [f"attack suite: {'PASS' if r['pass'] else 'FAIL'}"],
}


def text(result, logs):
    return "".join(f"{line}\n" for r in records(result, logs) for line in LINES[r["record"]](r))


def as_json(result, logs):
    return json.dumps(list(records(result, logs)), indent=1) + "\n"
