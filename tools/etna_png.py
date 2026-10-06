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
"""Just enough PNG for the Etna photographs: 8-bit RGB in, box filter, RGB out."""

import struct
import sys
import zlib


def decode(path):
    data = path.read_bytes()
    w, h, depth, kind, _, _, lace = struct.unpack(">IIBBBBB", data[16:29])
    if (depth, kind, lace) != (8, 2, 0):
        sys.exit(f"{path}: want 8-bit RGB without interlace")
    raw, i = b"", 8
    while i < len(data):
        n, tag = struct.unpack(">I4s", data[i:i + 8])
        raw += data[i + 8:i + 8 + n] if tag == b"IDAT" else b""
        i += 12 + n
    return w, h, unfilter(zlib.decompress(raw), w, h)


def unfilter(raw, w, h):
    stride, prev, rows, o = w * 3, bytearray(w * 3), [], 0
    for _ in range(h):
        f, line = raw[o], bytearray(raw[o + 1:o + 1 + stride])
        o += 1 + stride
        for x in range(stride):
            a = line[x - 3] if x >= 3 else 0
            b, c = prev[x], prev[x - 3] if x >= 3 else 0
            p = a + b - c
            pred = [0, a, b, (a + b) // 2,
                    a if abs(p - a) <= abs(p - b) and abs(p - a) <= abs(p - c) else b if abs(p - b) <= abs(p - c) else c][f]
            line[x] = (line[x] + pred) & 255
        rows.append(bytes(line))
        prev = line
    return rows


def shrink(w, h, rows, out_w):
    out_h = round(out_w * h / w)
    out = []
    for y in range(out_h):
        y0, y1 = y * h // out_h, max((y + 1) * h // out_h, y * h // out_h + 1)
        line = bytearray()
        for x in range(out_w):
            x0, x1 = x * w // out_w, max((x + 1) * w // out_w, x * w // out_w + 1)
            n, acc = (y1 - y0) * (x1 - x0), [0, 0, 0]
            for r in rows[y0:y1]:
                for xx in range(x0, x1):
                    for c in range(3):
                        acc[c] += r[xx * 3 + c]
            line += bytes(v // n for v in acc)
        out.append(bytes(line))
    return out_w, out_h, out


def encode(w, h, rows):
    chunk = lambda t, d: struct.pack(">I", len(d)) + t + d + struct.pack(">I", zlib.crc32(t + d))
    body = zlib.compress(b"".join(b"\0" + r for r in rows), 9)
    return (b"\x89PNG\r\n\x1a\n" + chunk(b"IHDR", struct.pack(">IIBBBBB", w, h, 8, 2, 0, 0, 0))
            + chunk(b"IDAT", body) + chunk(b"IEND", b""))
