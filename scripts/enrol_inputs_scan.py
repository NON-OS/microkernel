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
"""Every include_bytes!/include_str! of a trust file in a crate, and whether
a #[cfg] on its module, or a module above it, keeps it out of the build."""

import re
from pathlib import Path

TRUST = re.compile(r'include_(?:bytes|str)!\s*\(\s*"[^"]*nonos-data/trust/[^"]*"')
GATE = re.compile(r'#\[cfg\((test|feature\s*=\s*"([^"]+)")\)\]')
MOD = re.compile(r"^\s*(?:pub(?:\([^)]*\))?\s+)?mod\s+(\w+)\s*;", re.M)


def gated(file: Path, src: Path, on: set) -> bool:
    """Whether a #[cfg] on the declaration of `file`'s module, or of any
    module above it, leaves it out with features `on`."""
    while file.name not in ("lib.rs", "main.rs") or file.parent != src:
        name = file.parent.name if file.name == "mod.rs" else file.stem
        up = file.parent.parent if file.name == "mod.rs" else file.parent
        parents = [up / "mod.rs", up.with_suffix(".rs")] if up != src else [src / "lib.rs", src / "main.rs"]
        for p in (p for p in parents if p.is_file()):
            text = p.read_text()
            for d in (d for d in MOD.finditer(text) if d.group(1) == name):
                attrs = text[: d.start()].rstrip().splitlines()[-3:]
                for g in (GATE.search(a) for a in attrs if a.strip().startswith("#[")):
                    if g and (g.group(1) == "test" or g.group(2) not in on):
                        return True
        file = parents[0] if parents[0].is_file() else parents[-1]
    return False


def trust_includes(crate: Path, on: set) -> list:
    """(file, compiled) for each source file of `crate` that includes a trust file."""
    src = crate / "src"
    return [(f, not gated(f, src, on)) for f in sorted(src.rglob("*.rs")) if TRUST.search(f.read_text())]
