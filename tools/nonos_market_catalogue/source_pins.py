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
"""One pin of the sources lock (source_lock.py), in either shape: a build's,
by commit, or an upstream release asset's, by its URL, its sha256, its
BLAKE3 (A) and the checksum file that gave the sha256 (asset_recipe.py).
Both pin the bundled content's BLAKE3 (B) and size (S) and the logo's (G, R)."""

from .lock_file import HEX
from .source_recipe import URL

PINNED = ("commit", "B", "S", "G", "R")
ASSET = ("asset", "sha256", "A", "sums", "B", "S", "G", "R")
WIDE = {"commit": (40,), "sha256": (64,), "A": (64,), "B": (64,), "G": (0, 64), "R": (0, 64)}


def shape(pin) -> tuple:
    """The fields a pin of its shape has: an asset's when it names one."""
    return ASSET if isinstance(pin, dict) and "asset" in pin else PINNED


def checked(name: str, pin) -> None:
    """ValueError unless `pin` has exactly its shape's fields, each well formed."""
    if not isinstance(pin, dict) or set(pin) != set(shape(pin)) or not all(isinstance(v, str) for v in pin.values()):
        raise ValueError(f"{name!r} is not pinned by {', '.join(shape(pin))}")
    urls = all(URL.fullmatch(pin[f]) for f in ("asset", "sums") if f in pin)
    if not all(len(pin[f]) in n and HEX >= set(pin[f]) for f, n in WIDE.items() if f in pin) or not urls \
            or not pin["S"].isdigit():
        raise ValueError(f"{name}: a pin is not a commit or asset, its digests and size")
