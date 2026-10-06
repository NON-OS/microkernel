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
"""The ceremony's verbs, as tools/nonos-key-ceremony dispatches them."""

import os
import stat
import sys

from . import backup, catalog, guard, make, manifest


def refuse_unsafe(root, keys):
    problems = guard.check(root, keys)
    if problems:
        sys.exit("refusing: git would keep private halves\n  " + "\n  ".join(problems))


def run(args):
    root, keys = args.root, catalog.every_key(args.root)
    if args.cmd == "plan":
        for k in keys:
            print(f"{k.name:40} {k.kind:10} {k.private}  ->  {k.public}")
    elif args.cmd == "check":
        refuse_unsafe(root, keys)
        for k in keys:
            p = os.path.join(root, k.private)
            mode = stat.S_IMODE(os.stat(p).st_mode) if os.path.exists(p) else None
            state = "missing" if mode is None else ("ok" if mode == 0o600 else f"mode {mode:o}, want 600")
            print(f"{state:22} {k.private}")
    elif args.cmd == "make":
        refuse_unsafe(root, keys)
        todo = guard.missing(root, keys)
        for k in todo:
            make.make(root, k, args.capsule_sign)
            print(f"made {k.name}: public half {k.public}")
        print(f"{len(todo)} made, {len(keys) - len(todo)} already present and left as they were")
    elif args.cmd == "manifest":
        print(f"{manifest.write(root, keys)} keys fingerprinted in {manifest.MANIFEST}")
    elif args.cmd == "backup":
        backup.backup(root, args.out)
        print(f"wrote {args.out}")
    elif args.cmd == "restore":
        refuse_unsafe(root, keys)
        backup.restore(root, args.src)
        print("restored")
