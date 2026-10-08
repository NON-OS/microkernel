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

"""Writes nonos-build.json: the configuration a build resolved and the sha256
of every artifact it produced, in a fixed order, so two builds that agree byte
for byte write the same manifest."""

import argparse
import hashlib
import json
import os
import sys

BOUNDARY = (
    "These files are reproducible: the same commit and nonos.toml give the same "
    "bytes on any machine. Enrollment and signing are not: the seal draws fresh "
    "randomness for every STARK proof and signs with keys this build never sees."
)


def sha256(path):
    h = hashlib.sha256()
    with open(path, "rb") as f:
        for block in iter(lambda: f.read(1 << 20), b""):
            h.update(block)
    return h.hexdigest()


def files(root):
    found = []
    for base, dirs, names in os.walk(root):
        dirs.sort()
        for name in sorted(names):
            path = os.path.join(base, name)
            rel = os.path.relpath(path, root)
            if rel != "nonos-build.json":
                found.append(rel)
    return found


def main():
    ap = argparse.ArgumentParser(description=__doc__)
    ap.add_argument("root", help="the artifact tree")
    ap.add_argument("config", help="the resolved configuration, as JSON")
    ap.add_argument("--host", help="per-host facts (toolchain store paths), as "
                    "JSON; recorded beside config but not part of it, so the "
                    "reproducibility compare ignores them", default=None)
    args = ap.parse_args()
    with open(args.config) as f:
        config = json.load(f)
    manifest = {
        "format": "nonos-build/1",
        "boundary": BOUNDARY,
        "config": config,
        "artifacts": {rel: sha256(os.path.join(args.root, rel)) for rel in files(args.root)},
    }
    # The per-host facts live at the top level, outside `config`, so two
    # machines that build the same bytes still compare equal even though their
    # toolchains sit at different store paths (nonos-verify reproducible
    # compares `config` and `artifacts`, nothing else).
    if args.host:
        with open(args.host) as f:
            manifest["host"] = json.load(f)
    json.dump(manifest, sys.stdout, indent=1, sort_keys=True)
    sys.stdout.write("\n")


if __name__ == "__main__":
    main()
