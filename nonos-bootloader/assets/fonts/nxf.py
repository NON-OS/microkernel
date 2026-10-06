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
"""The atlas file: "NXF1", a face count; per face px, weight, ascent, line,
glyph count, table offset (16 B); per glyph code, left, top, width, height,
advance in 1/64 px, data offset (16 B); coverage, two pixels a byte."""

import struct


def pack(cov):
    cov = cov + [0] * (len(cov) % 2)
    return bytes(cov[i] << 4 | cov[i + 1] for i in range(0, len(cov), 2))


def write(out, faces):
    head, tables, data = bytearray(b"NXF1" + bytes([len(faces), 0, 0, 0])), bytearray(), bytearray()
    base, gsize = 8 + 16 * len(faces), 16 * sum(len(f[4]) for f in faces)
    for px, w, ascent, line, glyphs in faces:
        head += struct.pack("<BBHHHII", px, w, ascent, line, len(glyphs), base + len(tables), 0)
        for code, left, top, gw, gh, adv, cov in glyphs:
            tables += struct.pack("<BBhhHHHI", code, 0, left, top, gw, gh, adv, base + gsize + len(data))
            data += pack(cov)
    open(out, "wb").write(head + tables + data)

