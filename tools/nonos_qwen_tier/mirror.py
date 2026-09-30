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
"""`mirror`: the NONOS model repository, laid out for upload.

Each chosen tier's files are fetched from the Qwen team's repository, a cut
download resumed, checked against their pins and kept as <dir>/<tier>/<file>,
the layout layout.py names. The signed catalogue then goes beside them as
<dir>/catalogue.bin; it lists every pinned tier, each file under this
repository first, so a mirror of fewer tiers leaves the fetcher to fall back
to the upstream file for the rest. The base URL, the operator seed and the
signature are checked before a byte is downloaded.
"""

import os
import sys

from .catalogue import body, signed
from .download import fetch, url
from .layout import check_base


def lay_out(table, chosen, out, source=url):
    """Fetch every file of `chosen` into `out`, from `source(name)`."""
    for tier in chosen:
        for name, size, digest in table[tier]:
            fetch(name, size, digest, os.path.join(out, tier), source(name))


def mirror(table, chosen, out, base, serial, seed, pubkey, source=url):
    raw = body(table, check_base(base), serial)
    if not os.path.exists(seed):
        sys.exit(f"mirror: no operator seed at {seed}")
    with open(seed, "rb") as s, open(pubkey, "rb") as k:
        blob = signed(raw, s.read(), k.read())
    lay_out(table, chosen, out, source)
    os.makedirs(out, exist_ok=True)
    with open(os.path.join(out, "catalogue.bin"), "wb") as f:
        f.write(blob)
    print(f"catalogue: {len(table)} tiers, {len(blob)} bytes, serial {serial} -> {out}")
