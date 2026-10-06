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
"""A number in CI that moves one way only."""

import sys


def held(tag, value, baseline, grows_ok):
    """0 when `value` has not moved the wrong way past the baseline file's
    number, 1 when it has, 2 when the file is not a number."""
    want = baseline.read_text().strip()
    if not want.isdigit():
        print(f"{tag}: baseline {baseline} is not an integer: {want!r}", file=sys.stderr)
        return 2
    limit = int(want)
    print(f"[{tag}] baseline {limit}, delta {value - limit:+d}")
    if (value < limit) if grows_ok else (value > limit):
        way = "shrank below" if grows_ok else "grew past"
        print(f"::error::{tag} {way} its baseline: {value} against {limit}. "
              "Fix the change, or justify moving the baseline in the PR.", file=sys.stderr)
        return 1
    return 0
