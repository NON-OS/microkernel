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
"""The gzip vectors tests/no_memory.rs reads. Run from userland/inflate:
python3 tests/vectors/gen.py. mtime is 0, so the bytes are the same on
every run."""

import gzip
import struct
from pathlib import Path

HERE = Path(__file__).resolve().parent
BIG = bytes((i * 7 + i // 251) & 0xFF for i in range(1 << 20))


def member(data: bytes) -> bytes:
    return gzip.compress(data, compresslevel=9, mtime=0)


def isize(gz: bytes, n: int) -> bytes:
    return gz[:-4] + struct.pack("<I", n & 0xFFFFFFFF)


one = member(BIG)
(HERE / "big.gz").write_bytes(one)
(HERE / "apk_shape.gz").write_bytes(member(b"sig") + member(b"control") + one)
(HERE / "isize_more.gz").write_bytes(isize(one, len(BIG) + 1))
# Two halves: each inflates under a 700 KiB limit, joined they do not.
(HERE / "halves.gz").write_bytes(member(BIG[: len(BIG) // 2]) + member(BIG[len(BIG) // 2 :]))
(HERE / "isize_less.gz").write_bytes(isize(one, len(BIG) - 1))
