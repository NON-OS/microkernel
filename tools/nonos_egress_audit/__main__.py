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

"""nonos-egress-audit: exit 1, naming each, when the source and privacy.md
disagree about where the image can reach."""

import argparse
import sys
from pathlib import Path

from . import caps, check, ledger, scan

LEDGER = "docs/security/privacy.md"
# The list runs over the pages that start with its name, so it can be split.
PAGES = "privacy*.md"
# Host tools that write endpoints into the image as data, not as source.
TOOLS = ("tools/nonos_qwen_tier",)


def audit(root, page):
    root = root.resolve()
    capsules = caps.networked(root)
    dirs = caps.crates(root, capsules) + [root / t for t in TOOLS if (root / t).is_dir()]
    literals = scan.found(dirs, root)
    return capsules, literals, check.findings(capsules, literals, ledger.read(page))


def main():
    ap = argparse.ArgumentParser(description=__doc__)
    ap.add_argument("--root", type=Path, default=Path("."))
    ap.add_argument("--list", action="store_true", help="print what the source names")
    a = ap.parse_args()
    page = a.root / LEDGER
    if not page.exists() and not a.list:
        sys.exit(f"[egress] {LEDGER} is missing")
    text = "\n".join(p.read_text() for p in sorted(page.parent.glob(PAGES)))
    capsules, literals, bad = audit(a.root, text)
    if a.list:
        print("\n".join(capsules))
        print("\n".join(f"{k} {v[0]}" for k, v in sorted(literals.items())))
    for line in bad:
        print(f"[egress] {line}")
    print(f"[egress] {len(capsules)} networked capsules, {len(literals)} places, "
          f"{len(bad)} disagreements")
    return 1 if bad else 0


if __name__ == "__main__":
    sys.exit(main())
