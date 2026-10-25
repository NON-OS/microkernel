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
"""Keep the copy-only capsule features out of every committed build: each is
built only into a dev image's own copy, by tools/nonos-dev-image behind one
gate. No committed capsules.json, Capsule.mk or default features may name one.
"""

import argparse
import json
import re
import sys
from pathlib import Path

# feature: (its capsule's Cargo.toml, the one gate that turns it on)
FEATURES = {
    "selftest-panic": ("userland/capsule_linux/Cargo.toml",
                       'if a.profile == "selftest":\n        selftest_panic()'),
    "agentproof-headless": ("userland/capsule_terminal/Cargo.toml",
                            "if a.agentproof_job:\n        agentproof_job("),
}


def problems(root: Path) -> list:
    out = []
    catalogue = json.loads((root / "tools/nix/capsules.json").read_text())
    mks = sorted(root.glob("userland/*/Capsule.mk"))
    tool = (root / "tools/nonos-dev-image").read_text()
    for feature, (cargo, gate) in FEATURES.items():
        out += [f"capsules.json builds {e['slug']} with {feature}" for e in catalogue
                if feature in e.get("cargo_features", "")]
        out += [f"{mk.relative_to(root)} turns on {feature}" for mk in mks
                if any(line.startswith("CAPSULE_CARGO_FEATURES") and feature in line
                       for line in mk.read_text().splitlines())]
        default = re.search(r"^default\s*=\s*\[([^\]]*)\]", (root / cargo).read_text(), re.M)
        if default and feature in default.group(1):
            out.append(f"{cargo}'s default features name {feature}")
        if tool.count(gate) != 1:
            out.append(f"tools/nonos-dev-image turns on {feature} outside its gate")
    return out


def main() -> int:
    ap = argparse.ArgumentParser(description=__doc__)
    ap.add_argument("--root", type=Path, default=Path("."))
    ap.add_argument("--self-test", action="store_true")
    args = ap.parse_args()
    if args.self_test:
        sys.path.insert(0, str(Path(__file__).parent))
        from copy_only_fixture import self_test  # noqa: E402
        return self_test(problems)
    found = problems(args.root)
    for p in found:
        print(f"copy-only: {p}")
    print(f"copy-only: {len(found)} problems")
    return 1 if found else 0


if __name__ == "__main__":
    sys.exit(main())
