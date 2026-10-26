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
"""Hold every argv to one bound (src/process/core/argv_bound.rs).

MkArgs hands a process its argv joined with NULs; userland reads it into a
buffer, and an argv longer than the buffer comes back as nothing. Since I-C5
an install's argv is about 720 bytes and a 256-byte read lost it, so the
install ran in the run heap. This check fails when:
  - the kernel's ARGV_BYTES_MAX and nonos_libc's ARGV_BYTES_MAX differ;
  - MkToolRun's MAX_ARGV is larger than ARGV_BYTES_MAX;
  - anything under src/ writes a process's argv but set_argv (argv.rs), or
    set_argv stops refusing an argv that does not fit;
  - capsule_linux's heap.rs reads its args into anything but
    nonos_libc::ARGV_BYTES_MAX.
--self-test proves each rule bites on a decoy.
"""

import re
import shutil
import sys
import tempfile
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent
KERNEL = "src/process/core/argv_bound.rs"
LIBC = "userland/libc/src/process.rs"
TOOL = "src/syscall/microkernel/tool_run.rs"
SETTER = "src/process/core/argv.rs"
HEAP = "userland/capsule_linux/src/linux/heap.rs"
CONST = re.compile(r"(?:pub )?const (ARGV_BYTES_MAX|MAX_ARGV): usize = (\d+);")
WRITE = re.compile(r"\.argv\.lock\(\)\s*=")


def value(text, name):
    found = [int(v) for n, v in CONST.findall(text) if n == name]
    return found[0] if len(found) == 1 else None


def faults(root: Path) -> list:
    out = []
    kernel = value((root / KERNEL).read_text(), "ARGV_BYTES_MAX")
    libc = value((root / LIBC).read_text(), "ARGV_BYTES_MAX")
    tool = value((root / TOOL).read_text(), "MAX_ARGV")
    if kernel is None or kernel != libc:
        out.append(f"ARGV_BYTES_MAX: the kernel says {kernel}, nonos_libc {libc}")
    if tool is None or kernel is None or tool > kernel:
        out.append(f"MkToolRun's MAX_ARGV {tool} passes ARGV_BYTES_MAX {kernel}")
    for path in sorted((root / "src").rglob("*.rs")):
        rel = path.relative_to(root).as_posix()
        if rel != SETTER and WRITE.search(path.read_text(errors="replace")):
            out.append(f"{rel} writes a process's argv; use crate::process::set_argv")
    if "    if !fits(&words) {\n" not in (root / SETTER).read_text():
        out.append(f"{SETTER}: set_argv no longer refuses an argv that does not fit")
    if "[0u8; nonos_libc::ARGV_BYTES_MAX]" not in (root / HEAP).read_text():
        out.append(f"{HEAP} reads its args into a buffer not ARGV_BYTES_MAX long")
    return out


DECOYS = [
    (LIBC, "pub const ARGV_BYTES_MAX: usize = 4096;", "pub const ARGV_BYTES_MAX: usize = 256;"),
    (TOOL, "const MAX_ARGV: usize = 4096;", "const MAX_ARGV: usize = 8192;"),
    ("src/userspace/capsule_linux/run_net.rs", "let _ = crate::process::set_argv(pid, argv);",
     "crate::process::with_process(pid, |pcb| *pcb.argv.lock() = argv);"),
    (HEAP, "[0u8; nonos_libc::ARGV_BYTES_MAX]", "[0u8; 256]"),
    (SETTER, "    if !fits(&words) {", "    if false {"),
]


def self_test() -> int:
    with tempfile.TemporaryDirectory() as d:
        tree = Path(d)
        shutil.copytree(ROOT / "src", tree / "src")
        for rel in (LIBC, HEAP):
            (tree / rel).parent.mkdir(parents=True, exist_ok=True)
            shutil.copy(ROOT / rel, tree / rel)
        assert faults(tree) == [], f"self-test: the tree as it is fails: {faults(tree)}"
        for rel, old, new in DECOYS:
            path = tree / rel
            text = path.read_text()
            assert text.count(old) == 1, f"self-test: decoy {old!r} applies {text.count(old)} times"
            path.write_text(text.replace(old, new))
            assert faults(tree), f"self-test: decoy in {rel} passed"
            path.write_text(text)
    print(f"argv-bound: self-test bites on each of {len(DECOYS)} decoys")
    return 0


def main(argv) -> int:
    if "--self-test" in argv:
        return self_test()
    found = faults(ROOT)
    for f in found:
        print(f"argv-bound: {f}", file=sys.stderr)
    print(f"argv-bound: {'FAIL' if found else 'ok'}, every argv is held to one bound")
    return 1 if found else 0


if __name__ == "__main__":
    sys.exit(main(sys.argv[1:]))
