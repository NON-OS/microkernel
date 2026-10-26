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
"""Hold the kernel's log lines to redacted addresses: with the serial echo
(src/log/backend/serial.rs) and the log viewer reading the RAM log, a line
that formats an instruction, stack, faulting or guest address raw leaks the
kernel's layout (KASLR). Every log call under src/ is read with its
arguments; an address field not inside redact_address fails."""

import argparse
import re
import sys
import tempfile
from pathlib import Path

CALL = re.compile(r"\b(?:warn|error|log_\w+|security_log)!\s*\(|\blog_critical\s*\(")
FIELD = re.compile(r"\.(instruction_pointer|stack_pointer|accessed_address|guest_\w+_address)\b")


def raw_sites(text: str):
    for m in CALL.finditer(text):
        i, depth = m.end(), 1
        while i < len(text) and depth:
            depth += {"(": 1, ")": -1}.get(text[i], 0)
            i += 1
        args = text[m.end():i]
        for f in FIELD.finditer(args):
            # The field's whole path (`ctx.exception.`) is dropped first.
            before = re.sub(r"[\w.]*$", "", args[:f.start()]).rstrip()
            if not before.endswith("redact_address("):
                yield text.count("\n", 0, f.start() + m.end()) + 1, f.group(1)


def check(root: Path) -> int:
    problems = []
    for path in sorted((root / "src").rglob("*.rs")):
        for line, field in raw_sites(path.read_text(errors="replace")):
            problems.append(f"{path.relative_to(root)}:{line}: {field} formatted raw")
    for p in problems:
        print(f"redacted-addresses: {p}")
    print(f"redacted-addresses: {len(problems)} raw addresses in kernel warnings")
    return 1 if problems else 0


def self_test() -> int:
    raw = 'fn f(c: &C) { crate::log::warn!("at {:#x}", c.instruction_pointer); }\n'
    ok = 'fn f(c: &C) { crate::log::warn!("at {}", redact_address(c.instruction_pointer)); }\n'
    with tempfile.TemporaryDirectory() as d:
        (Path(d) / "src").mkdir()
        bites = (Path(d) / "src/a.rs").write_text(raw) and check(Path(d)) == 1
        passes = (Path(d) / "src/a.rs").write_text(ok) and check(Path(d)) == 0
    print(f"redacted-addresses self-test: caught {bites}, passed {passes}")
    return 0 if bites and passes else 1


def main() -> int:
    ap = argparse.ArgumentParser(description=__doc__)
    ap.add_argument("--root", type=Path, default=Path("."))
    ap.add_argument("--self-test", action="store_true")
    args = ap.parse_args()
    return self_test() if args.self_test else check(args.root)


if __name__ == "__main__":
    sys.exit(main())
