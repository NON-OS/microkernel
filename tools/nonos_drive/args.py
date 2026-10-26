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

"""nonos-drive's command line: the image, where to write, and the steps, given
on the line, in a file, or both, in that order. A line of a steps file that is
blank or starts with # is not a step."""

import argparse
from pathlib import Path


def parse(doc):
    ap = argparse.ArgumentParser(description=doc.split("\n", 1)[0],
                                 formatter_class=argparse.RawDescriptionHelpFormatter)
    ap.add_argument("--image", required=True)
    ap.add_argument("--out", required=True, type=Path)
    ap.add_argument("--steps", type=Path)
    ap.add_argument("--qemu-arg", action="append", default=[])
    ap.add_argument("step", nargs="*")
    a = ap.parse_args()
    steps = list(a.step)
    if a.steps:
        lines = (l.strip() for l in a.steps.read_text().splitlines())
        steps += [l for l in lines if l and not l.startswith("#")]
    return a, steps
