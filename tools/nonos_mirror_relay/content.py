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
"""The relay as a development image's NONOS package mirror (--content DIR).
The image's linux_packages names the host as QEMU's user network shows it,
10.0.2.2:8080, and the installer asks with the mirror's name as its Host
line (install/tools_install.rs, get_as), so that name is the one served.
A file is served at /linux/<blake3>.tar only from DIR, and only when its
BLAKE3 is its name: anything else is 502, as no mirror may serve other bytes
than the signed index pins. It is read from DIR, never fetched."""

import re
from pathlib import Path

DEV = "10.0.2.2"
CONTENT = re.compile(r"/linux/([0-9a-f]{64})\.tar")


def answer(content, host: str, path: str, b3):
    """(code, body, log line) for a request the content serves, else None:
    a relay without --content, another host or another path is not one."""
    m = CONTENT.fullmatch(path) if content is not None and host == DEV else None
    if m is None:
        return None
    at = Path(content) / f"{m.group(1)}.tar"
    if not at.is_file():
        return 404, b"", f"404 {host}{path}: not in {content}"
    body = at.read_bytes()
    if b3(body) != m.group(1):
        return 502, b"", f"502 {host}{path}: its BLAKE3 is not its name; not served"
    return 200, body, f"200 {len(body):9} {host}{path} content"
