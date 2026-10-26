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
"""Only the broker turns an MSI or MSI-X message on (irq/bind, through remapping).

The compat builders in src/drivers/pci wrote a message naming the local APIC
straight into a function's capability, with no remapping entry; they had no
caller and are gone. None of them may come back under src/drivers/pci.
"""

import argparse
import re
import sys
from pathlib import Path

ROOT = "src/drivers/pci"
GONE = re.compile(r"\bfn (configure_msi|configure_msi_multi|configure_msix|configure_msix_single)\b"
                  r"|\bstruct MsiController\b")


def problems(files: dict) -> list:
    return [f"{p}: {m.group(0)} is back" for p, t in sorted(files.items()) for m in GONE.finditer(t)]


def main() -> int:
    ap = argparse.ArgumentParser(description=__doc__.split("\n")[0])
    ap.add_argument("--self-test", action="store_true")
    a = ap.parse_args()
    files = {str(p): p.read_text() for p in Path(ROOT).rglob("*.rs")}
    if a.self_test:
        add = lambda f, s: {**files, f: files.get(f, "") + s}
        cases = [("as built", files, 0),
                 ("configure_msi back", add(f"{ROOT}/msi/core.rs", "\npub fn configure_msi(\n"), 1),
                 ("configure_msix back", add(f"{ROOT}/types/device_msi.rs", "\n    pub fn configure_msix(\n"), 1),
                 ("MsiController back", add(f"{ROOT}/msi/controller.rs", "\npub struct MsiController<'a> {}\n"), 1),
                 ("disable_msi stays", add(f"{ROOT}/msi/core.rs", "\npub fn disable_msi(\n"), 0)]
        bad = [w for w, f, n in cases if len(problems(f)) != n]
        print(f"msi-builders self-test: {len(cases)} cases, {len(bad)} wrong {bad if bad else ''}")
        return 1 if bad else 0
    found = problems(files)
    for p in found:
        print(f"msi-builders: {p}")
    print(f"msi-builders: {len(found)} problems")
    return 1 if found else 0


if __name__ == "__main__":
    sys.exit(main())
