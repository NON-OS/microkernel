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
"""The one place under tools/nix a test may name devnet: a feature lane in
tools/nix/test-lanes.json, which runs a library's tests with the feature on.
cargo test builds nothing an image takes, so the lane ships nothing; but a
capsule's lane is refused all the same, so the only crates that ever build
with devnet on are libraries under test and the development capsule set."""

import json
import os

from devnet_feature_scan import WORD

LANES = "tools/nix/test-lanes.json"
# What scripts/test_lanes writes for a manifest's feature-lanes = ["devnet"].
SUFFIX, ARGS = "+devnet", "--features devnet"


def capsule_dirs(root):
    """Every capsule's directory: the catalogue's, and each with a Capsule.mk."""
    path = root / "tools/nix/capsules.json"
    dirs = {e["dir"] for e in json.loads(path.read_text())} if path.is_file() else set()
    base = root / "userland"
    if base.is_dir():
        dirs |= {f"userland/{d}" for d in os.listdir(base) if (base / d / "Capsule.mk").is_file()}
    return dirs


def in_capsule(d, capsules):
    return any(d == c or d.startswith(c + "/") for c in capsules)


def lanes(path, text, capsules):
    """A lane names devnet only as a non-capsule crate's devnet feature lane."""
    try:
        table = json.loads(text)
    except ValueError:
        return [f"{path}: not JSON, so its lanes cannot be read"]
    out = []
    for key, args in sorted(table.items()):
        if not (WORD.search(key) or WORD.search(str(args))):
            continue
        crate = key[: -len(SUFFIX)] if key.endswith(SUFFIX) else None
        rest = str(args)[: -len(ARGS)] if str(args).endswith(ARGS) else None
        if crate is None or rest is None or WORD.search(crate) or WORD.search(rest):
            out.append(f"{path}: {key} names devnet outside a devnet feature lane")
        elif in_capsule(crate, capsules):
            out.append(f"{path}: {key} is a capsule's lane; only a library's tests take devnet")
    return out
