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
"""Keep the local test chain (the `devnet` cargo feature, P5) out of every
release. A capsule is built once per slug, so the feature reaches only the
flake's development set, capsules-dev, which no release takes. Fails when a
Cargo.toml dependency or a feature not named devnet turns it on; when a
Capsule.mk names it outside CAPSULE_DEV_CARGO_FEATURES, or a catalogue
outside capsules-dev.json's dev_cargo_features; or when nonos.toml, the
Makefile, mk, nonos-mk, nonos-ci, .github or tools/nix name it, but for the
strings the release ELF check looks for. That a release never takes
capsules-dev is scripts/check_dev_capsule_set.py's.
    scripts/check_devnet_feature.py [--self-test] [--root DIR]
"""

import argparse
import sys
from pathlib import Path

from devnet_feature_selftest import self_test
from devnet_feature_tree import failures


def main():
    ap = argparse.ArgumentParser(description=__doc__, formatter_class=argparse.RawDescriptionHelpFormatter)
    ap.add_argument("--self-test", action="store_true")
    ap.add_argument("--root", default=".")
    args = ap.parse_args()
    if args.self_test:
        return self_test()
    failed = failures(Path(args.root))
    for line in failed:
        print(f"devnet-feature: {line}")
    if not failed:
        print("devnet-feature: devnet is a development capsule feature alone; no release can turn it on")
    return 1 if failed else 0


if __name__ == "__main__":
    sys.exit(main())
