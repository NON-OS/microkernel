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
"""What the relay may send for a file the catalogue lists. A package its
lock pins is served only as the bytes it pins (B, the BLAKE3 its listing
signs), or not at all; anything else passes through, for the guest's own
Alpine signature check to judge. --tamper serves a pinned package with one
byte changed, so a booted guest can be seen to refuse it."""

import re

ALPINE = "dl-cdn.alpinelinux.org"
APK = re.compile(r"/alpine/v3\.20/(?:main|community)/x86_64/([a-z0-9][a-z0-9._+-]{0,127}\.apk)")


def pinned(host: str, path: str, pins: dict):
    """(package, the BLAKE3 hex it must have) for a file the lock pins, or None."""
    m = APK.fullmatch(path) if host == ALPINE else None
    for name, pin in pins.items() if m else ():
        if m.group(1) == f"{name}-{pin['V']}.apk":
            return name, pin["B"]
    return None


def tampered(data: bytes) -> bytes:
    """The same bytes with the last one changed."""
    return data[:-1] + bytes([data[-1] ^ 0xFF]) if data else b"\xff"


def checked(host: str, path: str, data: bytes, pins: dict, tamper, b3) -> bytes:
    """The bytes to send, or ValueError saying why none are."""
    pin = pinned(host, path, pins)
    if pin is None:
        return data
    name, want = pin
    if b3(data) != want:
        raise ValueError(f"{path}: not the bytes the lock pins for {name}; not served")
    return tampered(data) if name in tamper else data
