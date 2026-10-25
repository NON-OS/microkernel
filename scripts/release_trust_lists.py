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
"""The two lists of unsealed release capsules: the baseline the check reads,
and the one docs/build/feature-inventory.md gives between its unsealed
markers. They must name the same slugs, so the doc cannot drift."""

import re
from pathlib import Path

BEGIN, END = "<!-- unsealed: begin -->", "<!-- unsealed: end -->"


def read_baseline(path):
    lines = (l.split("#", 1)[0].strip() for l in Path(path).read_text().splitlines())
    return {l for l in lines if l}


def doc_slugs(text):
    """The backquoted slugs between the markers, or None without them."""
    if BEGIN not in text or END not in text.split(BEGIN, 1)[1]:
        return None
    return set(re.findall(r"`([a-z0-9-]+)`", text.split(BEGIN, 1)[1].split(END, 1)[0]))


def read_doc(path):
    return doc_slugs(Path(path).read_text())


def drift(baseline, listed):
    """One line per difference between the baseline and the doc's list."""
    if listed is None:
        return ["release trust: the doc has no unsealed list between its markers"]
    return ([f"release trust: {s} is in the baseline but not in the doc's list" for s in sorted(baseline - listed)]
            + [f"release trust: {s} is in the doc's list but not in the baseline" for s in sorted(listed - baseline)])


def unnamed(entries, slugs):
    """A shipped slug capsules.json has no entry for: no trust files to check."""
    known = {e["slug"] for e in entries}
    return [f"release trust: {s} ships in a release profile but capsules.json names no trust files for it"
            for s in sorted(set(slugs) - known)]
