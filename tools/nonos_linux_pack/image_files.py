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

"""Files an image ships beside its closure: a directory of them, at the
paths they land at, as one member named by its content (as GTK's caches
are, image_gtk.py), so two images never overwrite each other's. For
configuration that the app reads system-wide: LibreOffice's Work path, in
a registry file under its share/registry."""

from pathlib import Path

from . import image_format, image_members
from .image_gtk import tar
from .image_hash import digest


def files(root: Path) -> dict:
    """Every regular file under `root`, by its path below it; ValueError
    for a link or an empty tree, which an image does not carry."""
    out = {}
    for p in sorted(root.rglob("*")):
        if p.is_symlink():
            raise ValueError(f"{p.relative_to(root)} is a link")
        if p.is_file():
            out[p.relative_to(root).as_posix()] = p.read_bytes()
    if not out:
        raise ValueError(f"{root} holds no files")
    return out


def member(root: Path):
    body = tar(files(root))
    codec, stored = image_members.stored(body)
    return image_format.Part(b"nonos-files-" + digest(body)[:8].hex().encode(), codec, body, stored)
