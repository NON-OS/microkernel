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
"""What lights a feature. Read by check_dark_features."""

import re
from pathlib import Path

CARGO = Path("Cargo.toml")
BASELINE = Path("scripts/baselines/dark-features.txt")
LANE_FILES = ["Makefile", "mk", ".github/workflows"]
SRC = Path("src")

FEATURE_LINE = re.compile(r'^\s*([A-Za-z0-9_-]+)\s*=\s*\[([^\]]*)\]', re.M | re.S)
CFG_SITE = re.compile(r'feature\s*=\s*"([A-Za-z0-9_-]+)"')
FLAG = re.compile(r'--features[ =]+([^\s\\]+)')
BUILD_CALL = re.compile(r'\$\(call nonos_kernel_build,[^,]*,([^)]*)\)')
VARIABLE = re.compile(r'\$\([^)]*\)|\$\{\{[^}]*\}\}')
# A workflow matrix hands cargo its feature string through an expression, so
# the values live in the list under `features:` rather than on the command
# line. Items are `- a,b` lines until the indentation drops back.
# A matrix list may carry comment lines between its entries.
MATRIX_LIST = re.compile(r'^(\s+)features:\s*\n((?:\1\s+(?:-\s+[A-Za-z0-9_,-]+|#.*)\s*\n)+)', re.M)

def features_table(text):
    """name -> features it enables, from the [features] section only."""
    start = text.index("[features]")
    rest = text[start + len("[features]"):]
    end = re.search(r'^\[', rest, re.M)
    section = rest[:end.start()] if end else rest
    section = re.sub(r'#[^\n]*', '', section)
    return {name: re.findall(r'"([^"]+)"', deps) for name, deps in FEATURE_LINE.findall(section)}

def lane_features(root):
    """Every feature a lane names on a cargo command line, variables dropped."""
    names = set()
    for base in LANE_FILES:
        paths = [root / base] if (root / base).is_file() else (root / base).rglob("*")
        for p in paths:
            if not p.is_file():
                continue
            text = p.read_text(errors="replace").replace("$(_boot_comma)", ",")
            matrix = [item.split("-", 1)[1] for _, block in MATRIX_LIST.findall(text)
                      for item in block.split("\n")
                      if "-" in item and not item.lstrip().startswith("#")]
            for m in FLAG.findall(text) + BUILD_CALL.findall(text) + matrix:
                for name in VARIABLE.sub("", m).split(","):
                    if name.strip():
                        names.add(name.strip())
    return names

def enabled(table, seeds):
    """Transitive closure over what features enable."""
    out, todo = set(), list(seeds)
    while todo:
        f = todo.pop()
        if f in out:
            continue
        out.add(f)
        todo.extend(d for d in table.get(f, []) if "/" not in d and ":" not in d)
    return out

