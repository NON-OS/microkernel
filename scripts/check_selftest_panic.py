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
"""Keep capsule_linux's selftest-panic feature out of every committed build.

It makes the personality panic on one word, for the selftest image only:
tools/nonos-dev-image turns it on for that profile, in its own copy of the
tree. No committed build may name it (capsules.json, a Capsule.mk,
capsule_linux's defaults), and that one place must stay behind the profile.
"""

import argparse
import json
import re
import sys
from pathlib import Path

FEATURE = "selftest-panic"
GATE = 'if a.profile == "selftest":\n        selftest_panic()'


def problems(root: Path) -> list:
    out = []
    for e in json.loads((root / "tools/nix/capsules.json").read_text()):
        if FEATURE in e.get("cargo_features", ""):
            out.append(f"tools/nix/capsules.json builds {e['slug']} with {FEATURE}")
    for mk in sorted(root.glob("userland/*/Capsule.mk")):
        for line in mk.read_text().splitlines():
            if line.startswith("CAPSULE_CARGO_FEATURES") and FEATURE in line:
                out.append(f"{mk.relative_to(root)} turns on {FEATURE}")
    cargo = (root / "userland/capsule_linux/Cargo.toml").read_text()
    default = re.search(r"^default\s*=\s*\[([^\]]*)\]", cargo, re.M)
    if default and FEATURE in default.group(1):
        out.append(f"capsule_linux's default features name {FEATURE}")
    tool = (root / "tools/nonos-dev-image").read_text()
    if tool.count("selftest_panic()") != 2 or GATE not in tool:
        out.append(f"tools/nonos-dev-image turns on {FEATURE} outside the selftest profile")
    return out


def check(root: Path) -> int:
    found = problems(root)
    for p in found:
        print(f"selftest-panic: {p}")
    print(f"selftest-panic: {len(found)} problems")
    return 1 if found else 0


def main() -> int:
    ap = argparse.ArgumentParser(description=__doc__)
    ap.add_argument("--root", type=Path, default=Path("."))
    ap.add_argument("--self-test", action="store_true")
    args = ap.parse_args()
    if args.self_test:
        sys.path.insert(0, str(Path(__file__).parent))
        from selftest_panic_fixture import self_test  # noqa: E402
        return self_test(problems)
    return check(args.root)


if __name__ == "__main__":
    sys.exit(main())
