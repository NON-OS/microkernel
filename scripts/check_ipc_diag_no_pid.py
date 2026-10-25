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
"""No ipc.call diag line names a pid (K10, K19, B-04).

Kernel lines leave the machine in photos and bug reports: an ipc.call Site
may call only Site methods whose parameters and body carry no pid.
"""

import argparse
import re
import sys
from pathlib import Path

LABEL = 'Site::new(b"ipc.call")'
PIDLIKE = re.compile(r"pid|caller|server", re.I)
SITE = re.compile(r"static (\w+): [\w:]+ = [\w:]*" + re.escape(LABEL))


def methods(diag: str) -> dict:
    found = re.finditer(r"pub fn (\w+)\(([^)]*)\)[^{]*\{(.*?)\n    \}\n", diag, re.S)
    return {m.group(1): (m.group(2), m.group(3)) for m in found}


def problems(diag: str, sources: dict) -> list:
    known, out, sites = methods(diag), [], 0
    for path, text in sources.items():
        for name in SITE.findall(text):
            sites += 1
            for call in re.findall(rf"\b{name}\.(\w+)\(", text):
                params, body = known.get(call, ("?pid", ""))
                if PIDLIKE.search(params) or PIDLIKE.search(body):
                    out.append(f"{path}: {name}.{call} can format a pid")
    if sites == 0:
        out.append(f"no {LABEL} site found: the scan no longer sees the ipc.call lines")
    return out


SETTLE = "src/syscall/microkernel/ipc/call/settle.rs"
CASES = [("as built", lambda d, s: (d, s), 0),
         ("unanswered prints caller", lambda d, s: (d.replace('b" unanswered ");', 'b" unanswered ");\n        serial::print(b" caller=");'), s), 1),
         ("late takes a pid", lambda d, s: (d.replace("pub fn late(&self, what: &str,", "pub fn late(&self, what: &str, pid: u32,"), s), 1),
         ("site calls refused", lambda d, s: (d, {**s, SETTLE: s[SETTLE].replace("STARVED.starved(&endpoint.name, rc)", "STARVED.refused(&endpoint.name, pid, 0, 0)")}), 1),
         ("label renamed", lambda d, s: (d, {p: t.replace(LABEL, 'Site::new(b"ipc.cal")') for p, t in s.items()}), 1)]


def main() -> int:
    ap = argparse.ArgumentParser(description=__doc__.split("\n")[0])
    ap.add_argument("--self-test", action="store_true")
    a = ap.parse_args()
    diag = Path("src/sys/diag/mod.rs").read_text() + Path("src/sys/diag/late.rs").read_text()
    sources = {str(p): p.read_text() for p in sorted(Path("src").rglob("*.rs"))}
    if a.self_test:
        bad = [w for w, f, n in CASES if len(problems(*f(diag, sources))) != n]
        print(f"ipc-diag-no-pid self-test: {len(CASES)} cases, {len(bad)} wrong {bad if bad else ''}")
        return 1 if bad else 0
    found = problems(diag, sources)
    for p in found:
        print(f"ipc-diag-no-pid: {p}")
    print(f"ipc-diag-no-pid: {len(found)} problems")
    return 1 if found else 0


if __name__ == "__main__":
    sys.exit(main())
