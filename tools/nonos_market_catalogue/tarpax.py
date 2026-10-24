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
"""Pax extended headers, as install/tar_pax.rs reads them: `length
key=value\\n` records, of which only `path` and `linkpath` are taken."""


class Overrides:
    def __init__(self):
        self.path = None
        self.link = None


def decimal(text: bytes):
    if not text or not all(0x30 <= b <= 0x39 for b in text):
        return None
    return int(text)


def read(body: bytes, into: Overrides) -> None:
    at = 0
    while at < len(body):
        space = body.find(b" ", at)
        if space < 0:
            return
        length = decimal(body[at:space])
        if length is None:
            return
        end = at + length
        if end > len(body) or length <= space - at + 1:
            return
        record = body[space + 1:end - 1]
        key, eq, value = record.partition(b"=")
        if eq and key == b"path":
            into.path = value
        elif eq and key == b"linkpath":
            into.link = value
        at = end
