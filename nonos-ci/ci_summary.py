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
"""One verdict for a workflow run: every job, its result, and why a job
that did not run was left out, written to the job summary.

Reads NEEDS (the `needs` context as JSON) and, optionally, SKIPPED (JSON
object: job -> the reason it is allowed to be skipped) and REPORT_ONLY (JSON
list of jobs whose result is shown but decides nothing, such as a matrix
with known-red cells). Fails when any other job failed or was cancelled, or
was skipped with no reason given.

    NEEDS='${{ toJSON(needs) }}' SKIPPED='{...}' nonos-ci/ci_summary.py <title>
"""

import json
import os
import sys

MARK = {"success": "passed", "failure": "FAILED", "cancelled": "CANCELLED", "skipped": "skipped"}


def main():
    title = sys.argv[1] if len(sys.argv) > 1 else "CI"
    needs = json.loads(os.environ.get("NEEDS") or "{}")
    allowed = json.loads(os.environ.get("SKIPPED") or "{}")
    report_only = set(json.loads(os.environ.get("REPORT_ONLY") or "[]"))
    server, repo, run = (os.environ.get(k, "") for k in ("GITHUB_SERVER_URL", "GITHUB_REPOSITORY", "GITHUB_RUN_ID"))
    rows, bad = [], []
    for job in sorted(needs):
        result = needs[job].get("result", "unknown")
        note = ""
        if result == "skipped":
            note = allowed.get(job, "")
            if not note:
                bad.append(job)
                note = "skipped with no reason"
        elif result != "success" and job in report_only:
            note = "reported, does not gate"
        elif result != "success":
            bad.append(job)
        rows.append(f"| `{job}` | {MARK.get(result, result)} | {note} |")
    counts = {}
    for job in needs:
        r = needs[job].get("result", "unknown")
        counts[r] = counts.get(r, 0) + 1
    tally = ", ".join(f"{n} {MARK.get(r, r)}" for r, n in sorted(counts.items()))
    text = "\n".join([
        f"## {title}: {'FAILED' if bad else 'passed'}", "",
        f"{len(needs)} jobs: {tally}. {server}/{repo}/actions/runs/{run}", "",
        "| job | result | note |", "|---|---|---|", *rows, ""])
    print(text)
    if os.environ.get("GITHUB_STEP_SUMMARY"):
        with open(os.environ["GITHUB_STEP_SUMMARY"], "a", encoding="utf-8") as out:
            out.write(text + "\n")
    for job in bad:
        print(f"::error::{job}: {needs[job].get('result')}", file=sys.stderr)
    return 1 if bad else 0


if __name__ == "__main__":
    sys.exit(main())
