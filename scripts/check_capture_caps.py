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
"""Keep CameraCapture (37), MicrophoneCapture (38) and ScreenCapture (39) off every capsule.

The kernel never sees a camera or a microphone, nor takes a screenshot;
the USB stack, the audio service and the compositor ask MkCapCheck whether
a peer holds these bits. So signing is what keeps them scarce. No capsule
holds any of them until the owner decides Decision 9 (who may capture, and
with what consent), and a holder is then added below by name. With no arguments this checks every
userland/*/Capsule.mk; the seal calls it with --slug and --caps before it
signs a certificate or a manifest.
"""

import argparse
import re
import sys
from pathlib import Path

# bit -> (name, the capsule slugs allowed to ask for it)
CAPTURE = {37: ("CameraCapture", ()), 38: ("MicrophoneCapture", ()), 39: ("ScreenCapture", ())}
KEYS = ("CAPSULE_CAPS_CEILING", "CAPSULE_REQUIRED_CAPS", "CAPSULE_OPTIONAL_CAPS")


def refusals(slug, caps):
    out = []
    for value in (c for c in caps if c):
        for bit, (name, holders) in CAPTURE.items():
            if (int(value, 0) >> bit) & 1 and slug not in holders:
                who = ", ".join(holders) or "no capsule yet (Decision 9)"
                out.append(f"{slug}: caps {value} carry {name} (bit {bit}), held by {who}")
    return out


def capsules(root):
    for mk in sorted(root.glob("userland/*/Capsule.mk")):
        text = mk.read_text()
        slug = re.search(r"^CAPSULE_SLUG\s*:=\s*(\S+)", text, re.M)
        found = (re.search(rf"^{k}\s*:=\s*(0[xX][0-9A-Fa-f]+|\d+)", text, re.M) for k in KEYS)
        if slug:
            yield slug.group(1), [m.group(1) for m in found if m]


def main() -> int:
    ap = argparse.ArgumentParser(description=__doc__.split("\n\n")[0])
    ap.add_argument("--root", type=Path, default=Path("."), help="repository root")
    ap.add_argument("--slug", help="the capsule about to be signed")
    ap.add_argument("--caps", nargs="*", default=[], help="its caps ceiling, required and optional caps")
    args = ap.parse_args()
    pairs = [(args.slug, args.caps)] if args.slug else list(capsules(args.root))
    problems = [r for slug, caps in pairs for r in refusals(slug, caps)]
    for r in problems:
        print(f"capture-caps: {r}", file=sys.stderr)
    if not args.slug:
        print(f"capture-caps: {len(pairs)} capsules, {len(problems)} problems")
    return 1 if problems else 0


if __name__ == "__main__":
    sys.exit(main())
