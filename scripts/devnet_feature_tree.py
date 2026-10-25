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
"""The tree as scripts/check_devnet_feature.py reads it: which files each
rule of devnet_feature_scan.py applies to, and the lines that must hold."""

import os
from pathlib import Path

import devnet_feature_lanes as lanes
import devnet_feature_scan as scan

# The release ELF check still looks for the throwaway TLS root's name. Where
# a release could pick the development set is scripts/check_dev_capsule_set.py's
# to pin, not this check's.
PINS = {"tools/nix/release_names.nix": '\n    "NONOS-devnet-throwaway-root"\n'}


def pinned(files):
    """Each pinned line still holds."""
    return [f"{p}: no longer holds: {want}" for p, want in PINS.items() if want not in files.get(p, "")]


PLACES = ("nonos.toml", "Makefile", "mk", "nonos-mk", "nonos-ci", ".github", "tools/nix")
SUFFIXES = {".toml", ".nix", ".mk", ".sh", ".yml", ".yaml", ".json", ".py"}
# The catalogues, by the keys that may name devnet: capsules-dev.json's
# features alone, which only capsules.nix's bySlugDev reads.
CATALOGUES = {"tools/nix/capsules.json": (), "tools/nix/capsules-dev.json": ("dev_cargo_features",)}
# The source files each crate reads (tools/nix/inputs.py): paths, never a feature.
FILE_LISTS = {"tools/nix/inputs.json"}


def walk(root, top):
    """Each file under `top` that can name a feature, build output left out."""
    base = root / top
    paths = [base] if base.is_file() else []
    for d, subs, names in os.walk(base):
        subs[:] = sorted(s for s in subs if s not in ("target", ".git"))
        paths += [Path(d) / n for n in sorted(names)]
    for p in paths:
        if p.suffix in SUFFIXES or p.name in ("Makefile", "Cargo.toml"):
            yield p.relative_to(root).as_posix(), p.read_text(errors="replace")


def failures(root):
    out = []
    for path, text in walk(root, "Cargo.toml"):
        out += scan.cargo(path, text)
    for path, text in walk(root, "userland"):
        if path.endswith("Cargo.toml"):
            out += scan.cargo(path, text)
        elif path.endswith(".mk"):
            out += scan.capsule_mk(path, text)
    for top in PLACES:
        for path, text in walk(root, top):
            if path in FILE_LISTS:
                continue
            if path == lanes.LANES:
                out += lanes.lanes(path, text, lanes.capsule_dirs(root))
                continue
            out += scan.catalogue(path, text, CATALOGUES[path]) if path in CATALOGUES else scan.anywhere(path, text)
    pins = {p: (root / p).read_text() for p in PINS if (root / p).is_file()}
    return out + pinned(pins)
