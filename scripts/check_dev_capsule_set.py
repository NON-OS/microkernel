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
"""No release path takes the development capsule set (tools/nix/capsules.nix, bySlugDev,
CAPSULE_DEV_CARGO_FEATURES), the one owner of these pins: each line a release depends on
still says what it says, and each way a release could reach the set stays absent."""

import argparse
import sys
from pathlib import Path

from dev_capsule_set_pins import ABSENT, MUTANTS, PINS

ROOT = Path(__file__).resolve().parents[1]


def failures(files):
    return ([f"{p}: no longer holds: {want}" for p, wants in PINS.items() for want in wants if want not in files[p]]
            + [f"{p}: names {bad}" for p, bads in ABSENT.items() for bad in bads if bad in files[p]])


def main():
    ap = argparse.ArgumentParser(description=__doc__, formatter_class=argparse.RawDescriptionHelpFormatter)
    ap.add_argument("--self-test", action="store_true")
    files = {p: (ROOT / p).read_text() for p in PINS.keys() | ABSENT.keys()}
    if not ap.parse_args().self_test:
        failed = failures(files)
        print("\n".join(f"dev-capsule-set: {f}" for f in failed) or "dev-capsule-set: no release path takes capsules-dev")
        return 1 if failed else 0
    # A mutant whose old text is gone counts as missed, so the list cannot go stale quietly.
    missed = [i for i, (p, old, new) in enumerate(MUTANTS)
              if old not in files[p] or not failures(dict(files, **{p: files[p].replace(old, new)}))]
    bad = missed or failures(files)
    print(f"dev-capsule-set: self-test failed: mutants {missed}, tree {failures(files)}" if bad else
          f"dev-capsule-set: self-test passed, the tree accepted and {len(MUTANTS)} mutants refused")
    return 1 if bad else 0


if __name__ == "__main__":
    sys.exit(main())
