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
"""What a tier needs in memory, read from the fetcher's own table
(userland/capsule_model_fetch/src/need.rs), the one source: the fetcher
admits a catalogue only when each tier's memory is what that table gives,
and the store shows it on the tier's card. The rule is qwenchat's before it
loads (userland/linux_guests/cpp/qwenmem.cpp): the weights; a key and a
value at two bytes for every layer, KV head and head element over 2048
positions; and a margin of 64 MiB plus 64 bytes for every embedding value
of a 512-token batch.
"""

import os
import re
import sys

NEED = os.path.join(os.path.dirname(os.path.dirname(os.path.dirname(os.path.abspath(__file__)))),
                    "userland/capsule_model_fetch/src/need.rs")
ROW = re.compile(r'\("([\w.-]+)", (\d+), (\d+), (\d+), (\d+)\),')
CONST = re.compile(r"pub const (POSITIONS|BATCH|MARGIN): u64 = (\d+)(?: << (\d+))?;")


def _read():
    text = open(NEED).read()
    shapes = {t: tuple(int(x) for x in rest) for t, *rest in ROW.findall(text)}
    consts = {k: int(v) << int(s or 0) for k, v, s in CONST.findall(text)}
    if not shapes or sorted(consts) != ["BATCH", "MARGIN", "POSITIONS"]:
        sys.exit(f"{NEED}: no shapes or constants read")
    return shapes, consts


SHAPES, _C = _read()
POSITIONS, BATCH, MARGIN = _C["POSITIONS"], _C["BATCH"], _C["MARGIN"]


def memory(tier, weights):
    """Bytes `tier` needs in memory with `weights` bytes of model files."""
    if tier not in SHAPES:
        sys.exit(f"{tier}: no shape known, so no memory need; add it to {NEED}")
    layers, kv_heads, head, embd = SHAPES[tier]
    kv = layers * kv_heads * 2 * head * 2 * POSITIONS
    return weights + kv + MARGIN + BATCH * embd * 64
