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
"""Find kernel features that gate code and that no build lane ever turns on.

A cargo feature is a switch. Code behind `cfg(feature = "x")` compiles only
when something passes `x` to cargo, and in this tree the things that do are
few and all readable: the `default` list and the profile lists in Cargo.toml,
every `--features` in the makefiles and workflows, and the capsule selection
`tools/nonos-config` builds from the `nonos-capsule-*` features. A feature
none of those reach is dark: the code behind it has not been compiled by any
lane, so it has not been type-checked, linted, tested or booted since the
last time a person built it by hand.

That is how `nonos-smp` sat unbuilt with four defects in it, and the IOMMU
bring-up compiled into every image never ran. Written once, wired nowhere is
the one defect class this tree kept producing, because everything else has a
gate and this did not.

Every declared feature is checked. A cfg site naming a feature Cargo.toml does
not declare is code that can never compile in, and fails outright. A declared
feature with cfg sites and no lane is dark. The dark set is held against
scripts/baselines/dark-features.txt: a feature may leave the list when a lane
starts enabling it, and a feature may not join it. --write-baseline records
the current set, and --self-test proves the check bites on a decoy.
"""

import argparse
import re
import sys
import tempfile
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
MATRIX_LIST = re.compile(r'^(\s+)features:\s*\n((?:\1\s+-\s+[A-Za-z0-9_,-]+\s*\n)+)', re.M)


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
                      for item in block.split("\n") if "-" in item]
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


def cfg_sites(root):
    counts = {}
    for p in (root / SRC).rglob("*.rs"):
        for name in CFG_SITE.findall(p.read_text(errors="replace")):
            counts[name] = counts.get(name, 0) + 1
    return counts


def analyse(root):
    table = features_table((root / CARGO).read_text())
    seeds = set(table.get("default", [])) | lane_features(root)
    # tools/nonos-config lets a person select any capsule feature by slug, so
    # a capsule feature is reachable by construction.
    seeds |= {f for f in table if f.startswith("nonos-capsule-")}
    on = enabled(table, seeds)
    sites = cfg_sites(root)
    undeclared = sorted(f for f in sites if f not in table)
    dark = sorted(f for f in table if f not in on and sites.get(f, 0) > 0)
    return dark, undeclared, sites


def self_test():
    """A decoy feature with cfg sites and no lane must be reported."""
    with tempfile.TemporaryDirectory() as d:
        root = Path(d)
        (root / "Cargo.toml").write_text(
            '[package]\nname = "x"\n[features]\ndefault = ["lit"]\nlit = []\n'
            'decoy = []\n[dependencies]\n')
        (root / "src").mkdir()
        (root / "src/lib.rs").write_text('#[cfg(feature = "lit")] fn a() {}\n'
                                         '#[cfg(feature = "decoy")] fn b() {}\n'
                                         '#[cfg(feature = "ghost")] fn c() {}\n')
        (root / "mk").mkdir()
        (root / "Makefile").write_text("build:\n\tcargo build --features lit\n")
        (root / ".github/workflows").mkdir(parents=True)
        (root / ".github/workflows/x.yml").write_text(
            "    strategy:\n      matrix:\n        features:\n          - viamatrix\n"
            "    steps:\n      - run: cargo check --features lit,${{ matrix.features }}\n")
        (root / "Cargo.toml").write_text((root / "Cargo.toml").read_text().replace(
            'decoy = []\n', 'decoy = []\nviamatrix = []\n'))
        (root / "src/lib.rs").write_text((root / "src/lib.rs").read_text()
                                         + '#[cfg(feature = "viamatrix")] fn d() {}\n')
        dark, undeclared, _ = analyse(root)
        assert dark == ["decoy"], dark
        assert undeclared == ["ghost"], undeclared
    print("dark-features: self-test passed, the decoy was reported")


def main():
    ap = argparse.ArgumentParser(description=__doc__)
    ap.add_argument("--root", type=Path, default=Path("."))
    ap.add_argument("--write-baseline", action="store_true")
    ap.add_argument("--self-test", action="store_true")
    args = ap.parse_args()
    if args.self_test:
        self_test()
        return 0
    dark, undeclared, sites = analyse(args.root)
    if undeclared:
        print("dark-features: cfg sites name features Cargo.toml does not declare, "
              "so that code can never compile in:", file=sys.stderr)
        for f in undeclared:
            print(f"  {f} ({sites[f]} sites)", file=sys.stderr)
        return 1
    if args.write_baseline:
        (args.root / BASELINE).write_text("".join(f"{f}\n" for f in dark))
        print(f"dark-features: baseline written with {len(dark)} features")
        return 0
    known = set((args.root / BASELINE).read_text().split()) if (args.root / BASELINE).exists() else set()
    new = [f for f in dark if f not in known]
    for f in dark:
        print(f"dark-features: {f} gates {sites[f]} cfg sites and no lane enables it")
    if new:
        print("dark-features: features that joined the dark set:", file=sys.stderr)
        for f in new:
            print(f"  {f}", file=sys.stderr)
        print("either wire a lane that builds it or remove the switch; the baseline "
              "may only shrink", file=sys.stderr)
        return 1
    print(f"dark-features: {len(dark)} known dark features, none new")
    return 0


if __name__ == "__main__":
    sys.exit(main())
