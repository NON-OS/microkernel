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
"""The committed Alpine package the authentication tests run on
(vectors/README.md), and a one-bit change helper."""

import json
from pathlib import Path

HERE = Path(__file__).resolve().parent / "vectors"
APK = (HERE / "nitrokey-udev-rules-1.0.0-r2.apk").read_bytes()
REC = json.loads((HERE / "nitrokey-udev-rules-1.0.0-r2.json").read_text())


def flip(raw: bytes, at: int) -> bytes:
    return raw[:at] + bytes([raw[at] ^ 1]) + raw[at + 1:]
