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

"""Pack: a program's closure into the store, or the entries for it."""

import subprocess
import sys
from pathlib import Path

from .args import parser
from .closure import closure


def main(argv) -> int:
    args = parser().parse_args(argv)

    if not args.sysroot.is_dir():
        print(f"linux-pack: no sysroot at {args.sysroot}", file=sys.stderr)
        return 2

    entries, missing = {}, []
    for program in args.program:
        path = program if program.is_absolute() else args.sysroot / program
        if not path.is_file():
            print(f"linux-pack: no program at {path}", file=sys.stderr)
            return 2
        found, gone = closure(args.sysroot, path.resolve())
        entries.update(found)
        missing.extend(gone)

    for item in args.data:
        store, _, disk = item.partition("=")
        path = Path(disk) if disk else (args.sysroot / store.lstrip("/"))
        if not path.is_file():
            print(f"linux-pack: no data file at {path}", file=sys.stderr)
            return 2
        entries["/" + store.lstrip("/")] = path

    if missing:
        print("linux-pack: unresolved inside the sysroot, refusing to pack a "
              "program that cannot start:", file=sys.stderr)
        for name in sorted(set(missing)):
            print(f"  {name}", file=sys.stderr)
        return 1

    pairs = [f"{store}={disk}" for store, disk in sorted(entries.items())]
    total = sum(disk.stat().st_size for disk in entries.values())
    print(f"linux-pack: {len(pairs)} files, {total // 1024} KiB")
    if args.image is None:
        for pair in pairs:
            print(f"  --entry {pair}")
        return 0

    cmd = [sys.executable, str(args.pack), "--image", str(args.image),
           "--lba", str(args.lba)]
    for pair in pairs:
        cmd += ["--entry", pair]
    return subprocess.call(cmd)
