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
"""check_copy_only_features' self-test: a clean tree passes; each way a
feature could reach a committed build is caught."""

import json
import shutil
import tempfile
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent
FILES = ["tools/nix/capsules.json", "userland/capsule_linux/Cargo.toml", "tools/nonos-dev-image",
         "userland/capsule_linux/Capsule.mk", "userland/capsule_terminal/Cargo.toml"]


def tree(tmp: Path) -> Path:
    for f in FILES:
        (tmp / f).parent.mkdir(parents=True, exist_ok=True)
        shutil.copyfile(ROOT / f, tmp / f)
    return tmp


def edit(tmp: Path, f: str, old: str, new: str):
    s = (tmp / f).read_text()
    assert old in s, (f, old)
    (tmp / f).write_text(s.replace(old, new, 1))


def json_on(tmp: Path, slug: str, feature: str):
    entries = json.loads((p := tmp / "tools/nix/capsules.json").read_text())
    next(e for e in entries if e["slug"] == slug)["cargo_features"] = feature
    p.write_text(json.dumps(entries))


MUTANTS = {
    "json": lambda t: json_on(t, "linux", "selftest-panic"),
    "capsules.json headless": lambda t: json_on(t, "terminal", "agentproof-headless"),
    "Capsule.mk": lambda t: edit(t, "userland/capsule_linux/Capsule.mk", "include nonos-mk",
                                 "CAPSULE_CARGO_FEATURES := selftest-panic\ninclude nonos-mk"),
    "default": lambda t: edit(t, "userland/capsule_linux/Cargo.toml", "[features]\n",
                              '[features]\ndefault = ["selftest-panic"]\n'),
    "ungated": lambda t: edit(t, "tools/nonos-dev-image", 'if a.profile == "selftest":', "if 1:"),
    "headless default": lambda t: edit(t, "userland/capsule_terminal/Cargo.toml", "[features]\n",
                                       '[features]\ndefault = ["agentproof-headless"]\n'),
    "headless ungated": lambda t: edit(t, "tools/nonos-dev-image", "if a.agentproof_job:", "if 1:"),
}


def self_test(problems) -> int:
    bad = 0
    with tempfile.TemporaryDirectory() as d:
        if problems(tree(Path(d))):
            print("copy-only self-test: the clean tree is refused")
            bad += 1
    for name, mutate in MUTANTS.items():
        with tempfile.TemporaryDirectory() as d:
            mutate(t := tree(Path(d)))
            caught = bool(problems(t))
            print(f"copy-only self-test: {name}: {'caught' if caught else 'ESCAPED'}")
            bad += not caught
    return 1 if bad else 0
