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
"""Hold the platform gates (src/arch/acpi_capsule/gate.rs) to what they say.

`acpi_only!` is the one place the kernel says which architectures are ACPI
platforms, `msi_only!` which bind PCI message interrupts; modules that exist
only there are declared inside them. So:

  1. Each is defined once, in gate.rs, with two arms: the plain one gates each
     item on exactly its arches (GATES), the `else` one on exactly their
     negation. An arch added to an arm, or the arms disagreeing, fails.
  2. The ACPI arches are MSI arches: the ACPI broker's late writer keeps the
     MSI side table (pci_index/late.rs).
  3. Every invocation under src/ holds only `mod` and `use` items, with
     attributes and visibility: rustfmt does not format a macro's body, so no
     code may live there.

    scripts/check_platform_gates.py [--self-test]
"""

import re
import sys
from pathlib import Path

GATES = {"acpi_only": ("x86_64",), "msi_only": ("x86_64", "aarch64")}
GATE = "src/arch/acpi_capsule/gate.rs"
ITEM = re.compile(r"\s*(?:#\[path\s*=\s*\"[^\"]+\"\]\s*)*(?:pub(?:\([^)]*\))?\s+)?(?:mod\s+\w+|use\s+[^;{}]*(?:\{[^{}]*\})?[^;{}]*);")


def strip(text):
    return re.sub(r"//[^\n]*", "", text)


def predicate(arches):
    one = [f'target_arch = "{a}"' for a in arches]
    return one[0] if len(one) == 1 else f"any({', '.join(one)})"


def arms(body, name):
    m = re.search(rf"macro_rules!\s*{name}\s*\{{(.*?)\n\}}", body, re.S)
    if not m:
        return None
    arm = re.compile(r"\(\s*(else\s+)?\$\(\$item:item\)\*\s*\)\s*=>\s*\{\s*\$\(\s*#\[cfg\((.*?)\)\]\s*\$item\s*\)\*\s*\};", re.S)
    found = [(bool(a.group(1)), re.sub(r"\s+", " ", a.group(2)).strip()) for a in arm.finditer(m.group(1))]
    return dict(found) if len(found) == 2 and len(dict(found)) == 2 else {}


def check_gate(text):
    body, failed = strip(text), []
    for name, arches in GATES.items():
        got, want = arms(body, name), predicate(arches)
        if got is None:
            failed.append(f"{GATE}: {name}! is not defined")
        elif set(got) != {True, False}:
            failed.append(f"{GATE}: {name}! is not exactly an `else` arm and a plain arm")
        elif got[False] != want:
            failed.append(f"{GATE}: {name}!'s plain arm gates on `{got[False]}`, not `{want}`")
        elif got[True] != f"not({want})":
            failed.append(f"{GATE}: {name}!'s `else` arm is `{got[True]}`, not `not({want})`")
    if not set(GATES["acpi_only"]) <= set(GATES["msi_only"]):
        failed.append("an ACPI arch is not an MSI arch")
    return failed


def bodies(text, name):
    """The text inside each `name! { ... }`, braces balanced."""
    for m in re.finditer(rf"{name}!\s*\{{", text):
        depth, i = 1, m.end()
        while depth and i < len(text):
            depth += {"{": 1, "}": -1}.get(text[i], 0)
            i += 1
        yield text[m.end():i - 1]


def check_uses(files):
    failed = []
    for path, text in files:
        body = strip(text)
        for name in GATES:
            if re.search(rf"macro_rules!\s*{name}\b", body) and path != GATE:
                failed.append(f"{path}: a second definition of {name}!")
            if path == GATE:
                continue
            for inner in bodies(body, name):
                if ITEM.sub("", re.sub(r"^\s*else\b", "", inner)).strip():
                    failed.append(f"{path}: {name}! holds more than mod and use items: {inner.strip()[:60]!r}")
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
    acpi = 'cfg(target_arch = "x86_64")] $item'
    msi = 'cfg(any(target_arch = "x86_64", target_arch = "aarch64"))] $item'
    cases = [
        (files, True),
        (with_gate(gate.replace(acpi, 'cfg(any(target_arch = "x86_64", target_arch = "aarch64"))] $item')), False),
        (with_gate(gate.replace('not(target_arch = "x86_64")', 'not(target_arch = "aarch64")')), False),
        (with_gate(gate.replace(msi, 'cfg(any(target_arch = "x86_64", target_arch = "aarch64", target_arch = "riscv64"))] $item')), False),
        (with_gate(gate.replace(msi, 'cfg(target_arch = "x86_64")] $item')), False),
        (with_gate(gate.replace("macro_rules! msi_only", "macro_rules! msi_gate")), False),
        (with_gate(gate.replace("(else $($item:item)*)", "(other $($item:item)*)", 1)), False),
        (files + [("src/x.rs", "crate::arch::acpi_only! { fn f() {} }")], False),
        (files + [("src/x.rs", "crate::arch::msi_only! { mod a; const C: u8 = 1; }")], False),
        (files + [("src/x.rs", "macro_rules! acpi_only { () => {} }")], False),
        (files + [("src/x.rs", 'crate::arch::msi_only! { else #[path = "a.rs"] pub(crate) mod a; pub use a::{b, c}; }')], True),
    ]
    wrong = [i for i, (fs, ok) in enumerate(cases) if (not check(fs)) != ok]
    print(f"platform-gates: self-test {'failed on ' + str(wrong) if wrong else 'passed'}, "
          f"{sum(not ok for _, ok in cases)} mutants rejected")
    return 1 if wrong else 0


def main():
    files = list(tree(Path(".")))
    if "--self-test" in sys.argv[1:]:
        return self_test(files)
    failed = check(files)
    for line in failed:
        print(f"platform-gates: {line}")
    if not failed:
        n = {g: sum(len(list(bodies(strip(t), g))) for p, t in files if p != GATE) for g in GATES}
        print(f"platform-gates: acpi_only! names x86_64, msi_only! x86_64 and aarch64; "
              f"{n['acpi_only']} and {n['msi_only']} invocations hold only mod and use")
    return 1 if failed else 0


if __name__ == "__main__":
    sys.exit(main())
