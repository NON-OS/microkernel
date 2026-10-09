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
"""Build the browser's text-decoding tables from the Encoding Standard.
usage: gen_encoding_index.py <dir holding encodings.json and index-*.txt>
Both come from https://encoding.spec.whatwg.org/. Each table is a run of
little-endian u16 code points indexed by pointer (.idx files), 0 where the index has
none; Big5's plane-2 code points keep their low 16 bits and set a bit in
big5_astral.idx. gb18030 ranges are (pointer, code point) u32 pairs.
labels.txt holds one encoding per line, its name first (#n for the n-th
single-byte table), then its labels. SOURCE.txt records each index's
identifier and date."""
import json, struct, sys
from pathlib import Path

OUT = Path("userland/capsule_browser/src/browser/http/response/charset/index")
SRC = Path(sys.argv[1])


def index(name):
    rows, meta = {}, []
    for line in (SRC / f"index-{name}.txt").read_text(encoding="utf-8").split("\n"):
        if line.startswith("# Identifier") or line.startswith("# Date"):
            meta.append(line[2:])
        elif line.strip() and not line.startswith("#"):
            p, c = line.split("\t")[:2]
            rows[int(p)] = int(c, 16)
    SOURCES[name] = f"index-{name}.txt " + " ".join(meta)
    return rows


def u16s(rows, size):
    return b"".join(struct.pack("<H", rows.get(p, 0) & 0xFFFF) for p in range(size))


SOURCES, labels, single = {}, [], []
for group in json.loads((SRC / "encodings.json").read_text()):
    for e in group["encodings"]:
        name = e["name"].lower()
        if group["heading"] == "Legacy single-byte encodings":
            table = "iso-8859-8" if name == "iso-8859-8-i" else name
            single.append(u16s(index(table), 128))
            name = f"#{len(single) - 1}"
        labels.append(" ".join([name] + e["labels"]))
OUT.mkdir(parents=True, exist_ok=True)
(OUT / "labels.txt").write_text("\n".join(labels) + "\n")
(OUT / "single_byte.idx").write_bytes(b"".join(single))
for name, file in [("jis0208",) * 2, ("jis0212",) * 2, ("gb18030",) * 2, ("euc-kr", "euc_kr")]:
    rows = index(name)
    (OUT / f"{file}.idx").write_bytes(u16s(rows, max(rows) + 1))
big5 = index("big5")
(OUT / "big5.idx").write_bytes(u16s(big5, max(big5) + 1))
bits = bytearray((max(big5) + 8) // 8)
for p, c in big5.items():
    if c > 0xFFFF:
        assert c >> 16 == 2, "Big5 leaves plane 2"
        bits[p // 8] |= 1 << (p % 8)
(OUT / "big5_astral.idx").write_bytes(bytes(bits))
ranges = index("gb18030-ranges")
(OUT / "gb18030_ranges.idx").write_bytes(b"".join(struct.pack("<II", p, c) for p, c in sorted(ranges.items())))
(OUT / "SOURCE.txt").write_text("https://encoding.spec.whatwg.org/\n" + "\n".join(SOURCES.values()) + "\n")
