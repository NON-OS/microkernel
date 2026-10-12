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
"""Fail when a kernel spawn mirror grants a capsule more than its manifest.

A capsule runs with `required | (optional & granted)` from its signed
manifest, and the spawn refuses a grant wider than the manifest
(src/security/capsule_manifest/verify/caps_bits.rs). A mask cut in
Capsule.mk and left in the mirror's `requested_caps` therefore stops that
capsule at boot, and nothing on the host said so: the capability audit holds
each mask to its code, not the mirror to the manifest. This evaluates every
`requested_caps` a spawn file names, through Capability::X.bit() terms,
serial_debug_cap(), hex literals and the u64 constants the file defines, and
fails on any bit outside the manifest's required and optional sets.

A capsule's README that quotes `CAPSULE_REQUIRED_CAPS` is held to the same
manifest, so a cut mask does not go on being described as the old one."""

import argparse
import sys
from pathlib import Path

from mirror_caps_readme import stale_readmes
from mirror_caps_wider import wider


def main():
    ap = argparse.ArgumentParser(description=__doc__.splitlines()[0])
    ap.add_argument("--root", type=Path, default=Path("."))
    ap.add_argument("--self-test", action="store_true")
    a = ap.parse_args()
    if a.self_test:
        from mirror_caps_selftest import self_test

        self_test()
        return 0
    bad, checked = wider(a.root)
    for line in bad:
        print(f"mirror caps: {line}")
    stale = stale_readmes(a.root)
    for line in stale:
        print(f"mirror caps: {line}")
    print(f"mirror caps: {checked} grants checked, {len(bad)} wider than their manifest, "
          f"{len(stale)} READMEs quoting another mask")
    return 1 if bad or stale else 0


if __name__ == "__main__":
    sys.exit(main())
