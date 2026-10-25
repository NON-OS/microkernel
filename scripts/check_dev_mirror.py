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
"""Hold capsule_linux's direct dial to development builds (docs/build/dev-mirror.md).

A mirror on the local network is dialled directly, past nonos_route_link,
and only the `dev-mirror` feature compiles that path in. This check fails
when:
  - the feature is not declared, or is in `default`, or in capsules.json's
    cargo_features for the release set;
  - a module of the direct path (net/raw.rs, net/raw_io.rs, net/route.rs,
    install/http/direct.rs) or KIND_STREAM is declared without
    #[cfg(feature = "dev-mirror")] on the line before it;
  - any other line of capsule_linux names the direct path without that
    cfg within the three lines before it.
--self-test proves each rule bites on a decoy.
"""

import json
import re
import shutil
import sys
import tempfile
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent
CAPSULE = "userland/capsule_linux"
CFG = '#[cfg(feature = "dev-mirror")]'
GATED = {"src/linux/net/mod.rs": ["pub mod raw;", "pub mod raw_io;", "pub mod route;"],
         "src/linux/install/http/mod.rs": ["pub(super) mod direct;"],
         "src/linux/net/ops.rs": ["pub const KIND_STREAM"]}
PATH_FILES = {"src/linux/net/raw.rs", "src/linux/net/raw_io.rs", "src/linux/net/route.rs",
              "src/linux/install/http/direct.rs"}
NAMES = re.compile(r"net::raw\b|raw_io::|route::is_local|direct::direct|\bdirect\(|KIND_STREAM")


def faults(root: Path) -> list:
    out, crate = [], root / CAPSULE
    toml = (crate / "Cargo.toml").read_text()
    features = toml.split("[features]", 1)[1].split("\n[", 1)[0] if "[features]" in toml else ""
    if not re.search(r"^dev-mirror\s*=", features, re.M):
        out.append("capsule_linux declares no dev-mirror feature")
    if re.search(r"^default\s*=.*dev-mirror", features, re.M):
        out.append("dev-mirror is in capsule_linux's default features")
    for e in json.loads((root / "tools/nix/capsules.json").read_text()):
        if "dev-mirror" in e.get("cargo_features", ""):
            out.append(f"capsules.json: {e['slug']} builds with dev-mirror in the release set")
    for rel, items in GATED.items():
        lines = (crate / rel).read_text().splitlines()
        for item in items:
            at = [i for i, line in enumerate(lines) if line.strip().startswith(item)]
            if not at or any(i == 0 or lines[i - 1].strip() != CFG for i in at):
                out.append(f"{rel}: {item} without {CFG}")
    for path in sorted((crate / "src").rglob("*.rs")):
        rel = str(path.relative_to(crate))
        if rel in PATH_FILES:
            continue
        lines = path.read_text().splitlines()
        for i, line in enumerate(lines):
            code = line.split("//", 1)[0]
            if NAMES.search(code) and not any(CFG in p for p in lines[max(0, i - 3):i]):
                out.append(f"{rel}:{i + 1}: names the direct path outside {CFG}")
    return out


DECOYS = [
    ("src/linux/net/mod.rs", CFG + "\npub mod raw;\n", "pub mod raw;\n"),
    ("src/linux/install/http/mod.rs", CFG + "\npub(super) mod direct;\n", "pub(super) mod direct;\n"),
    ("src/linux/net/ops.rs", CFG + "\npub const KIND_STREAM", "pub const KIND_STREAM"),
    ("src/linux/install/http/get.rs", "    " + CFG + "\n    let raw = match is_local", "    let raw = match is_local"),
    ("Cargo.toml", "dev-mirror = []", "default = [\"dev-mirror\"]\ndev-mirror = []"),
    ("Cargo.toml", "dev-mirror = []", ""),
]


def self_test() -> int:
    with tempfile.TemporaryDirectory() as d:
        tree = Path(d)
        shutil.copytree(ROOT / CAPSULE, tree / CAPSULE, ignore=shutil.ignore_patterns("target"))
        (tree / "tools/nix").mkdir(parents=True)
        shutil.copy(ROOT / "tools/nix/capsules.json", tree / "tools/nix/capsules.json")
        assert faults(tree) == [], f"self-test: the tree as it is fails: {faults(tree)}"
        for rel, old, new in DECOYS:
            path = tree / CAPSULE / rel
            text = path.read_text()
            assert text.count(old) == 1, f"self-test: decoy {old!r} applies {text.count(old)} times in {rel}"
            path.write_text(text.replace(old, new))
            assert faults(tree), f"self-test: decoy in {rel} ({new.strip()!r}) passed"
            path.write_text(text)
    print(f"dev-mirror: self-test bites on each of {len(DECOYS)} decoys")
    return 0


def main(argv) -> int:
    if "--self-test" in argv:
        return self_test()
    found = faults(ROOT)
    for f in found:
        print(f"dev-mirror: {f}", file=sys.stderr)
    print(f"dev-mirror: {'FAIL' if found else 'ok'}, capsule_linux's direct dial is development only")
    return 1 if found else 0


if __name__ == "__main__":
    sys.exit(main(sys.argv[1:]))
