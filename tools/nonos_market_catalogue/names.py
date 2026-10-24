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
"""The names a listing is built from, held to Alpine's own spelling before
any of them reaches a URL, a cache path or a document's file name. The index
is fetched over the network, so its fields are untrusted: a version of
`../x` would otherwise write outside the cache. No pattern admits a `/` or
a leading dot, so no name can step out of a directory."""

import re

PACKAGE = re.compile(r"[a-z0-9][a-z0-9._+-]{0,63}")
VERSION = re.compile(r"[0-9][0-9a-z._+-]{0,63}")
LISTING = re.compile(r"(linux|community|nonos\.app)\.[a-z0-9][a-z0-9_.+-]{0,63}")
# A package is fetched whole before it is checked, so no larger one is asked
# for: the installer inflates at most 128 MiB from it (install/unpacked.rs).
MAX_PACKAGE = 128 << 20


def package(name) -> str:
    """`name` if it is a package name Alpine could have written, else ValueError."""
    if not isinstance(name, str) or not PACKAGE.fullmatch(name):
        raise ValueError(f"{name!r} is not a package name")
    return name


def version(text) -> str:
    """`text` if it is a version Alpine could have written, else ValueError."""
    if not isinstance(text, str) or not VERSION.fullmatch(text):
        raise ValueError(f"{text!r} is not a package version")
    return text


def listing(text) -> str:
    """`text` if it is a listing id of a namespace the generator writes, safe
    as a file name, else ValueError."""
    if not isinstance(text, str) or not LISTING.fullmatch(text):
        raise ValueError(f"{text!r} is not a catalogue listing id")
    return text


def index_record(rec) -> dict:
    """An APKINDEX record whose name, version and licence are usable, or
    ValueError (a bad value) or TypeError (a bad type) naming the field."""
    if not isinstance(rec, dict):
        raise TypeError("the index record is not a record")
    package(rec.get("P"))
    version(rec.get("V"))
    if not isinstance(rec.get("L", ""), str) or not isinstance(rec.get("base"), str):
        raise TypeError(f"{rec.get('P')}: the index gives no licence or source")
    size = rec.get("S")
    if not isinstance(size, str) or not size.isdecimal() or not 0 < int(size) <= MAX_PACKAGE:
        raise ValueError(f"{rec.get('P')}: the index gives no size, or one past {MAX_PACKAGE}")
    return rec


def cache_safe(rec: dict) -> bool:
    """Whether a record's name and version may name a cache file."""
    try:
        return bool(package(rec.get("P")) and version(rec.get("V")))
    except ValueError:
        return False
