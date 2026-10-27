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
"""No process's bytes, and no Linux guest's name, reach a release console.

A release profile drops `capsule-serial-debug`, and its promise is that no
capsule writes to a console anyone could read (tools/nix/config.nix). The
kernel paths that copy a process's own bytes to serial, MkDebug, StdoutWrite
and the fd stdio path, are each held behind that feature, and a process is
named on a line only through `Line::who`, which shows a Linux guest only as
"foreign" there (src/sys/serial/who.rs). This check fails a copy of process
bytes to serial outside those paths or not behind the feature, and a raw
process name written into a line.
"""

import argparse
import re
import sys
from pathlib import Path

# The paths that may copy a process's bytes to serial, each behind the gate.
COPIES: tuple = ("src/syscall/microkernel/debug.rs", "src/syscall/microkernel/stdout_write.rs",
                 "src/fs/fd/syscalls/stdio.rs")
# Raw byte writers that are not a process's bytes, and why.
BYTES: dict = {"src/arch/x86_64/diag/fatal_notice.rs": "the kernel's own fatal notice",
               "src/boot/panic/end.rs": "the kernel's own panic line"}
GATE = "capsule-serial-debug"
COPY = re.compile(r"serial::(passthrough|core::write_byte)\s*\(")
NAME = re.compile(r"\.str\([^;]*?(\.name\.lock\(\)|\bpcb\.name\(\))")


def problems(sources: dict) -> list:
    out = []
    for path, text in sources.items():
        if path.startswith("src/sys/serial/"):
            continue
        for m in COPY.finditer(text):
            line = text.count("\n", 0, m.start())
            near = "\n".join(text.split("\n")[max(0, line - 6):line + 1])
            if path in BYTES:
                continue
            if path not in COPIES:
                out.append(f"{path}: a process's bytes copied to serial outside {', '.join(COPIES)}")
            elif GATE not in near:
                out.append(f"{path}: a copy to serial not behind {GATE}")
        for m in NAME.finditer(text):
            out.append(f"{path}: a process name written raw into a line; use Line::who")
    out += [f"{p}: listed but copies nothing to serial" for p in (*COPIES, *BYTES) if p in sources
            and not COPY.search(sources[p])]
    return out


D = "src/syscall/microkernel/debug.rs"
G = 'if !private && cfg!(feature = "capsule-serial-debug") {'
CASES = [("as built", lambda s: s, 0),
         ("an ungated copy", lambda s: {**s, D: s[D].replace(G, "if !private {")}, 1),
         ("a copy elsewhere", lambda s: {**s, "src/x.rs": "fn f(b: &[u8]) { crate::sys::serial::passthrough(b); }"}, 1),
         ("a raw name", lambda s: {**s, "src/x.rs": "fn f(l: &mut Line, p: &P) { l.str(p.name.lock().as_bytes()); }"}, 1),
         ("a raw pcb.name()", lambda s: {**s, "src/x.rs": "fn f(l: &mut Line, pcb: &P) { l.str(pcb.name().as_bytes()); }"}, 1),
         ("a profile name", lambda s: {**s, "src/x.rs": "fn f(l: &mut Line, p: &P) { l.str(p.name().as_bytes()); }"}, 0),
         ("a name through who", lambda s: {**s, "src/x.rs": "fn f(l: &mut Line, p: &P) { l.who(p.name.lock().as_bytes()); }"}, 0)]


def main() -> int:
    ap = argparse.ArgumentParser(description=__doc__.split("\n")[0])
    ap.add_argument("--self-test", action="store_true")
    a = ap.parse_args()
    sources = {str(p): p.read_text() for p in sorted(Path("src").rglob("*.rs"))}
    if a.self_test:
        bad = [w for w, f, n in CASES if len(problems(f(sources))) != n]
        print(f"private-serial self-test: {len(CASES)} cases, {len(bad)} wrong {bad if bad else ''}")
        return 1 if bad else 0
    found = problems(sources)
    for p in found:
        print(f"private-serial: {p}")
    print(f"private-serial: {len(found)} problems")
    return 1 if found else 0


if __name__ == "__main__":
    sys.exit(main())
