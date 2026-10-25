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
"""Keep InputDrain (capability bit 36) to the input router.

Bit 36 drains the raw input ring, every key typed into every program, and
nothing in the kernel ties it to a name, so signing is what keeps it to one
capsule. The drivers post with InputSource and never need it. This refuses the
bit in any other capsule's caps ceiling, required caps or optional caps. With
no arguments it checks every userland/*/Capsule.mk; the capsule signing rules
call it with --slug and --caps before they sign a certificate or a manifest.
"""

import argparse
import re
import sys
from pathlib import Path

INPUT_DRAIN_BIT = 36
HOLDERS = ("input-router",)
KEYS = ("CAPSULE_CAPS_CEILING", "CAPSULE_REQUIRED_CAPS", "CAPSULE_OPTIONAL_CAPS")


def refusals(slug, caps):
    if slug in HOLDERS:
        return []
    return [f"{slug}: caps {c} carry InputDrain (bit {INPUT_DRAIN_BIT}), held only by "
            f"{', '.join(HOLDERS)}" for c in caps if c and (int(c, 0) >> INPUT_DRAIN_BIT) & 1]


def every_capsule(root):
    for mk in sorted(root.glob("userland/*/Capsule.mk")):
        text = mk.read_text()
        slug = re.search(r"^CAPSULE_SLUG\s*:=\s*(\S+)", text, re.M)
        values = (re.search(rf"^{k}\s*:=\s*(0[xX][0-9A-Fa-f]+|\d+)", text, re.M) for k in KEYS)
        if slug:
            yield slug.group(1), [v.group(1) for v in values if v]


def main() -> int:
    ap = argparse.ArgumentParser(description=__doc__.split("\n\n")[0])
    ap.add_argument("--root", type=Path, default=Path("."), help="repository root")
    ap.add_argument("--slug", help="the capsule about to be signed")
    ap.add_argument("--caps", nargs="*", default=[], help="its caps ceiling, required and optional caps")
    args = ap.parse_args()
    pairs = [(args.slug, args.caps)] if args.slug else list(every_capsule(args.root))
    found = [r for slug, caps in pairs for r in refusals(slug, caps)]
    for r in found:
        print(f"input-drain-cap: {r}", file=sys.stderr)
    if not args.slug:
        print(f"input-drain-cap: {len(pairs)} capsules, {len(found)} problems")
    return 1 if found else 0


if __name__ == "__main__":
    sys.exit(main())
