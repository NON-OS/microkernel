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

"""The small image both sides are held to: three packages' tars, written
here with no times and no owners, one stored, two xz. The Python suite
checks the builder still writes the committed vector byte for byte, and
userland/app_image_proofs checks the device's parser and decompressor read
it (userland/nonos_app_image/tests/vectors/)."""

import io
import tarfile
from pathlib import Path

from . import image_format, image_members

VECTORS = Path(__file__).resolve().parents[2] / "userland/nonos_app_image/tests/vectors"
CHUNK = image_format.CHUNK_MIN


def tar(files: dict, links: dict = None) -> bytes:
    buf = io.BytesIO()
    with tarfile.open(fileobj=buf, mode="w", format=tarfile.PAX_FORMAT) as t:
        for path, body in sorted(files.items()):
            info = tarfile.TarInfo(path)
            info.size, info.mode, info.mtime, info.uid, info.gid = len(body), 0o755, 0, 0, 0
            info.uname = info.gname = ""
            t.addfile(info, io.BytesIO(body))
        for path, to in sorted((links or {}).items()):
            info = tarfile.TarInfo(path)
            info.type, info.linkname, info.mtime, info.uid, info.gid = tarfile.SYMTYPE, to, 0, 0, 0
            info.uname = info.gname = ""
            t.addfile(info)
    return buf.getvalue()


def packages() -> list:
    long = "usr/share/demo/" + "a-long-name-" * 10 + "that-needs-pax.txt"
    elf = b"\x7fELF\x02\x01\x01" + bytes(57) + b"demo program" * 40
    return [
        ("demo", tar({"usr/bin/demo": elf, long: b"pax\n"}, {"usr/bin/demo-link": "demo"})),
        ("libdemo", tar({"usr/lib/libdemo.so.1": bytes(range(256)) * 300})),
        ("demo-data", tar({"usr/share/demo/x": b"x" * 9000, "usr/share/demo/y": b"y\n"})),
    ]


def build() -> tuple:
    parts = image_members.members(packages())
    image = image_format.build(parts)
    return image, image_format.chunk_list(image, CHUNK)
