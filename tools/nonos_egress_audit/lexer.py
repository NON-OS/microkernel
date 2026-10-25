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

"""Reading Rust and Python source a line at a time, comments cut."""

import re

BLOCK = re.compile(r"/\*.*?\*/", re.S)


def uncomment(text):
    """Block comments blanked, their line breaks kept, so line numbers hold."""
    return BLOCK.sub(lambda m: "\n" * m.group(0).count("\n"), text)


def code(line, quotes='"'):
    """A line with its // comment cut, the slashes of a URL in a string kept.
    Python quotes with either mark; in Rust a ' is a char or a lifetime."""
    out, quoted, i = [], None, 0
    while i < len(line):
        c = line[i]
        if c == "\\" and quoted:
            out.append(line[i:i + 2])
            i += 2
            continue
        if c in quotes and quoted in (None, c):
            quoted = None if quoted else c
        if not quoted and line.startswith("//", i):
            break
        out.append(c)
        i += 1
    return "".join(out)


def host_of(literal):
    """A URL is listed by its host; a bare name is its own host."""
    if "://" not in literal:
        return literal
    place = re.split(r"[/?#]", literal.split("://", 1)[1], maxsplit=1)[0]
    place = place.rsplit("@", 1)[-1]
    return place if place.startswith("[") else place.split(":", 1)[0]
