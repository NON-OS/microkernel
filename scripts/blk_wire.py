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
"""The block wire as the source states it, for check_blk_wire.py: the client's
per-driver table, the constants a driver's protocol files define, and which
capsule registers which service name."""

import re
from pathlib import Path

CLIENT = "userland/nonos_blk_client/src"
# Each field of a row, and the names a driver's protocol may give it.
WANT = {"magic": "MAGIC", "VERSION": "VERSION", "HDR_LEN": "HDR_LEN", "capacity": "OP_CAPACITY",
        "read": "OP_READ_BLOCKS OP_READ", "write": "OP_WRITE_BLOCKS OP_WRITE", "flush": "OP_FLUSH"}
NUM = r"(0x[0-9a-fA-F_]+|\d[\d_]*)"


def num(v):
    return int(v.replace("_", ""), 0)


def consts(files):
    """Every `pub const NAME: type = number;` in `files`, by name."""
    pattern = rf"pub const (\w+):\s*\w+\s*=\s*{NUM}\s*;"
    return {n: num(v) for f in files for n, v in re.findall(pattern, f.read_text())}


def table(root):
    """The client's rows by driver: magic, the four ops, the first service name."""
    text = "".join(p.read_text() for p in sorted((root / CLIENT / "driver").glob("*.rs")))
    rows = {}
    # The magic arms only, not every arm that ends in a number (`instances`).
    magic = re.search(r"fn magic\b[^{]*\{(.*?)\n {0,4}\}", text, re.S)
    for d, v in re.findall(rf"Driver::(\w+)\s*=>\s*{NUM},", magic.group(1) if magic else ""):
        rows.setdefault(d, {})["magic"] = num(v)
    for d, body in re.findall(r"Driver::(\w+)\s*=>\s*Ops\s*\{([^}]*)\}", text):
        rows.setdefault(d, {}).update((k, num(v)) for k, v in re.findall(r"(\w+):\s*(\w+)", body))
    for d, name in re.findall(r"Driver::(\w+)\s*=>[^\n]*?b\"(driver\.\w+)\"", text):
        rows.setdefault(d, {}).setdefault("service", name)
    return rows


def handles(root):
    """The capsule directory that registers each service name."""
    found = {}
    for mk in Path(root).glob("userland/*/Capsule.mk"):
        for h in re.findall(r"^CAPSULE_HANDLE\s*:=\s*(\S+)", mk.read_text(), re.M):
            found[h] = mk.parent
    return found
