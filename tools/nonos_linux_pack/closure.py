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

"""Every file a program needs, resolved inside its sysroot."""

from pathlib import Path

from .dynamic import dynamic
from .elf import DEFAULT_LIBDIRS, interp, phdrs


def find(sysroot: Path, name: str, extra: list) -> Path:
    """A library by soname, inside the sysroot and nowhere else."""
    if name.startswith("/"):
        candidate = sysroot / name.lstrip("/")
        return candidate if candidate.is_file() else None
    for directory in list(extra) + list(DEFAULT_LIBDIRS):
        candidate = sysroot / directory.lstrip("/") / name
        if candidate.is_file():
            return candidate
    return None


def closure(sysroot: Path, program: Path):
    """Every file the program needs, as store path -> file on disk."""
    out = {}
    missing = []
    queue = [("/" + str(program.relative_to(sysroot)), program)]
    seen = set()
    while queue:
        store_path, disk = queue.pop()
        if store_path in seen:
            continue
        seen.add(store_path)
        data = disk.read_bytes()
        out[store_path] = disk
        headers = phdrs(data)
        ld = interp(data, headers)
        if ld:
            target = sysroot / ld.lstrip("/")
            if target.is_file():
                queue.append((ld, target))
            else:
                missing.append(ld)
        needed, runpath = dynamic(data, headers)
        for name in needed:
            found = find(sysroot, name, runpath)
            if found is None:
                missing.append(name)
                continue
            queue.append(("/" + str(found.relative_to(sysroot)), found))
    return out, missing
