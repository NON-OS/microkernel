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
"""The syscalls the Linux personality serves, and the ones x86_64 defines.

--self-test builds a personality whose table sits in a family directory
(serve/table/), as capsule_linux's does, and fails unless its calls count.
"""

import argparse
import re
import sys
import tempfile
from pathlib import Path

LINUX = Path("userland/capsule_linux")
CONST = re.compile(r"pub const ([A-Z0-9_]+): u64 = (\d+);")
# A number module named as itself (nr::), or under the alias a file gives it
# (nr_path as np, nr_sig as ns): every alias a file declares is followed, so
# a call routed through one is never missed.
ALIAS = re.compile(r"\bnr(?:_[a-z]+)? as ([a-z_]+)\b")


def uses(text):
    names = {"nr"} | set(ALIAS.findall(text))
    use = re.compile(r"\b(?:%s)::([A-Z0-9_]+)" % "|".join(sorted(names)))
    return set(use.findall(text))


def served(root):
    numbers = {}
    for p in (root / LINUX / "src/linux/abi").glob("nr*.rs"):
        numbers.update({k: int(v) for k, v in CONST.findall(p.read_text())})
    used = set()
    # Every depth: the table lives in serve/table/, and a file moved into a
    # family directory still serves what it served.
    for p in (root / LINUX / "src/linux/serve").rglob("*.rs"):
        used |= uses(p.read_text())
    return {numbers[u] for u in used if u in numbers}


def defined(root):
    out = {}
    for line in (root / LINUX / "abi/x86_64-syscalls.txt").read_text().splitlines():
        if line and not line.startswith("#"):
            n, name = line.split()
            out[int(n)] = name
    return out


FIXTURE = {
    "abi/x86_64-syscalls.txt": "# n name\n0 read\n1 write\n2 open\n",
    "src/linux/abi/nr.rs": "pub const READ: u64 = 0;\n",
    "src/linux/abi/nr_file.rs": "pub const WRITE: u64 = 1;\npub const OPEN: u64 = 2;\n",
    "src/linux/serve/dispatch.rs": "match n { nr::READ => read(), _ => table::answer(n) }\n",
    "src/linux/serve/table/file.rs": "use super::nr_file as nf;\nmatch n { nf::WRITE => write() }\n",
}


def self_test():
    with tempfile.TemporaryDirectory() as d:
        root = Path(d)
        for rel, text in FIXTURE.items():
            path = root / LINUX / rel
            path.parent.mkdir(parents=True, exist_ok=True)
            path.write_text(text)
        got = served(root)
        names = sorted(defined(root)[n] for n in got)
    if names != ["read", "write"]:
        print(f"linux_calls: self-test failed, served {names}, want read and write")
        return 1
    print("linux_calls: self-test passed, a call served from serve/table/ was counted")
    return 0


if __name__ == "__main__":
    ap = argparse.ArgumentParser(description=__doc__.split("\n", 1)[0])
    ap.add_argument("--self-test", action="store_true", help="prove a nested table is read")
    if not ap.parse_args().self_test:
        ap.error("this module is read by nonos-linux-coverage and disclosure.py; run --self-test")
    sys.exit(self_test())
