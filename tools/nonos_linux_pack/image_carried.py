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

"""What of a package an image leaves out: the files the NONOS image itself
carries in its Linux tree, which the device refuses to let any package
replace (capsule_linux install/place/check.rs, member/image.rs shadows).
A package holding one of them would fail the whole install there, so the
image is built without that entry and says so by name; the image's own
file stays in its place. The list is the "linux" group of
tools/nix/store.json, as capsule_linux's build/image_files.rs reads it, so
the device and the image agree on it."""

import io
import json
import tarfile
from pathlib import Path

from nonos_market_catalogue.tarwalk import walk

from .image_split import END, entries

STORE = Path(__file__).resolve().parents[1] / "nix/store.json"
ROOT = "/linux/"


def carried(store: Path = STORE) -> list:
    """The image's own files, each from the root, sorted; ValueError when the
    catalogue holds no "linux" group of paths in /linux."""
    group = json.loads(store.read_text()).get("linux")
    if not isinstance(group, list) or not group:
        raise ValueError(f"{store}: no \"linux\" group")
    files = set()
    for item in group:
        path = item.get("path") if isinstance(item, dict) else None
        if not isinstance(path, str) or not path.startswith(ROOT):
            raise ValueError(f"{store}: {path!r} is not a path in /linux")
        files.add("/" + path[len(ROOT):])
    return sorted(files)


def shadows(at: str, link: bool, image: list) -> bool:
    """member/image.rs: `at` is one of `image`'s files, or, for a link, a
    directory above one."""
    if at in image:
        return True
    under = at.rstrip("/") + "/"
    return link and any(f.startswith(under) for f in image)


def _link_target(entry: bytes) -> str:
    """A hard link's target from the root, `./` cut as the installer cuts it."""
    with tarfile.open(fileobj=io.BytesIO(entry + END)) as t:
        to = t.getmembers()[0].linkname
    while to.startswith("./"):
        to = to[2:]
    return "/" + to.lstrip("/")


def without(name: str, tar: bytes, image: list) -> tuple:
    """`tar` with each entry the device would refuse as `image`'s left out,
    and the paths left out. Unchanged, byte for byte, when there are none.
    ValueError when a hard link kept would name an entry left out."""
    kept, gone = [], []
    for e in entries(tar):
        walked = walk(e + END)
        if len(walked) != 1:
            kept.append(e)
            continue
        w = walked[0]
        at = "/" + w.name.decode("utf-8", "surrogateescape")
        placed = not w.name.startswith(b".") and w.kind != "dir"
        if placed and shadows(at, w.kind == "symlink", image):
            gone.append(at)
            continue
        kept.append(e)
    if not gone:
        return tar, []
    for e in kept:
        w = walk(e + END)
        if len(w) == 1 and w[0].kind == "hardlink" and _link_target(e) in gone:
            raise ValueError(f"{name}: a hard link to {_link_target(e)}, which the image carries")
    return b"".join(kept) + END, gone
