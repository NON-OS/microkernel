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
"""No kernel serial line is written in pieces.

A line built from several print calls is several writes with gaps between,
and on more than one CPU another CPU's line lands in a gap
("[INIT] [USER-FIRST] pid=0x28 rip=0x2BB799E0Capsules spawned"). Outside
src/sys/serial a line is a Line (built whole, written once), or one byte
literal that ends its own line, or a process's own bytes (passthrough).
"""

import argparse
import re
import sys
from pathlib import Path

# Files that may still print in pieces, and why. Empty: none may.
ALLOWED: dict = {}

PIECE = re.compile(r"serial::(print_hex|print_dec|print_str|print_u64)\s*\(")
PRINT = re.compile(r"serial::print\s*\(")
WHOLE = re.compile(r'\s*b"((?:[^"\\]|\\.)*)"\s*,?\s*\)')
IMPORT = re.compile(r"use\s+(?:crate::sys|super)::serial::(?:\{[^}]*\b(?:print|print_hex|print_dec|print_str|print_u64)\b[^}]*\}|(?:print|print_hex|print_dec|print_str|print_u64)\b|\*)")
ALIAS = re.compile(r"serial::print\w*\s+as\b|=\s*(?:crate::sys::)?serial::print\w*\s*;")


def problems(sources: dict) -> list:
    out = []
    for path, text in sources.items():
        if path.startswith("src/sys/serial/") or path in ALLOWED:
            continue
        for m in PIECE.finditer(text):
            out.append(f"{path}: {m.group(1)} prints a piece of a line")
        for m in PRINT.finditer(text):
            lit = WHOLE.match(text, m.end())
            if not (lit and lit.group(1).endswith("\\n")):
                out.append(f"{path}: print of something other than a whole line")
        for rule in (IMPORT, ALIAS):
            if rule.search(text):
                out.append(f"{path}: a print function imported or aliased")
    out += [f"{p}: allowed but no longer prints in pieces" for p in ALLOWED if p not in sources]
    return out


A = "src/kernel_core/init/platform/baseline.rs"
CASES = [("as built", lambda s: s, 0),
         ("a piece", lambda s: {**s, A: s[A] + '\nfn x() { crate::sys::serial::print(b"[X] "); }'}, 1),
         ("a hex piece", lambda s: {**s, A: s[A] + "\nfn x() { crate::sys::serial::print_hex(1); }"}, 1),
         ("an import", lambda s: {**s, A: s[A] + "\nuse crate::sys::serial::{print_dec, println};"}, 1),
         ("an alias", lambda s: {**s, A: s[A] + "\nfn x() { let p = crate::sys::serial::print; }"}, 1),
         ("a whole literal line", lambda s: {**s, A: s[A] + '\nfn x() { crate::sys::serial::print(b"[X] ok\\n"); }'}, 0)]


def main() -> int:
    ap = argparse.ArgumentParser(description=__doc__.split("\n")[0])
    ap.add_argument("--self-test", action="store_true")
    a = ap.parse_args()
    sources = {str(p): p.read_text() for p in sorted(Path("src").rglob("*.rs"))}
    if a.self_test:
        bad = [w for w, f, n in CASES if len(problems(f(sources))) != n]
        print(f"serial-whole-lines self-test: {len(CASES)} cases, {len(bad)} wrong {bad if bad else ''}")
        return 1 if bad else 0
    found = problems(sources)
    for p in found:
        print(f"serial-whole-lines: {p}")
    print(f"serial-whole-lines: {len(found)} problems")
    return 1 if found else 0


if __name__ == "__main__":
    sys.exit(main())
