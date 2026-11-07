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

"""BLAKE3 for images: b3sum when it is on the PATH, the pure blake3.py
otherwise. Both give the same digest (test_image_format.py holds them to
each other); b3sum is only faster, and an image is hundreds of megabytes."""

import shutil
import subprocess

from .blake3 import blake3


def digest(data: bytes) -> bytes:
    tool = shutil.which("b3sum")
    if tool is None or len(data) < (1 << 20):
        return blake3(data)
    out = subprocess.run([tool, "--no-names", "-"], input=data, capture_output=True, check=True)
    text = out.stdout.decode("ascii").strip()
    if len(text) != 64:
        raise ValueError("b3sum gave no 32-byte digest")
    return bytes.fromhex(text)
