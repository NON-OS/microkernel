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
"""The Debian packages a glibc program from the NONOS mirror runs on, as its
content pins them at usr/share/nonos-debs/<package>, one line a package:
`<name> <version> <pool file> <sha256>`. The device reads it by the same
rules (install/deb/pinned.rs) and installs each before the program
(install/deb/needs.rs). The content's BLAKE3 is the listing's, so these
pins are signed with it. A recipe's `needs` names the packages, space
separated; its content's pins must hold every one."""

import re

DIR = "usr/share/nonos-debs/"
MAX_NEEDS = 256
# Debian policy 5.6.1, and the device's bound.
NAME = re.compile(r"[a-z0-9][a-z0-9+.-]{1,63}")
VERSION = re.compile(r"[!-~]{1,128}")
SHA256 = re.compile(r"[0-9a-f]{64}")
# install/deb/fields.rs `safe`: a pool path under the mirror's root.
POOL = re.compile(r"[A-Za-z0-9._+~:%/-]+\.deb")


def names(text: str) -> list:
    """A recipe's `needs`, or ValueError."""
    got = text.split(" ")
    if not got or len(got) > MAX_NEEDS or len(set(got)) != len(got) or not all(map(NAME.fullmatch, got)):
        raise ValueError(f"its needs {text!r} are not distinct Debian package names, space separated")
    return got


def parse(text: str) -> list:
    """[(name, version, pool file, sha256)], or ValueError: whole or not at all."""
    out = []
    for line in text.splitlines():
        f = line.split(" ")
        ok = len(f) == 4 and NAME.fullmatch(f[0]) and VERSION.fullmatch(f[1]) and SHA256.fullmatch(f[3])
        if not ok or not POOL.fullmatch(f[2]) or any(p in ("", ".", "..") for p in f[2].split("/")):
            raise ValueError(f"its Debian pins have a line that is not name, version, pool file and sha256: {line!r}")
        out.append(tuple(f))
    if not out or len(out) > MAX_NEEDS or len({p[0] for p in out}) != len(out):
        raise ValueError(f"its Debian pins are empty, past {MAX_NEEDS}, or name a package twice")
    return out


def carried(entries, package: str):
    """The pins `entries` (tarwalk files) carry for `package`, or None."""
    last = {e.name: e.body for e in entries}
    body = last.get((DIR + package).encode())
    # Not ASCII is a UnicodeDecodeError, which is a ValueError.
    return None if body is None else parse(body.decode("ascii"))


def lines(pins: list) -> str:
    """The file, from parse's tuples; parse reads it back as it was."""
    return "".join(" ".join(p) + "\n" for p in pins)
