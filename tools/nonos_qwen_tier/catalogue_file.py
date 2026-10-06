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
"""Writing the catalogue where the image build or a mirror wants it."""

import os
import sys

from .catalogue import body, signed
from .layout import check_base


def write(out, table, base, serial, seed, pubkey):
    """Build, sign and write the catalogue. Without the operator seed the file
    is left empty, which the fetcher reads as no catalogue, as the market does."""
    base = check_base(base) if base else ""
    if not base:
        print("catalogue: no --nonos-mirror, so no file names the NONOS repository",
              file=sys.stderr)
    raw = body(table, base, serial)
    os.makedirs(os.path.dirname(os.path.abspath(out)), exist_ok=True)
    if not os.path.exists(seed):
        print(f"catalogue: no operator seed at {seed}; the image has no model catalogue",
              file=sys.stderr)
        return open(out, "wb").close()
    with open(seed, "rb") as s, open(pubkey, "rb") as k:
        blob = signed(raw, s.read(), k.read())
    with open(out, "wb") as f:
        f.write(blob)
    print(f"catalogue: {len(table)} tiers, {len(blob)} bytes, serial {serial} -> {out}")
