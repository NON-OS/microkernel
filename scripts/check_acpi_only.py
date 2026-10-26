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
"""Hold `acpi_only!` (src/arch/acpi_capsule/gate.rs) to what it says.

The macro is the one place the kernel says which architectures are ACPI
platforms; modules that exist only there are declared inside it. So:

  1. It is defined once, in gate.rs, with two arms: the plain one gates each
     item on exactly the ACPI architectures (ACPI_ARCHES), the `else` one on
     exactly their negation. An arch added to one arm, or the arms disagreeing,
     fails.
  2. Every invocation under src/ holds only `mod` and `use` items, with
     attributes and visibility: rustfmt does not format a macro's body, so no
     code may live there.

    scripts/check_acpi_only.py [--self-test]
"""

import re
import sys
from pathlib import Path

ACPI_ARCHES = ("x86_64",)
GATE = "src/arch/acpi_capsule/gate.rs"
DEFINE = re.compile(r"macro_rules!\s*acpi_only\b")
ARM = re.compile(r"\(\s*(else\s+)?\$\(\$item:item\)\*\s*\)\s*=>\s*\{\s*\$\(\s*#\[cfg\((.*?)\)\]\s*\$item\s*\)\*\s*\};")
INVOKE = re.compile(r"acpi_only!\s*\{")
ITEM = re.compile(r"\s*(?:#\[path\s*=\s*\"[^\"]+\"\]\s*)*(?:pub(?:\([^)]*\))?\s+)?(?:mod\s+\w+|use\s+[^;{}]*(?:\{[^{}]*\})?[^;{}]*);")


def strip(text):
    return re.sub(r"//[^\n]*", "", text)


def predicate(arches):
    one = [f'target_arch = "{a}"' for a in arches]
    return one[0] if len(one) == 1 else f"any({', '.join(one)})"


def check_gate(text):
    body = strip(text)
    arms = {bool(m.group(1)): re.sub(r"\s+", " ", m.group(2)).strip() for m in ARM.finditer(body)}
    want = predicate(ACPI_ARCHES)
    failed = []
    if len(ARM.findall(body)) != 2 or set(arms) != {True, False}:
        failed.append(f"{GATE}: acpi_only! is not exactly an `else` arm and a plain arm")
    elif arms[False] != want:
        failed.append(f"{GATE}: the plain arm gates on `{arms[False]}`, not the ACPI arches `{want}`")
    elif arms[True] != f"not({want})":
        failed.append(f"{GATE}: the `else` arm is `{arms[True]}`, not `not({want})`")
    return failed


def bodies(text):
    """The text inside each `acpi_only! { ... }`, braces balanced."""
    for m in INVOKE.finditer(text):
        depth, i = 1, m.end()
        while depth and i < len(text):
            depth += {"{": 1, "}": -1}.get(text[i], 0)
            i += 1
        yield text[m.end():i - 1]


def check_uses(files):
    failed = []
    for path, text in files:
        body = strip(text)
        if DEFINE.search(body) and path != GATE:
            failed.append(f"{path}: a second definition of acpi_only!")
        if path == GATE:
            continue
        for inner in bodies(body):
            rest = re.sub(r"^\s*else\b", "", inner)
            if ITEM.sub("", rest).strip():
                failed.append(f"{path}: acpi_only! holds more than mod and use items: {inner.strip()[:60]!r}")
    return failed


def check(files):
    gate = dict(files).get(GATE)
    return (check_gate(gate) if gate else [f"{GATE} is missing"]) + check_uses(files)


def tree(root):
    for p in sorted((root / "src").rglob("*.rs")):
        yield p.relative_to(root).as_posix(), p.read_text()


def self_test(files):
    files = list(files)
    gate = dict(files)[GATE]
    def with_gate(text):
        return [(p, text if p == GATE else t) for p, t in files]
    plain = 'cfg(target_arch = "x86_64")] $item'
    cases = [
        (files, True),
        (with_gate(gate.replace(plain, 'cfg(any(target_arch = "x86_64", target_arch = "aarch64"))] $item')), False),
        (with_gate(gate.replace('not(target_arch = "x86_64")', 'not(target_arch = "aarch64")')), False),
        (with_gate(gate.replace("(else $($item:item)*)", "(other $($item:item)*)")), False),
        (files + [("src/x.rs", "crate::arch::acpi_only! { fn f() {} }")], False),
        (files + [("src/x.rs", "crate::arch::acpi_only! { mod a; const C: u8 = 1; }")], False),
        (files + [("src/x.rs", "macro_rules! acpi_only { () => {} }")], False),
        (files + [("src/x.rs", 'crate::arch::acpi_only! { else #[path = "a.rs"] pub(crate) mod a; pub use a::{b, c}; }')], True),
    ]
    wrong = [i for i, (fs, ok) in enumerate(cases) if (not check(fs)) != ok]
    print(f"acpi-only: self-test {'failed on ' + str(wrong) if wrong else 'passed'}, "
          f"{sum(not ok for _, ok in cases)} mutants rejected")
    return 1 if wrong else 0


def main():
    files = list(tree(Path(".")))
    if "--self-test" in sys.argv[1:]:
        return self_test(files)
    failed = check(files)
    for line in failed:
        print(f"acpi-only: {line}")
    n = sum(len(list(bodies(strip(t)))) for p, t in files if p != GATE)
    if not failed:
        print(f"acpi-only: the gate names {', '.join(ACPI_ARCHES)} alone; {n} invocations hold only mod and use")
    return 1 if failed else 0


if __name__ == "__main__":
    sys.exit(main())
