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

A mirror on the local network is dialled directly, past nonos_route_link.
Two features compile a direct path in, and no release build carries either:
`dev-mirror`, a development image's mirror, and `install-fast-route`, an app
image's chunks straight from the mirror, which waits for the project's sign-off. This check fails when:
  - either feature is not declared, or is in `default`, or in capsules.json's
    cargo_features for the release set;
  - a module of the direct path (net/raw.rs, net/raw_io.rs, net/route.rs,
    install/http/direct.rs, install/image/fast.rs), KIND_STREAM or the
    development mirror's address or its relay for a named mirror
    (install/mirror.rs, AT and dialled) is declared without its
    cfg on the line before it: `dev-mirror` alone for the development
    mirror's own code, `install-fast-route` alone for fast.rs, and either of
    the two only for raw_io and KIND_STREAM, which both paths use;
  - any other line of capsule_linux names the direct path without one of
    those cfgs within the three lines before it.
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
FAST = '#[cfg(feature = "install-fast-route")]'
EITHER = '#[cfg(any(feature = "dev-mirror", feature = "install-fast-route"))]'
FEATURES = ("dev-mirror", "install-fast-route")
# Each gated item, and the cfgs that may stand on the line before it.
GATED = {"src/linux/net/mod.rs": [("pub mod raw;", {CFG}), ("pub mod raw_io;", {CFG, EITHER}),
                                  ("pub mod route;", {CFG})],
         "src/linux/install/http/mod.rs": [("pub(super) mod direct;", {CFG})],
         "src/linux/install/image/mod.rs": [("mod fast;", {FAST})],
         "src/linux/net/ops.rs": [("pub const KIND_STREAM", {CFG, EITHER})],
         "src/linux/install/mirror.rs": [('const AT: &str = "10.0.2.2:8080"', {CFG}),
                                         ("pub fn dialled(_: &'static str, _: u16)", {CFG})]}
PATH_FILES = {"src/linux/net/raw.rs", "src/linux/net/raw_io.rs", "src/linux/net/route.rs",
              "src/linux/install/http/direct.rs", "src/linux/install/image/fast.rs"}
NAMES = re.compile(r"net::raw\b|raw_io::|route::is_local|direct::direct|\bdirect\(|KIND_STREAM")


def faults(root: Path) -> list:
    out, crate = [], root / CAPSULE
    toml = (crate / "Cargo.toml").read_text()
    features = toml.split("[features]", 1)[1].split("\n[", 1)[0] if "[features]" in toml else ""
    for feat in FEATURES:
        if not re.search(rf"^{feat}\s*=", features, re.MULTILINE):
            out.append(f"capsule_linux declares no {feat} feature")
        if re.search(rf"^default\s*=.*{feat}", features, re.MULTILINE):
            out.append(f"{feat} is in capsule_linux's default features")
    for e in json.loads((root / "tools/nix/capsules.json").read_text()):
        for feat in FEATURES:
            if feat in e.get("cargo_features", ""):
                out.append(f"capsules.json: {e['slug']} builds with {feat} in the release set")
    for rel, items in GATED.items():
        lines = (crate / rel).read_text().splitlines()
        for item, cfgs in items:
            at = [i for i, line in enumerate(lines) if line.strip().startswith(item)]
            if not at or any(i == 0 or lines[i - 1].strip() not in cfgs for i in at):
                out.append(f"{rel}: {item} without {' or '.join(sorted(cfgs))}")
    for path in sorted((crate / "src").rglob("*.rs")):
        rel = str(path.relative_to(crate))
        if rel in PATH_FILES:
            continue
        lines = path.read_text().splitlines()
        for i, line in enumerate(lines):
            code = line.split("//", 1)[0]
            near = lines[max(0, i - 3):i]
            if NAMES.search(code) and not any(c in p for p in near for c in (CFG, FAST, EITHER)):
                out.append(f"{rel}:{i + 1}: names the direct path outside its cfg")
    return out


DECOYS = [
    ("src/linux/net/mod.rs", CFG + "\npub mod raw;\n", "pub mod raw;\n"),
    ("src/linux/install/http/mod.rs", CFG + "\npub(super) mod direct;\n", "pub(super) mod direct;\n"),
    ("src/linux/net/ops.rs", EITHER + "\npub const KIND_STREAM", "pub const KIND_STREAM"),
    ("src/linux/install/mirror.rs", '"dl-cdn.alpinelinux.org:80"', '"10.0.2.2:8080"'),
    ("src/linux/install/mirror.rs", CFG + "\npub fn dialled(_:", "pub fn dialled(_:"),
    ("src/linux/install/http/get.rs", "    " + CFG + "\n    let raw = match is_local", "    let raw = match is_local"),
    ("Cargo.toml", "dev-mirror = []", "default = [\"dev-mirror\"]\ndev-mirror = []"),
    ("Cargo.toml", "dev-mirror = []", ""),
    ("src/linux/install/image/mod.rs", FAST + "\nmod fast;\n", "mod fast;\n"),
    ("src/linux/net/mod.rs", EITHER + "\npub mod raw_io;\n", "pub mod raw_io;\n"),
    ("src/linux/net/mod.rs", CFG + "\npub mod route;\n", EITHER + "\npub mod route;\n"),
    ("Cargo.toml", "install-fast-route = []", "default = [\"install-fast-route\"]\ninstall-fast-route = []"),
    ("Cargo.toml", "install-fast-route = []", ""),
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
    print(f"dev-mirror: {'FAIL' if found else 'ok'}, capsule_linux's direct dial is in no release build")
    return 1 if found else 0


if __name__ == "__main__":
    sys.exit(main(sys.argv[1:]))
