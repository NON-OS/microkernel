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

"""App images from --content DIR, for a development boot (capsule_linux
install/image/): /linux/<pin>.chunks only when its BLAKE3 is <pin>, and
/linux/<pin>.image only beside that list and of the length it names, whole
or by one `Range: bytes=F-L`, as the installer asks for each chunk. The
device holds every chunk to the list's digests whatever this serves."""

import re
import struct
from pathlib import Path

from .content import DEV

IMAGE = re.compile(r"/linux/([0-9a-f]{64})\.(chunks|image)")
RANGE = re.compile(r"bytes=(\d+)-(\d+)")


def answer(content, host: str, path: str, b3, rng: str = ""):
    """(code, body, log line, headers) for an image request, else None."""
    m = IMAGE.fullmatch(path) if content is not None and host == DEV else None
    if m is None:
        return None
    pin, kind = m.groups()
    listing = Path(content) / f"{pin}.chunks"
    if not listing.is_file():
        return 404, b"", f"404 {host}{path}: not in {content}", {}
    raw = listing.read_bytes()
    if b3(raw) != pin or len(raw) < 32:
        return 502, b"", f"502 {host}{path}: the list's BLAKE3 is not its name; not served", {}
    if kind == "chunks":
        return 200, raw, f"200 {len(raw):9} {host}{path} list", {}
    at = Path(content) / f"{pin}.image"
    total = struct.unpack_from("<Q", raw, 16)[0]
    if not at.is_file() or at.stat().st_size != total:
        return 404, b"", f"404 {host}{path}: no image of the list's length", {}
    if not rng:
        body = at.read_bytes()
        return 200, body, f"200 {len(body):9} {host}{path} image", {}
    r = RANGE.fullmatch(rng.strip())
    first, last = (int(r.group(1)), int(r.group(2))) if r else (1, 0)
    if first > last or last >= total:
        return 416, b"", f"416 {host}{path}: {rng}", {"Content-Range": f"bytes */{total}"}
    with at.open("rb") as f:
        f.seek(first)
        body = f.read(last - first + 1)
    line = f"206 {len(body):9} {host}{path} bytes {first}-{last}"
    return 206, body, line, {"Content-Range": f"bytes {first}-{last}/{total}"}
