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
"""Rust's text handling, where the device's parsers lean on it: `str::lines`
(split at \\n, one trailing \\r dropped, no empty last line) and
`str::trim` (Unicode White_Space only, which is not what Python's strip
takes away)."""

WHITE_SPACE = frozenset(map(chr, (*range(0x09, 0x0E), 0x20, 0x85, 0xA0, 0x1680,
                                  *range(0x2000, 0x200B), 0x2028, 0x2029, 0x202F, 0x205F, 0x3000)))


def lines(text: str) -> list:
    out = text.split("\n")
    if out and out[-1] == "":
        out.pop()
    return [x.removesuffix("\r") for x in out]


def trim(text: str) -> str:
    start, end = 0, len(text)
    while start < end and text[start] in WHITE_SPACE:
        start += 1
    while end > start and text[end - 1] in WHITE_SPACE:
        end -= 1
    return text[start:end]
