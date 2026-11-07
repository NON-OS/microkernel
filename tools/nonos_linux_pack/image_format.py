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

"""The app image and its chunk list, written byte for byte as
userland/nonos_app_image's builder writes them (build.rs, build_list.rs).
The list's BLAKE3 is the image's measurement: the pin a listing signs."""

import re
import struct
from typing import NamedTuple

from .image_hash import digest

IMAGE_MAGIC, LIST_MAGIC, VERSION = b"NONOSAI1", b"NONOSAC1", 1
HEADER_LEN = MEMBER_LEN = 64
LIST_HEAD = 32
MAX_MEMBERS, MAX_IMAGE, MAX_RAW = 1024, 1 << 30, 256 << 20
CHUNK_MIN, CHUNK_MAX = 64 << 10, 16 << 20
# nonos_app_image name.rs: an Alpine package name, a store name with no path.
NAME = re.compile(rb"[A-Za-z0-9][A-Za-z0-9._+-]{0,63}")
TAR, XZ = 0, 1


class Part(NamedTuple):
    name: bytes
    codec: int
    tar: bytes
    stored: bytes


def shaped(p: Part) -> bool:
    return (NAME.fullmatch(p.name) is not None and 0 < len(p.tar) <= MAX_RAW
            and len(p.tar) % 512 == 0 and len(p.stored) > 0 and p.codec in (TAR, XZ)
            and (p.codec != TAR or p.stored == p.tar))


def build(parts: list) -> bytes:
    """The image of `parts`, the listed package first; ValueError for any
    part the device's parser would refuse."""
    names = [p.name for p in parts]
    data_at = HEADER_LEN + MEMBER_LEN * len(parts) + sum(map(len, names))
    total = data_at + sum(len(p.stored) for p in parts)
    if not 1 <= len(parts) <= MAX_MEMBERS or total > MAX_IMAGE:
        raise ValueError(f"an image holds 1 to {MAX_MEMBERS} members and at most {MAX_IMAGE} bytes")
    if len(set(names)) != len(names) or not all(map(shaped, parts)):
        raise ValueError("a member is misnamed, named twice, or not a whole tar")
    head = struct.pack("<8sHHHHII Q", IMAGE_MAGIC, VERSION, 0, len(parts), 0,
                       sum(map(len, names)), 0, total).ljust(HEADER_LEN, b"\0")
    rows = b"".join(struct.pack("<QQHBB12x", len(p.stored), len(p.tar), len(p.name), p.codec,
                                int(i == 0)) + digest(p.tar) for i, p in enumerate(parts))
    return head + rows + b"".join(names) + b"".join(p.stored for p in parts)


def chunk_list(image: bytes, chunk: int) -> bytes:
    if chunk & (chunk - 1) or not CHUNK_MIN <= chunk <= CHUNK_MAX:
        raise ValueError(f"a chunk is a power of two from {CHUNK_MIN} to {CHUNK_MAX} bytes")
    if not 0 < len(image) <= MAX_IMAGE:
        raise ValueError("an image is 1 byte to MAX_IMAGE")
    head = struct.pack("<8sIIQ", LIST_MAGIC, VERSION, chunk, len(image)).ljust(LIST_HEAD, b"\0")
    return head + b"".join(digest(image[i:i + chunk]) for i in range(0, len(image), chunk))


def measurement(listing: bytes) -> bytes:
    return digest(listing)
