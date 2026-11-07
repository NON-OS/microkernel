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
"""The kernel's boot census must be green (track 8).

The kernel says it once init's spawns are done, at the first look with
nothing missing or 30 s later (src/hardware/boot_census/run.rs), so the
boot waits for its summary line as part of readiness (verdict.READY). A
VM has no battery, lid or AC adapter; the census says them absent, which is
green. Anything the machine has that did not come up is missing, with its
reason, and fails the cell.
"""

import re

SUMMARY = re.compile(r"\[CENSUS\] summary up=\d+ absent=\d+ held=\d+ missing=(\d+) verdict=(green|red)")
MISSING = re.compile(r"\[CENSUS\] ([a-z]+) missing expected=\d+ up=\d+ why=(.*)")


def verdict(text):
    """Why the last census said is not green, if it is not."""
    said = list(SUMMARY.finditer(text))
    if not said:
        return ["no [CENSUS] summary line"]
    last = said[-1]
    if last.group(2) == "green" and last.group(1) == "0":
        return []
    rows = MISSING.findall(text[:last.start()].rsplit("[CENSUS] taken", 1)[-1])
    return [f"census: {cls} missing: {why.strip()}" for cls, why in rows] or [
        f"census verdict {last.group(2)} with {last.group(1)} missing"]
