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

"""nonos-zero-state-check: exit 0 only when a secret typed into a live boot
was in RAM before the power button, gone from RAM after the wipe, absent from
the disk, and unknown to the next boot."""

import argparse
import sys
from pathlib import Path

from . import leftovers
from .memscan import marker
from .runs import first_boot, second_boot

def main():
    ap = argparse.ArgumentParser(description=__doc__)
    ap.add_argument("--image", type=Path, required=True)
    ap.add_argument("--out", type=Path, required=True)
    ap.add_argument("--steps", type=Path, required=True, help="first boot: type {secret}")
    ap.add_argument("--off", type=Path, required=True, help="the desktop's Power Off")
    ap.add_argument("--after", type=Path, required=True, help="second boot's steps")
    ap.add_argument("--mem", default="3G")
    ap.add_argument("--off-secs", type=float, default=180)
    a = ap.parse_args()
    a.out.mkdir(parents=True, exist_ok=True)
    secret, bad = marker(), []

    def say(what, got, want):
        ok = bool(got) == want
        print(f"[ZS-TEST] {what}: {got} ({'as it must be' if ok else 'WRONG'})", flush=True)
        bad.extend([] if ok else [what])

    print(f"[ZS-TEST] this run's secret: {secret}", flush=True)
    first_boot(a, secret, say)
    added = leftovers.store_rows_added(a.image, a.out / "boot1.img")
    say("store rows the boot added", added, False)
    say("on the disk", leftovers.disk_holds(a.out / "boot1.img", secret), False)
    say("known to the next boot", second_boot(a, secret), False)
    print(f"[ZS-TEST] {'PASS' if not bad else 'FAIL: ' + ', '.join(bad)}", flush=True)
    return 1 if bad else 0


if __name__ == "__main__":
    sys.exit(main())
