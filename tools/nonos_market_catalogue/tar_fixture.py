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
"""Raw tar records built byte by byte for the walk's tests, where Python's
own tarfile would refuse to write what a hostile archive holds."""


def head(name: bytes, flag: bytes, size: int, prefix: bytes = b"") -> bytes:
    h = bytearray(512)
    h[:len(name)] = name
    h[124:136] = f"{size:011o}\0".encode()
    h[156:157] = flag
    h[257:262] = b"ustar"
    h[345:345 + len(prefix)] = prefix
    return bytes(h)


def pax(key: str, value: str) -> bytes:
    body = f" {key}={value}\n"
    n = len(body) + 2
    return f"{n + len(str(n)) - 2}{body}".encode()


def rec(name: bytes, flag: bytes, body: bytes = b"", prefix: bytes = b"") -> bytes:
    pad = b"\0" * (-len(body) % 512)
    return head(name, flag, len(body), prefix) + body + pad
