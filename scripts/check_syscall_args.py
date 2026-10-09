#!/usr/bin/env python3
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
"""Check that every published syscall's arguments are the ones the kernel reads.

`abi/syscalls.toml` lists each call's arguments, and a toolchain built from it
puts them in argument registers in that order. The kernel reads exactly the
registers its dispatch arm passes, `SYS_X => sys_x(a.a0, a.a1, ...)` under
`src/syscall/microkernel/dispatch/`. A description with another count puts a
pointer where the kernel reads a length (docs/FINDINGS.md F1).

An arm may also only pick a handler, `SYS_X => sys_x,`, from a table of
handlers that share one signature; then the registers are the ones the shared
call after the match reads. For every published call whose arm has either
shape, the description must name as many arguments as the arm reads, and the
arm must read a0 upward with no gap. Calls dispatched another way are counted
and left to their own checks.
"""

import argparse
import re
import sys
import tomllib
from pathlib import Path

NUMBER = re.compile(r'pub const (SYS_[A-Z0-9_]+): u64 = tag4\(b"([A-Z0-9]{4})"\);')
ARM = re.compile(r"\b(SYS_[A-Z0-9_]+)\s*=>")
REGISTER = re.compile(r"\ba\.a([0-5])\b")
# An arm that only names its handler, from a table the match builds.
PICK = re.compile(r"^\s*=>\s*[a-z_][a-z0-9_]*\s*,\s*$")


def arms(root):
    """Each dispatch arm's constant and the argument registers it reads."""
    found = {}
    for path in sorted((root / "src/syscall/microkernel/dispatch").glob("*.rs")):
        text = path.read_text()
        starts = [(m.start(), m.end(), m.group(1)) for m in ARM.finditer(text)]
        for i, (at, after, name) in enumerate(starts):
            end = starts[i + 1][0] if i + 1 < len(starts) else len(text)
            body = text[at:end].split("_ =>")[0]
            regs = {int(r) for r in REGISTER.findall(body)}
            if not regs and PICK.match("=>" + text[after:end].split("\n")[0]):
                # The shared call: what follows the match's catch-all arm, to
                # the end of the function.
                catch_all = text.find("_ =>", at)
                rest = text[catch_all:].split("\n}\n", 1)[0] if catch_all >= 0 else ""
                regs = {int(r) for r in REGISTER.findall(rest)}
            found[name] = sorted(regs)
    return found


def check(root):
    published = tomllib.loads((root / "abi/syscalls.toml").read_text()).get("desc", {})
    numbers = root / "src/syscall/microkernel/numbers.rs"
    name_of = {tag: name for name, tag in NUMBER.findall(numbers.read_text())}
    read = arms(root)
    problems, checked, elsewhere = [], 0, 0
    for tag, desc in sorted(published.items()):
        regs = read.get(name_of.get(tag, ""))
        if regs is None:
            elsewhere += 1
            continue
        checked += 1
        if regs != list(range(len(regs))):
            problems.append(f"{tag}: the kernel reads registers {regs}, with a gap")
        if len(desc.get("args", [])) != len(regs):
            problems.append(f"{tag}: published {len(desc.get('args', []))} arguments, "
                            f"the kernel reads {len(regs)}")
    return problems, checked, elsewhere


def main():
    ap = argparse.ArgumentParser(description=__doc__.split("\n", 1)[0])
    ap.add_argument("--root", default=".", help="the repository root")
    args = ap.parse_args()
    problems, checked, elsewhere = check(Path(args.root))
    for p in problems:
        print(f"syscall-args: {p}", file=sys.stderr)
    print(f"syscall-args: {checked} calls checked against their dispatch arm, "
          f"{elsewhere} dispatched another way, {len(problems)} problems")
    return 1 if problems else 0


if __name__ == "__main__":
    sys.exit(main())
