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
"""The crates a crate builds by path, and the features each is built with:
its dependencies' and build-dependencies' (never its dev-dependencies'),
the default set unless an edge turns it off, closed over each feature table."""

import tomllib
from pathlib import Path


def manifest(crate: Path) -> dict:
    return tomllib.loads((crate / "Cargo.toml").read_text())


def expand(m: dict, wanted: set) -> tuple:
    """`wanted` closed over the feature table; plus `dep/feature` asks."""
    table, own, deps, todo = m.get("features", {}), set(), {}, list(wanted)
    while todo:
        f = todo.pop()
        if "/" in f:
            dep, sub = f.split("/", 1)
            deps.setdefault(dep.rstrip("?"), set()).add(sub)
        elif not f.startswith("dep:") and f not in own:
            own.add(f)
            todo += table.get(f, [])
    return own, deps


def enabled(root: Path) -> dict:
    """Every crate `root` builds by path (not its dev-dependencies), with the
    features it is built with, `root` included."""
    out, todo = {root.resolve(): {"default"}}, [root.resolve()]
    while todo:
        crate = todo.pop()
        m = manifest(crate)
        own, asked = expand(m, out[crate])
        out[crate] = own
        for name, spec in {**m.get("dependencies", {}), **m.get("build-dependencies", {})}.items():
            if not isinstance(spec, dict) or "path" not in spec:
                continue
            dep = (crate / spec["path"]).resolve()
            want = set(spec.get("features", [])) | asked.get(name, set())
            if spec.get("default-features", True):
                want.add("default")
            if dep not in out or not want <= out[dep]:
                out[dep] = out.get(dep, set()) | want
                todo.append(dep)
    return out
