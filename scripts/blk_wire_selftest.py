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
"""The decoys for check_blk_wire.py: a client table with two rows, one capsule
that defines its own protocol constants and one on the shared block wire, and
each drift planted in turn must be reported."""

import tempfile
from pathlib import Path

from blk_wire import CLIENT, WIRE


def const(pairs):
    return "".join(f"pub const {n}: u16 = {v};\n" for n, v in pairs)


# D defines its own protocol. W re-exports the shared wire, which answers a
# capacity with GEOMETRY and writes its flush in hex, so that dropping D's
# flush shows the shared crate is never read for a capsule that does not use it.
OWN = (("MAGIC", "0x4E43"), ("VERSION", 1), ("HDR_LEN", 20), ("OP_CAPACITY", 2),
       ("OP_READ", 3), ("OP_WRITE_BLOCKS", 4), ("OP_FLUSH", 5))
SHARED = (("MAGIC", "0x4E57"), ("VERSION", 1), ("HDR_LEN", 20), ("OP_GEOMETRY", 2),
          ("OP_READ", 3), ("OP_WRITE", 4), ("OP_FLUSH", "0x5"))
TABLE = ("fn magic(self) {\n    Driver::D => 0x4E43,\n    Driver::W => 0x4E57,\n}\n"
         "fn instances(self) {\n    Driver::D => 1,\n}\n"
         'Driver::D => b"driver.d0"\nDriver::W => &[b"driver.w0"]\n'
         "Driver::D => Ops { capacity: 2, read: 3, write: 4, flush: 5 },\n"
         "Driver::W => Ops { capacity: 2, read: 3, write: 4, flush: 5 },\n")
FILES = {f"{CLIENT}/driver/t.rs": TABLE,
         f"{CLIENT}/wire/header.rs":
             "pub const VERSION: u16 = 1;\npub const HDR_LEN: usize = 20;\n",
         "userland/c/Capsule.mk": "CAPSULE_HANDLE := driver.d0\n",
         "userland/c/src/protocol/p.rs": const(OWN),
         "userland/w/Capsule.mk": "CAPSULE_HANDLE := driver.w0\n",
         "userland/w/src/protocol/mod.rs": "pub use nonos_blk_wire::{HDR_LEN, OP_GEOMETRY};\n",
         f"{WIRE}/h.rs": const(SHARED)}
# Each pair plants one drift by replacing text in every file; the first plants
# none. The last has W define its own read op, which wins over the shared one.
DECOYS = (("", ""), ("write: 4", "write: 7"), ("0x4E43,", "0x4E44,"), ("driver.d0\n", "x\n"),
          ("0x4E57,", "0x4E58,"), ("OP_GEOMETRY: u16 = 2", "OP_GEOMETRY: u16 = 9"),
          ("nonos_blk_wire::", "elsewhere::"), ("OP_FLUSH: u16 = 5;", "OP_FLUSHED: u16 = 5;"),
          ("OP_GEOMETRY};\n", "OP_GEOMETRY};\npub const OP_READ: u16 = 8;\n"))


def self_test(problems):
    """Run `problems` over the fixture once per decoy."""
    for old, new in DECOYS:
        with tempfile.TemporaryDirectory() as d:
            for rel, text in FILES.items():
                (Path(d) / rel).parent.mkdir(parents=True, exist_ok=True)
                (Path(d) / rel).write_text(text.replace(old, new) if old else text)
            assert bool(problems(Path(d))[0]) == bool(old), (old, problems(Path(d)))
    print("blk wire: self-test passed, each decoy drift was reported")
