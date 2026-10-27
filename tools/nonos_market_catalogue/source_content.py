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
"""A source recipe's content, from the directory the seal passes as
--sources: the build's output for `<package>` as a tree, `<package>/`, or a
tar, `<package>.tar`. Either way it is bundled again as packages.bundle
bundles the in-tree tools, so its bytes depend only on its files: a tar's
own order, times and owners do not reach the listing. The content is
regular files only; a link or a device is refused, never followed."""

import tarfile
import tempfile
from pathlib import Path

from .packages import bundle


def tree_files(top: Path) -> list:
    """(path in the content, path under `top`) for each file, or ValueError."""
    out = []
    for at in sorted(top.rglob("*")):
        rel = at.relative_to(top).as_posix()
        if at.is_symlink() or not (at.is_file() or at.is_dir()):
            raise ValueError(f"its content's {rel} is not a regular file")
        if at.is_file():
            out.append((rel, rel))
    if not out:
        raise ValueError("its content holds no file")
    return out


def bundled(where: Path, package: str) -> bytes:
    """The content tar, or ValueError naming what is missing or refused."""
    if where is None:
        raise ValueError("no --sources directory holds its content")
    top, tar = where / package, where / f"{package}.tar"
    if top.exists() == tar.exists():
        said = "both" if top.exists() else "neither"
        raise ValueError(f"{where} holds {said} of {package}/ and {package}.tar")
    if top.exists():
        if top.is_symlink() or not top.is_dir():
            raise ValueError(f"{top} is not a directory")
        return bundle(tree_files(top), top)
    with tempfile.TemporaryDirectory() as d:
        try:
            with tarfile.open(tar, mode="r:") as t:
                if any(not (m.isfile() or m.isdir()) for m in t.getmembers()):
                    raise ValueError(f"{tar} holds a member that is not a regular file")
                t.extractall(d, filter="data")
        except tarfile.TarError as why:
            raise ValueError(f"{tar}: {why}") from None
        return bundle(tree_files(Path(d)), Path(d))
