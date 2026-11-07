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

"""An image's members from authenticated packages: each package's files
as the device's own check yields them (capsule_linux install/auth/
package.rs, the catalogue's alpine_auth.py), cut where a pack needs it,
and stored xz-compressed when that is smaller."""

import lzma

from . import image_format, image_split

# One preset, one check, one thread: the same tar gives the same stream on
# the same liblzma. The device reads every check type (userland/xz).
XZ = {"format": lzma.FORMAT_XZ, "check": lzma.CHECK_CRC64, "preset": 9}


def stored(tar: bytes) -> tuple:
    packed = lzma.compress(tar, **XZ)
    return (image_format.XZ, packed) if len(packed) < len(tar) else (image_format.TAR, tar)


def members(packages: list) -> list:
    """`packages` is [(name, files tar)], the listed package first. Each
    becomes one member, or several when its files need more than one pack."""
    parts = []
    for i, (name, tar) in enumerate(packages):
        cuts = image_split.split(tar, held=i == 0)
        for member, cut in zip(image_split.names(name, len(cuts)), cuts):
            codec, body = stored(cut)
            parts.append(image_format.Part(member.encode(), codec, cut, body))
    return parts
