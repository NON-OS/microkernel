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

"""`nonos-linux-pack image --package NAME --out DIR`: build an app's image
from Alpine with its closure, write `<measurement>.image` and
`<measurement>.chunks` for the package mirror, and print the unsigned
catalogue entry. The same inputs build the same bytes."""

import argparse
import json
from pathlib import Path

from . import image_box, image_build, image_entry

MIRROR = "https://dl-cdn.alpinelinux.org/alpine"
# 4 MiB: on the mixnet each chunk is one round trip on the kept stream, and a
# broken stream loses at most one chunk.
CHUNK = 4 << 20


def parser() -> argparse.ArgumentParser:
    p = argparse.ArgumentParser(prog="nonos-linux-pack image", description=__doc__)
    p.add_argument("--package", required=True, help="the Alpine package the listing sells")
    p.add_argument("--with", dest="extras", action="append", default=[],
                   help="a package the closure adds that nothing names (as a recipe's `with`)")
    p.add_argument("--out", required=True, type=Path, help="where the image and its list go")
    p.add_argument("--cache", type=Path, help="the fetched packages, kept between runs")
    p.add_argument("--mirror", default=MIRROR, help="Alpine's tree, up to its release directories")
    p.add_argument("--package-mirror", default="mirror.nonos.software:80",
                   help="name:port of the NONOS package mirror the device fetches from")
    p.add_argument("--chunk", type=int, default=CHUNK, help="chunk size, a power of two")
    p.add_argument("--mask", default="", help="the box's services, as a recipe's mask (WAYLAND FILES_HOME ...)")
    p.add_argument("--program", default="", help="the program the policy names, a path in the image")
    p.add_argument("--files", type=Path, help="a directory of files the image ships, at the paths they land at")
    p.add_argument("--no-gtk-caches", action="store_true",
                   help="leave GTK's caches out; the device then builds them at every launch")
    return p


def main(argv) -> int:
    a = parser().parse_args(argv)
    cache = a.cache or a.out / "cache"
    try:
        ix = image_build.index(a.mirror, cache)
        built = image_build.build(ix, a.mirror, cache, a.package, tuple(a.extras), a.chunk, not a.no_gtk_caches,
                                  a.files)
    except ValueError as why:
        print(f"nonos-linux-pack image: {a.package}: {why}")
        return 1
    try:
        box = image_box.box(built["members"], a.mask, a.program) if a.program else None
    except ValueError as why:
        print(f"nonos-linux-pack image: {a.package}: {why}")
        return 1
    root = built["measurement"].hex()
    a.out.mkdir(parents=True, exist_ok=True)
    (a.out / f"{root}.image").write_bytes(built["image"])
    (a.out / f"{root}.chunks").write_bytes(built["list"])
    entry = image_entry.entry(a.package, built, a.package_mirror, a.chunk)
    if box is not None:
        entry["box"] = box
    (a.out / f"{root}.json").write_text(json.dumps(entry, indent=2) + "\n")
    print(json.dumps(entry, indent=2))
    return 0
