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
"""Which embedding modules lack the empty-image assert: a module that
include_bytes!s a capsule ELF must assert every const it includes is not
empty, and a macro that embeds by `include_bytes!($elf)` must assert too."""

import re
from pathlib import Path

ELF = re.compile(r'include_bytes!\(\s*(?:concat!\(\s*)?"[^"]*userland/[^"]*/target/')
CONST = re.compile(r"const (\w+): &(?:'static )?\[u8\]\s*=\s*include_bytes!")
ASSERT = re.compile(r"const _: \(\) = assert!\((.*?)\);", re.S)


def problems(src: Path) -> list:
    out = []
    for f in sorted(src.rglob("*.rs")):
        text = f.read_text()
        if "macro_rules!" in text and "include_bytes!($elf)" in text:
            if not ASSERT.search(text):
                out.append(f"{f}: the embedding macro carries no empty-image assert")
            continue
        if not ELF.search(text):
            continue
        asserted = " ".join(a.group(1) for a in ASSERT.finditer(text))
        missing = [c for c in CONST.findall(text) if f"!{c}.is_empty()" not in asserted]
        if missing:
            out.append(f"{f}: no empty-image assert for {', '.join(missing)}")
    return out
