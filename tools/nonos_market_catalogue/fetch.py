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
"""Downloads for the catalogue: bounded in bytes and in time before they
are read, written to the cache only whole (a `.part` renamed into place),
and every failure, a refused connection, a cut transfer or a peer that
trickles past the deadline, a ValueError naming the URL. Nothing here
vouches for bytes: the caller checks them against the signed index every
run, so a bad cached file is found and dropped, never trusted."""

import http.client
import time
import urllib.request
from pathlib import Path

CHUNK = 1 << 16


def read_within(r, cap: int, deadline: float) -> bytes:
    """At most cap + 1 bytes of `r`, or ValueError once the deadline passes."""
    out = bytearray()
    while len(out) <= cap:
        if time.monotonic() > deadline:
            raise ValueError("the download passed its deadline")
        part = r.read(min(CHUNK, cap + 1 - len(out)))
        if not part:
            break
        out += part
    return bytes(out)


def fetch(url: str, to: Path, cap: int, seconds: float = 600, fresh: bool = False) -> bytes:
    """At most `cap` bytes, from the cache at `to` unless `fresh`, else from `url`."""
    if not fresh and to.is_file() and to.stat().st_size <= cap:
        return to.read_bytes()
    try:
        with urllib.request.urlopen(url, timeout=60) as r:
            data = read_within(r, cap, time.monotonic() + seconds)
    except (OSError, http.client.HTTPException) as why:
        raise ValueError(f"{url}: {why}") from None
    if len(data) > cap:
        raise ValueError(f"{url}: larger than {cap} bytes")
    to.parent.mkdir(parents=True, exist_ok=True)
    part = to.with_name(to.name + ".part")
    part.write_bytes(data)
    part.replace(to)
    return data
