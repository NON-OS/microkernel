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
"""The Mandelbrot set in 24-bit colour, two pixels to a character cell: each
cell is an upper half block whose foreground is the top pixel and whose
background is the one below, so a terminal of W columns and R rows draws
W by 2R pixels. Pure Python, the same arithmetic on any machine, so the same
pixels as on Linux.

    python3 /usr/share/nonos/tour/mandelbrot.py [WIDTH [HEIGHT]]
"""

import os
import sys

DEPTH = 96


def escape(cx, cy):
    """Iterations before z leaves the radius-2 disc, DEPTH if it never does."""
    zx = zy = 0.0
    for i in range(DEPTH):
        zx, zy = zx * zx - zy * zy + cx, 2.0 * zx * zy + cy
        if zx * zx + zy * zy > 4.0:
            return i
    return DEPTH


def colour(i):
    """Black inside the set; outside, a smooth palette by escape speed."""
    if i == DEPTH:
        return (0, 0, 0)
    t = i / DEPTH
    return (int(9 * (1 - t) * t ** 3 * 255), int(15 * (1 - t) ** 2 * t ** 2 * 255),
            int(8.5 * (1 - t) ** 3 * t * 255))


def main():
    try:
        size = os.get_terminal_size()
    except OSError:
        size = os.terminal_size((80, 24))
    width = int(sys.argv[1]) if len(sys.argv) > 1 else size.columns
    height = int(sys.argv[2]) if len(sys.argv) > 2 else 2 * (size.lines - 2)
    out = []
    for row in range(0, height - 1, 2):
        for col in range(width):
            cx = -2.2 + 3.2 * col / width
            top = colour(escape(cx, -1.2 + 2.4 * row / height))
            below = colour(escape(cx, -1.2 + 2.4 * (row + 1) / height))
            out.append("\x1b[38;2;%d;%d;%dm\x1b[48;2;%d;%d;%dm▀" % (top + below))
        out.append("\x1b[0m\n")
    sys.stdout.write("".join(out))
    print(f"{width} x {height} pixels, {DEPTH} iterations, CPython {sys.version.split()[0]}")


if __name__ == "__main__":
    main()
