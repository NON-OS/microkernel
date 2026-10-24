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
"""An Alpine package's files, and the program the installer records for it.

An .apk is three gzip members, the signature, the control file and the
data, each a tar cut short; inflated and joined they read as one tar.
capsule_linux records usr/bin/<name> when the package has it and is an
ELF, else the first ELF under usr/bin in archive order
(install/record/program.rs), and the policy has to name that one."""

import io
import tarfile
import zlib

LIMIT = 256 << 20


def inflate(raw: bytes) -> bytes:
    """Every gzip member of `raw`, joined; refused past LIMIT."""
    out, rest = bytearray(), raw
    while rest:
        z = zlib.decompressobj(31)
        try:
            out += z.decompress(rest, LIMIT + 1 - len(out))
        except zlib.error as why:
            raise ValueError(f"the package is not gzip: {why}") from None
        if len(out) > LIMIT or z.unconsumed_tail:
            raise ValueError("the package inflates past its limit")
        if not z.eof:
            raise ValueError("the package ends inside a gzip member")
        rest = z.unused_data
    return bytes(out)


def files(raw: bytes) -> list:
    """(name, body) for every regular file, in archive order."""
    try:
        with tarfile.open(fileobj=io.BytesIO(inflate(raw)), mode="r:", ignore_zeros=True) as tar:
            return [(m.name, tar.extractfile(m).read()) for m in tar.getmembers() if m.isfile()]
    except tarfile.TarError as why:
        raise ValueError(f"the package is not a tar: {why}") from None


def program(raw: bytes, name: str):
    """(path, body) of the program a package starts as, or None."""
    elves = [(n, b) for n, b in files(raw) if n.startswith("usr/bin/") and b.startswith(b"\x7fELF")]
    own = [e for e in elves if e[0] == f"usr/bin/{name}"]
    return (own or elves or [None])[0]
