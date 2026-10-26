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
"""An x86_64 kernel without nonos-arch-iommu cannot be built.

The platform init reaches the IOMMU unconditionally
(src/kernel_core/init/platform/hardware_broker.rs), so the combination is made
unrepresentable rather than made to build:

  1. In Cargo.toml, arch-x86_64 implies nonos-arch-iommu.
  2. Every feature set that reaches `kernel`, other than the bootstrap atoms
     `kernel` and `standalone`, resolves to nonos-arch-iommu.
  3. src/arch/x86_64/mod.rs refuses to compile without it (compile_error!),
     for a hand-picked --no-default-features set no profile names.

    scripts/check_x86_iommu_feature.py [--self-test]
"""

import re
import sys
from pathlib import Path

ATOMS = {"kernel", "standalone"}
GUARD = re.compile(r'#\[cfg\(not\(feature = "nonos-arch-iommu"\)\)\]\s*compile_error!\(')


def features(text):
    m = re.search(r"^\[features\]\s*$(.*?)(?=^\[|\Z)", text, re.M | re.S)
    body = re.sub(r"#[^\n]*", "", m.group(1)) if m else ""
    found = re.finditer(r"^\s*([A-Za-z0-9_-]+)\s*=\s*\[(.*?)\]", body, re.M | re.S)
    return {f.group(1): re.findall(r'"([^"/]+)"', f.group(2)) for f in found}


def closure(feats, name):
    seen, todo = set(), [name]
    while todo:
        n = todo.pop()
        if n not in seen:
            seen.add(n)
            todo.extend(d for d in feats.get(n, []) if not d.startswith("dep:"))
    return seen


def check(cargo, arch_mod):
    feats = features(cargo)
    failed = []
    if "nonos-arch-iommu" not in closure(feats, "arch-x86_64"):
        failed.append("Cargo.toml: arch-x86_64 does not imply nonos-arch-iommu")
    for name in sorted(feats):
        reach = closure(feats, name)
        if name not in ATOMS and "kernel" in reach and "nonos-arch-iommu" not in reach:
            failed.append(f"Cargo.toml: {name} builds a kernel without nonos-arch-iommu")
    if not GUARD.search(arch_mod):
        failed.append("src/arch/x86_64/mod.rs: no compile_error! without nonos-arch-iommu")
    return failed


def self_test(cargo, arch_mod):
    cases = [
        ((cargo, arch_mod), True),
        ((cargo.replace('arch-x86_64 = ["nonos-arch-iommu"]', "arch-x86_64 = []"), arch_mod), False),
        ((cargo.replace('arch-x86_64 = ["nonos-arch-iommu"]', "arch-x86_64 = []")
          .replace('  "nonos-arch-iommu",\n  "nonos-iommu-enforce",', '  "nonos-iommu-enforce",')
          .replace('nonos-iommu-enforce  = ["nonos-arch-iommu"]', "nonos-iommu-enforce  = []")
          .replace('nonos-iommu-amdvi = ["nonos-arch-iommu"]', "nonos-iommu-amdvi = []"), arch_mod), False),
        ((cargo.replace("[features]\n", '[features]\nbare-x86 = ["kernel", "standalone"]\n', 1), arch_mod), False),
        ((cargo, arch_mod.replace("compile_error!(", "// compile_error!(")), False),
    ]
    wrong = [i for i, ((c, a), ok) in enumerate(cases) if (not check(c, a)) != ok]
    print(f"x86-iommu-feature: self-test {'failed on ' + str(wrong) if wrong else 'passed'}, "
          f"{sum(not ok for _, ok in cases)} mutants rejected")
    return 1 if wrong else 0


def main():
    cargo = Path("Cargo.toml").read_text()
    arch_mod = Path("src/arch/x86_64/mod.rs").read_text()
    if "--self-test" in sys.argv[1:]:
        return self_test(cargo, arch_mod)
    failed = check(cargo, arch_mod)
    for line in failed:
        print(f"x86-iommu-feature: {line}")
    if not failed:
        print("x86-iommu-feature: no x86_64 kernel feature set lacks nonos-arch-iommu")
    return 1 if failed else 0


if __name__ == "__main__":
    sys.exit(main())
