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

"""Check that loader and kernel agree on every shared handoff struct and constant."""

import argparse
import pathlib
import re
import sys

ROOT = pathlib.Path(__file__).resolve().parent.parent
STRUCT = re.compile(r"#\[repr\(C[^\]]*\)\][^{]*?pub struct (\w+)\s*\{(.*?)\n\}", re.S)
FIELD = re.compile(r"pub(?:\([^)]*\))?\s+(\w+)\s*:\s*([^,]+),")
CONST = re.compile(r"pub const (\w+)\s*:\s*\w+\s*=\s*([^;]+);")


def read(root):
    structs, consts = {}, {}
    for p in sorted(pathlib.Path(root).rglob("*.rs")):
        text = re.sub(r"//[^\n]*|/\*.*?\*/", "", p.read_text(), flags=re.S)
        for m in STRUCT.finditer(text):
            structs[m.group(1)] = [(f, re.sub(r"\s", "", t)) for f, t in FIELD.findall(m.group(2))]
        for name, value in CONST.findall(text):
            consts[name] = re.sub(r"[\s_]", "", value)
    return structs, consts


def problems(kernel, loader):
    (ks, kc), (ls, lc) = read(kernel), read(loader)
    for kind, k, l in (("struct", ks, ls), ("const", kc, lc)):
        for name in sorted(set(k) & set(l)):
            if k[name] != l[name]:
                yield f"{kind} {name}: kernel {k[name]} != loader {l[name]}"


def self_test():
    import tempfile
    good = "#[repr(C)]\npub struct H {\n pub a: u32,\n pub b: u64,\n}\npub const V: u16 = 2;\n"
    swap = good.replace("pub a: u32,\n pub b: u64,", "pub b: u64,\n pub a: u32,")
    for other, bites in ((good, False), (swap, True), (good.replace("= 2", "= 3"), True)):
        with tempfile.TemporaryDirectory() as d:
            trees = [pathlib.Path(d, n) for n in "kl"]
            for tree, text in zip(trees, (good, other)):
                tree.mkdir()
                (tree / "t.rs").write_text(text)
            assert bool(list(problems(*trees))) == bites
    print("handoff mirror: self-test PASS")


def main():
    ap = argparse.ArgumentParser(description=__doc__.split("\n", 1)[0])
    ap.add_argument("--self-test", action="store_true", help="prove the check bites")
    if ap.parse_args().self_test:
        return self_test() or 0
    found = list(problems(ROOT / "src/boot/handoff", ROOT / "nonos-bootloader/src/handoff"))
    print("".join(f"handoff mirror: {p}\n" for p in found) + f"handoff mirror: {len(found)} differ")
    return 1 if found else 0


if __name__ == "__main__":
    sys.exit(main())
