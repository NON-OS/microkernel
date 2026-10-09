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
"""The tiers as the signed personality pins them, and a file checked against its pin."""

import glob
import hashlib
import os
import re
import sys

TOOLS = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
MODELS = os.path.join(TOOLS, "../userland/capsule_linux/src/linux/file/models")
PIN = re.compile(r'tier: "([\w.-]+)",\s*name: b"/([^"]+)",\s*bytes: ([\d_]+),\s*'
                 r'sha256: hex32\(b"([0-9a-f]{64})"\)')
TABLE = re.compile(r'pub const (\w+): &\[Pinned\] = &\[(.*?)\n\];', re.S)
FAMILIES = re.compile(r'FAMILIES: &\[&\[Pinned\]\] = &\[([\w, ]+)\]')

# What the data volume can keep, as the kernel counts it: a directory entry
# holds 56 bytes of name (src/fs/blockfs/dir_consts.rs, NAME_BYTES), and
# beside each file the import path writes <name>.sha256 and, while it comes,
# <name>.partial; a streamed name is at most 49 bytes with its slash
# (src/fs/blockfs_volume/import_feed/live.rs, NAME_MAX). pinned.rs holds the
# same rule, checked when the personality is built.
ENTRY_BYTES = 56
STREAM_NAME_MAX = 1 + ENTRY_BYTES - len(".partial")


def keepable(name):
    """Whether the volume can keep `name` (no slash), its record and its mark."""
    return (0 < len(name) and 1 + len(name) <= STREAM_NAME_MAX
            and len(name + ".sha256") <= ENTRY_BYTES
            and len(name + ".partial") <= ENTRY_BYTES)


def pins():
    """Every tier's files, family by family in the order pinned.rs gives."""
    tables = {}
    for path in sorted(glob.glob(os.path.join(MODELS, "pinned*.rs"))):
        for const, body in TABLE.findall(open(path).read()):
            tables[const] = PIN.findall(body)
    order = FAMILIES.search(open(os.path.join(MODELS, "pinned.rs")).read())
    if not order or sorted(tables) != sorted(order.group(1).replace(" ", "").split(",")):
        sys.exit(f"{MODELS}: the tables and pinned.rs's FAMILIES do not agree")
    out = {}
    for const in order.group(1).replace(" ", "").split(","):
        for tier, name, size, digest in tables[const]:
            if not keepable(name):
                sys.exit(f"{name}: {len(name) + 1} bytes with its slash, longer than the "
                         f"data volume keeps ({STREAM_NAME_MAX}); pin it under a shorter name")
            out.setdefault(tier, []).append((name, int(size.replace("_", "")), digest))
    return out


def check(path, size, digest):
    if not os.path.exists(path) or os.path.getsize(path) != size:
        return False
    h = hashlib.sha256()
    with open(path, "rb") as f:
        while chunk := f.read(1 << 24):
            h.update(chunk)
    return h.hexdigest() == digest
