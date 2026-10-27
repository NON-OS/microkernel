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
beside each ELF. A listed package's programs are held until its policy is
bound, each a file of its own beside the pack (install/place/packed.rs,
`held && elf`), so check() leaves its ELFs out of the pack and holds each
to the store's largest file instead."""

from . import tarwalk

# userland/nonos_linux_pack/src/format.rs
MAX_PACK = 64 << 20
MAX_ENTRIES = 65536
# userland/capsule_vfs types.rs MAX_FILE_BYTES: a held program is one file.
MAX_FILE = 1 << 27
HEADER_LEN, ENTRY_LEN, DATA_ALIGN, FILE_ALIGN = 64, 48, 4096, 64
# src/security/local_build/trailer.rs TRAILER_LEN, and install/market/minted.rs TRAILER.
TRAILER_LEN = 8 + 32 + 32
TRAILER = b".zk_trailer.bin"


def up(n: int, to: int) -> int:
    return -(-n // to) * to


def sizes(files: bytes, held: bool = False) -> dict:
    """Every name the pack holds and its length, as install/place/packed.rs
    gathers them: names the installer passes over (a leading `.`) are not,
    nor, when `held`, the programs it writes beside the pack."""
    kept = {e.name: e.body for e in tarwalk.files(files) if not e.name.startswith(b".")}
    kept = {n: b for n, b in kept.items() if not (held and b.startswith(b"\x7fELF"))}
    out = {name: len(body) for name, body in kept.items()}
    out.update({n + TRAILER: TRAILER_LEN for n, b in kept.items() if b.startswith(b"\x7fELF")})
    return out


def pack_len(files: bytes, held: bool = False) -> int:
    kept = sizes(files, held)
    names = sorted(kept)
    end = up(HEADER_LEN + ENTRY_LEN * len(names) + sum(map(len, names)), DATA_ALIGN)
    for name in names:
        if kept[name]:
            end = up(end, FILE_ALIGN) + kept[name]
    return end


def check(files: bytes) -> None:
    """ValueError when a listed package's files do not make one pack and
    held programs the store takes."""
    big = [e.name for e in tarwalk.files(files) if e.body.startswith(b"\x7fELF") and len(e.body) > MAX_FILE]
    if big:
        raise ValueError(f"its program {big[0].decode(errors='replace')} is past the {MAX_FILE >> 20} MiB a file holds")
    if len(sizes(files, True)) > MAX_ENTRIES:
        raise ValueError(f"its pack would hold more than {MAX_ENTRIES} files")
    n = pack_len(files, True)
    if n > MAX_PACK:
        raise ValueError(f"its pack would be {n} bytes, past the {MAX_PACK >> 20} MiB a pack holds")
