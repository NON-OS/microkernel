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
"""Only crate::fs opens a data volume scope (privacy/volume-gate).

A scope lets the kernel's data volume I/O pass the block clients' gate as
the kernel. The token, `VolumeIo`, has a field private to
src/fs/cryptoblock/volume_io.rs, and `volume_scope` is fs-only; this scan
holds every other file to that, so a path a capsule reaches never opens one.
"""

import argparse
import re
import sys
from pathlib import Path

DEFINES = {
    "src/hardware/block_device/volume_scope.rs": "pub fn volume_scope(_: &VolumeIo)",
    "src/hardware/block_device/mod.rs": "pub use volume_scope::",
}
USE = re.compile(r"\bvolume_scope\s*\(|\bVolumeIo\s*\(")


def problems(sources: dict) -> list:
    out = []
    for path, text in sources.items():
        if path.startswith("src/fs/"):
            continue
        for n, line in enumerate(text.splitlines(), 1):
            code = line.split("//")[0]
            if USE.search(code) and not (path in DEFINES and DEFINES[path] in code):
                out.append(f"{path}:{n}: opens a data volume scope outside crate::fs")
    if not any(p.startswith("src/fs/") and "volume_scope();" in t for p, t in sources.items()):
        out.append("no scope opened under src/fs: the scan no longer sees the volume's I/O")
    return out


GATE = "src/hardware/nvme_capsule/capability.rs"
CASES = [("as built", lambda s: s, 0),
         ("a gate opens one", lambda s: {**s, GATE: s[GATE] + "\nfn x() { let _ = crate::hardware::block_device::volume_scope(&t); }\n"}, 1),
         ("a token made outside fs", lambda s: {**s, GATE: s[GATE] + "\nfn y() { let _ = VolumeIo(()); }\n"}, 1),
         ("fs stops using it", lambda s: {p: (t if not p.startswith("src/fs/") else t.replace("volume_scope(", "volume_cope(")) for p, t in s.items()}, 1)]


def main() -> int:
    ap = argparse.ArgumentParser(description=__doc__.split("\n")[0])
    ap.add_argument("--self-test", action="store_true")
    a = ap.parse_args()
    sources = {str(p): p.read_text() for p in sorted(Path("src").rglob("*.rs"))}
    if a.self_test:
        bad = [w for w, f, n in CASES if len(problems(f(sources))) != n]
        print(f"volume-io self-test: {len(CASES)} cases, {len(bad)} wrong {bad if bad else ''}")
        return 1 if bad else 0
    found = problems(sources)
    for p in found:
        print(f"volume-io: {p}")
    print(f"volume-io: {len(found)} problems")
    return 1 if found else 0


if __name__ == "__main__":
    sys.exit(main())
