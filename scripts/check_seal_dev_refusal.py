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
"""A release seal handed the development capsule set refuses it, whatever capsules.DEVELOPMENT
says (tools/nonos_seal/release_set.py): the cases, then the guard with its refusal taken out,
keyed on the flag, or blind to a capsule built with dev features, each of which must fail one."""

import importlib
import sys
import tempfile
import types
from pathlib import Path

GUARD = Path(__file__).resolve().parents[1] / "tools/nonos_seal/release_set.py"
MUTANTS = {"refusal removed": ("    if found:\n", "    if False:\n"),
           "keyed on the flag": ("    if not release:\n", "    if not release or capsules.DEVELOPMENT:\n"),
           "blind to a -dev capsule": ('== f"nonos-capsule-{slug}-dev"', "is None")}
REL = {"hello": "nonos-capsule-hello", "shield": "nonos-capsule-shield"}
DEV = dict(REL, hello="nonos-capsule-hello-dev")
# (release, capsules.DEVELOPMENT, set, members, refused): a dev set is refused by its name alone.
CASES = [(True, False, "nonos-capsules", REL, False), (True, False, "nonos-capsules-dev", REL, True),
         (True, False, "nonos-capsules-dev", DEV, True), (True, False, "nonos-capsules", DEV, True),
         (True, True, "nonos-capsules-dev", DEV, True), (True, True, "nonos-capsules", REL, False),
         (False, True, "nonos-capsules-dev", DEV, False)]


def refuses(guard, capsules, case):
    """Whether the guard refuses one case, on a link farm as linkFarm makes it: <hash>-<name>/<slug>."""
    release, capsules.DEVELOPMENT, name, members, _ = case
    with tempfile.TemporaryDirectory() as tmp:
        farm = Path(tmp) / f"{'0' * 32}-{name}"
        farm.mkdir()
        for slug, capsule in members.items():
            (farm / slug).symlink_to(Path(tmp) / f"{'f' * 32}-{capsule}")
        try:
            guard.refuse(str(farm), release)
        except SystemExit:
            return True
    return False


def wrong(guard, capsules):
    return [n for n, c in enumerate(CASES) if refuses(guard, capsules, c) != c[-1]]


def mutant(capsules, old, new):
    """The guard with one change; a change that no longer applies leaves it whole, and fails as a mutant."""
    ns = {"capsules": capsules, "__name__": "release_set_mutant", "__package__": "tools.nonos_seal"}
    exec(compile(GUARD.read_text().replace(old, new), str(GUARD), "exec"), ns)
    return types.SimpleNamespace(**ns)


def main():
    sys.path.insert(0, str(GUARD.parents[2]))
    capsules = importlib.import_module("tools.nonos_seal.capsules")
    bad = wrong(importlib.import_module("tools.nonos_seal.release_set"), capsules)
    failed = ([f"the guard gets cases {bad} wrong"] if bad else []) + [f"mutant '{m}' passes every case"
              for m, (old, new) in MUTANTS.items() if not wrong(mutant(capsules, old, new), capsules)]
    print("\n".join(f"seal-dev-refusal: {line}" for line in failed) or f"seal-dev-refusal: a release refuses "
          f"the development set in all {len(CASES)} cases, and each of {len(MUTANTS)} guard mutants fails one")
    return 1 if failed else 0


if __name__ == "__main__":
    sys.exit(main())
