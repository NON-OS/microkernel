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
"""Keep the development catalogue out of every release.

tools/devnet/market/index.json is a schema 3 test catalogue with priced
listings under placeholder payees. Only a development seal in
nonos-dev-image's copy encodes it, signed by that copy's throwaway operator
key, which a release market does not trust (nonos-mk/tests/dev_catalogue_*).
This fails when a release path names it (mk, nonos-mk, tools/nix, the seal
but for its one gate, the market capsules, nonos-data), when a line that
keeps a release from choosing it changes, when the seal's choice no longer
behaves so when run, or when the catalogue stops being placeholders.

    scripts/check_dev_catalogue.py [--self-test] [--root DIR]
"""

import argparse
import json
import subprocess
import sys
from pathlib import Path

import dev_catalogue_scan as scan
from dev_catalogue_behaviour import behaviour
from dev_catalogue_content import SOURCE, catalogue, load
import dev_catalogue_chain as chain


def failures(root):
    listed = subprocess.run(["git", "-C", str(root), "ls-files", "-z"], capture_output=True, check=True)
    out = []
    for path in sorted(p for p in listed.stdout.decode().split("\0") if p and scan.watched(p)):
        if (root / path).is_file() and not path.endswith((".bin", ".png", ".jpg", ".gz", ".xz")):
            out += scan.named(path, (root / path).read_text(errors="replace"))
    out += scan.pinned({p: (root / p).read_text() for p, _ in scan.PINS if (root / p).is_file()})
    out += catalogue(json.loads((root / SOURCE).read_text()))
    out += chain.dev_chains(json.loads((root / SOURCE).read_text())) + chain.release_catalogues(root)
    return out + behaviour(*load(root))


def main():
    ap = argparse.ArgumentParser(description=__doc__, formatter_class=argparse.RawDescriptionHelpFormatter)
    ap.add_argument("--self-test", action="store_true")
    ap.add_argument("--root", default=".")
    args = ap.parse_args()
    root = Path(args.root).resolve()
    if args.self_test:
        from dev_catalogue_selftest import self_test
        return self_test(root)
    failed = failures(root)
    for line in failed:
        print(f"dev-catalogue: {line}")
    if not failed:
        print("dev-catalogue: only a development seal in nonos-dev-image's copy takes the test catalogue")
    return 1 if failed else 0


if __name__ == "__main__":
    sys.exit(main())
