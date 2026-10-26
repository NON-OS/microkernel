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
"""How long a package's pack is (userland/nonos_linux_pack, build.rs), so
a package whose pack would pass the format's bounds is refused when its
listing is built, never cut at install. It counts every regular file the
installer may pack, the last copy of a name, and the trailer it mints
beside each ELF. A held market program is written as a file of its own
instead, so the count is never short of the pack the device writes."""

from . import tarwalk

# userland/nonos_linux_pack/src/format.rs
MAX_PACK = 64 << 20
MAX_ENTRIES = 65536
HEADER_LEN, ENTRY_LEN, DATA_ALIGN, FILE_ALIGN = 64, 48, 4096, 64
# src/security/local_build/trailer.rs TRAILER_LEN, and install/market/minted.rs TRAILER.
TRAILER_LEN = 8 + 32 + 32
TRAILER = b".zk_trailer.bin"


def up(n: int, to: int) -> int:
    return -(-n // to) * to


def sizes(files: bytes) -> dict:
    """Every name the pack holds and its length, as install/place/packed.rs
    gathers them: names the installer passes over (a leading `.`) are not."""
    kept = {e.name: e.body for e in tarwalk.files(files) if not e.name.startswith(b".")}
    out = {name: len(body) for name, body in kept.items()}
    out.update({n + TRAILER: TRAILER_LEN for n, b in kept.items() if b.startswith(b"\x7fELF")})
    return out


def pack_len(files: bytes) -> int:
    held = sizes(files)
    names = sorted(held)
    end = up(HEADER_LEN + ENTRY_LEN * len(names) + sum(map(len, names)), DATA_ALIGN)
    for name in names:
        if held[name]:
            end = up(end, FILE_ALIGN) + held[name]
    return end


def check(files: bytes) -> None:
    """ValueError when the package's files do not make one pack."""
    if len(sizes(files)) > MAX_ENTRIES:
        raise ValueError(f"its pack would hold more than {MAX_ENTRIES} files")
    n = pack_len(files)
    if n > MAX_PACK:
        raise ValueError(f"its pack would be {n} bytes, past the {MAX_PACK >> 20} MiB a pack holds")
