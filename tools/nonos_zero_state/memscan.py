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

"""Counting a secret in a memory dump or a disk image, a slice at a time, so
an 8 GiB dump is never held whole."""

import os
import secrets
import string

SLICE = 64 << 20


def marker():
    """A secret no image holds: two fixed letters, then sixteen random ones.
    Letters only, so every keyboard layout setup offers types it alike."""
    return "zq" + "".join(secrets.choice(string.ascii_lowercase) for _ in range(16))


def count(path, needle):
    """How many times `needle` is in the file, across slice edges too."""
    needle = needle.encode()
    found, tail = 0, b""
    with open(path, "rb") as f:
        while True:
            chunk = f.read(SLICE)
            if not chunk:
                return found
            data = tail + chunk
            found += data.count(needle)
            # Keep one byte short of a match, so none is counted twice.
            tail = data[-(len(needle) - 1):]


def count_and_drop(path, needle):
    """Count, then delete the dump: it holds this boot's RAM."""
    try:
        return count(path, needle)
    finally:
        if os.path.exists(path):
            os.remove(path)
