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
"""An .apk built here the way Alpine lays one out, for the tests: a
signature, a control and a data member, each its own gzip stream."""

import gzip
import io
import tarfile

ELF = b"\x7fELF\x02\x01\x01" + b"\x00" * 57


def tar(files: list, links: dict | None = None) -> bytes:
    buf = io.BytesIO()
    with tarfile.open(fileobj=buf, mode="w", format=tarfile.USTAR_FORMAT) as t:
        for name, body in files:
            info = tarfile.TarInfo(name)
            info.size = len(body)
            t.addfile(info, io.BytesIO(body))
        for name, target in (links or {}).items():
            info = tarfile.TarInfo(name)
            info.type, info.linkname = tarfile.SYMTYPE, target
            t.addfile(info)
    return buf.getvalue()


def make(files: list, links: dict | None = None) -> bytes:
    """The three members, the data member holding `files` and `links`."""
    sign = tar([(".SIGN.RSA.test.rsa.pub", b"sig")])
    control = tar([(".PKGINFO", b"pkgname = test\n")])
    return b"".join(gzip.compress(m) for m in (sign, control, tar(files, links)))
