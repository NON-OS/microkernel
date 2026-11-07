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

"""An app's image from Alpine, built offline: both signed indexes fetched
fresh and verified under the keys the device pins, the closure resolved by
the device's rule, each package fetched from its own branch, held to its
signed size and authenticated by the device's chain, then cut and stored
as members. Nothing is signed here: the measurement is what the project's
signing step signs."""

import sys
from pathlib import Path

from nonos_market_catalogue import alpine_auth
from nonos_market_catalogue.alpine_index import signed_index
from nonos_market_catalogue.apkindex import text
from nonos_market_catalogue.fetch import fetch
from nonos_market_catalogue.sources import ARCH, BRANCHES, INDEX_CAP, RELEASE

from . import image_carried, image_closure, image_files, image_format, image_gtk, image_members
from .image_index import Index

# A package may inflate past what the device inflates in one piece
# (libreoffice-common: 343 MB); the image cuts it into members it can.
INFLATE = image_format.MAX_IMAGE


def index(mirror: str, cache: Path) -> Index:
    ix = Index()
    for at, branch in enumerate(BRANCHES):
        url = f"{mirror}/{RELEASE}/{branch}/{ARCH}/APKINDEX.tar.gz"
        raw = fetch(url, cache / f"{branch}-APKINDEX.tar.gz", INDEX_CAP, fresh=True)
        ix.add(text(signed_index(raw)), at)
    return ix


def files(mirror: str, cache: Path, pkg) -> bytes:
    """The package's files, or ValueError naming it."""
    branch = BRANCHES[pkg.branch]
    url = f"{mirror}/{RELEASE}/{branch}/{ARCH}/{pkg.name}-{pkg.version}.apk"
    apk = fetch(url, cache / branch / f"{pkg.name}-{pkg.version}.apk", pkg.size)
    if len(apk) != pkg.size:
        raise ValueError(f"{pkg.name}: {len(apk)} bytes where the signed index says {pkg.size}")
    try:
        return alpine_auth.package_files(apk, alpine_auth.checksum(pkg.checksum), INFLATE)
    except ValueError as why:
        (cache / branch / f"{pkg.name}-{pkg.version}.apk").unlink(missing_ok=True)
        raise ValueError(f"{pkg.name}: {why}") from None


def carried_out(name: str, tar: bytes, image: list) -> bytes:
    """`tar` without the files the NONOS image carries (`image`), each said."""
    tar, gone = image_carried.without(name, tar, image)
    for at in gone:
        print(f"nonos-linux-pack image: {name}: {at} left out: the NONOS image carries it", file=sys.stderr)
    return tar


def build(ix: Index, mirror: str, cache: Path, name: str, extras: tuple, chunk: int, gtk: bool = True,
          shipped: Path = None) -> dict:
    """The image, its chunk list, the measurement and what went in: the
    closure's members, each without the files the NONOS image carries
    (image_carried.py), then the files `shipped` holds (image_files.py), then
    GTK's caches when the closure carries GTK's
    tools and `gtk` asks for them (image_gtk.py)."""
    pkgs = image_closure.closure(ix, name, extras)
    image = image_carried.carried()
    parts = image_members.members([(p.name, carried_out(p.name, files(mirror, cache, p), image)) for p in pkgs])
    if shipped is not None:
        parts = parts + [image_files.member(shipped)]
    extra = image_gtk.caches(parts) if gtk else None
    parts = parts + [extra] if extra else parts
    image = image_format.build(parts)
    listing = image_format.chunk_list(image, chunk)
    return {
        "image": image,
        "list": listing,
        "measurement": image_format.measurement(listing),
        "packages": [(p.name, p.version) for p in pkgs],
        "members": parts,
    }
