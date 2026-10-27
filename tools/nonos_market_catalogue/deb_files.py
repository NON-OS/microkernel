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
"""The names a .deb places, so a program's DT_NEEDED and interpreter can be
held to the pinned packages before they are listed: the ar archive's data
member, xz, gzip or zstd, opened as the device opens it (deb/ar.rs)."""

import io
import shutil
import subprocess
import tarfile

LIB_DIRS = ("lib/", "lib64/", "usr/lib/", "usr/lib64/", "lib/x86_64-linux-gnu/", "usr/lib/x86_64-linux-gnu/")


def members(deb: bytes) -> dict:
    """name -> body of an ar archive, or ValueError."""
    if not deb.startswith(b"!<arch>\n"):
        raise ValueError("not an ar archive")
    out, at = {}, 8
    while at + 60 <= len(deb):
        head = deb[at:at + 60]
        name, size = head[:16].decode("ascii").strip().rstrip("/"), int(head[48:58].decode("ascii").strip())
        out[name] = deb[at + 60:at + 60 + size]
        at += 60 + size + (size & 1)
    return out


def data_tar(deb: bytes) -> bytes:
    parts = members(deb)
    if parts.get("debian-binary") != b"2.0\n":
        raise ValueError("not a version 2.0 .deb")
    name = next((n for n in parts if n.startswith("data.tar")), None)
    if name is None:
        raise ValueError("no data member")
    if name.endswith(".zst"):
        if shutil.which("zstd") is None:
            raise ValueError("a zstd data member, and no zstd to open it")
        return subprocess.run(["zstd", "-dc"], input=parts[name], stdout=subprocess.PIPE, check=True).stdout
    return parts[name]


def placed(deb: bytes) -> set:
    """Every path the package places, files and links, with no leading `./`."""
    with tarfile.open(fileobj=io.BytesIO(data_tar(deb))) as t:
        return {m.name.removeprefix("./") for m in t.getmembers() if m.isfile() or m.issym()}


def libraries(paths: set) -> set:
    """The base names of what sits directly in a library directory."""
    return {p.rsplit("/", 1)[1] for p in paths if "/" in p and p.rsplit("/", 1)[0] + "/" in LIB_DIRS}
