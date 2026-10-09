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

"""Writes, or checks, tools/nix/capsules.json and tools/nix/store.json from
what make prints about the capsules and the store (mk/60-nix.mk).

    python3 tools/nix/catalogues.py          regenerate both, from make
    python3 tools/nix/catalogues.py --check  fail if either is stale

The check reads make's output from $TMPDIR/{capsule,store}.json, which the
flake's `catalogues` check writes first."""

import argparse
import json
import os
import subprocess
import sys

HERE = os.path.dirname(os.path.abspath(__file__))


def from_make(which):
    out = subprocess.run(["make", "-s", "--no-print-directory", "NONOS_IN_FLAKE=1", "NONOS_QUIET=1",
                          f"nonos-mk-{which}-catalogue"], capture_output=True, text=True, check=True)
    return json.loads(out.stdout)


def shape(which, raw):
    if which == "capsule":
        return raw
    def files(entries):
        return [dict(zip(("path", "file"), e.split("=", 1))) for e in entries.split()]
    # The groups, then the Linux packages: one list of files per package.
    return {group: ({name: files(e) for name, e in entries.items()} if isinstance(entries, dict) else files(entries))
            for group, entries in raw.items()}


def text(data):
    return json.dumps(data, indent=1) + "\n"


def main():
    ap = argparse.ArgumentParser(description=__doc__, formatter_class=argparse.RawDescriptionHelpFormatter)
    ap.add_argument("--check", action="store_true")
    args = ap.parse_args()
    stale = []
    for which, name in (("capsule", "capsules.json"), ("store", "store.json")):
        if args.check:
            with open(os.path.join(os.environ["TMPDIR"], f"{which}.json")) as f:
                raw = json.load(f)
        else:
            raw = from_make(which)
        path = os.path.join(HERE, name)
        want = text(shape(which, raw))
        if args.check:
            with open(path) as f:
                if f.read() != want:
                    stale.append(f"tools/nix/{name}")
        else:
            with open(path, "w") as f:
                f.write(want)
    if stale:
        sys.exit("stale: " + ", ".join(stale) + "; regenerate with python3 tools/nix/catalogues.py")


if __name__ == "__main__":
    main()
