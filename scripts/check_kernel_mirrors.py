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
"""Fail when a capsule the catalogue says the kernel spawns has no spawn.

Each tools/nix/capsules.json entry with a `kernel_mirror` must have that
directory, an .rs file in it, and its `feature` named in kernel source
(`feature = "<feature>"`). capsule_power had neither and never ran, so the
shell's power-off found no service.  check_kernel_mirrors.py [--self-test]"""

import json
import sys
from pathlib import Path

CAPSULES = Path("tools/nix/capsules.json")


def missing(entries, exists, has_rs, named):
    """[(slug, why)] for each entry whose mirror or feature is absent."""
    out = []
    for e in entries:
        mirror, feature = e.get("kernel_mirror", ""), e.get("feature", "")
        if not mirror:
            continue
        if not exists(mirror):
            out.append((e["slug"], f"{mirror} does not exist"))
        elif not has_rs(mirror):
            out.append((e["slug"], f"{mirror} holds no .rs file"))
        if feature and not named(feature):
            out.append((e["slug"], f'no kernel source names feature = "{feature}"'))
    return out


def kernel_text():
    return "\n".join(p.read_text(errors="replace") for p in Path("src").rglob("*.rs"))


def main():
    entries = json.loads(CAPSULES.read_text())
    text = kernel_text()
    bad = missing(
        entries,
        lambda m: Path(m).is_dir(),
        lambda m: any(Path(m).glob("*.rs")),
        lambda f: f'feature = "{f}"' in text,
    )
    for slug, why in bad:
        print(f"kernel mirrors: {slug}: {why}")
    if bad:
        return 1
    n = sum(1 for e in entries if e.get("kernel_mirror"))
    print(f"kernel mirrors: {n} capsules the kernel spawns, each with its mirror and its feature in kernel source")
    return 0


if __name__ == "__main__":
    if "--self-test" in sys.argv[1:]:
        from check_kernel_mirrors_selftest import self_test

        sys.exit(self_test())
    sys.exit(main())
