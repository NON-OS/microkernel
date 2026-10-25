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
    """A secret no image holds: two fixed letters, then sixteen random ones,
    lower case only. Typed on a layout that moves letters (AZERTY, QWERTZ),
    it lands as other text, and the before-check fails rather than passes."""
    return "zq" + "".join(secrets.choice(string.ascii_lowercase) for _ in range(16))


def forms(secret):
    """Each way a program may hold typed text: bytes, UTF-16 and UTF-32."""
    return [secret.encode(e) for e in ("ascii", "utf-16-le", "utf-32-le")]


def count(path, needle):
    """How many times `needle`, in any of its forms, is in the file, across
    slice edges too."""
    needles = forms(needle)
    keep = max(len(n) for n in needles) - 1
    found, tail = 0, b""
    with open(path, "rb") as f:
        while True:
            chunk = f.read(SLICE)
            if not chunk:
                return found
            data = tail + chunk
            # Only matches that end in this chunk: one before it was counted.
            found += sum(data.count(n) - tail.count(n) for n in needles)
            tail = data[-keep:]


def count_and_drop(path, needle):
    """Count, then delete the dump: it holds this boot's RAM."""
    try:
        return count(path, needle)
    finally:
        if os.path.exists(path):
            os.remove(path)
