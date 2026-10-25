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
"""The self-test of check_selftest_panic: a clean tree passes, and each way
the feature could reach a committed build is caught."""

import json
import shutil
import tempfile
from importlib.machinery import SourceFileLoader
from importlib.util import module_from_spec, spec_from_loader
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent
FILES = ["tools/nix/capsules.json", "userland/capsule_linux/Cargo.toml",
         "userland/capsule_linux/Capsule.mk", "tools/nonos-dev-image"]


def tree(tmp: Path) -> Path:
    for f in FILES:
        (tmp / f).parent.mkdir(parents=True, exist_ok=True)
        shutil.copyfile(ROOT / f, tmp / f)
    return tmp


def edit(tmp: Path, f: str, old: str, new: str):
    p = tmp / f
    s = p.read_text()
    assert old in s, (f, old)
    p.write_text(s.replace(old, new, 1))


def json_on(tmp: Path):
    p = tmp / "tools/nix/capsules.json"
    entries = json.loads(p.read_text())
    next(e for e in entries if e["slug"] == "linux")["cargo_features"] = "selftest-panic"
    p.write_text(json.dumps(entries))


MUTANTS = {
    "capsules.json": json_on,
    "Capsule.mk": lambda t: edit(t, "userland/capsule_linux/Capsule.mk", "include nonos-mk",
                                 "CAPSULE_CARGO_FEATURES := selftest-panic\ninclude nonos-mk"),
    "default": lambda t: edit(t, "userland/capsule_linux/Cargo.toml", "[features]\n",
                              '[features]\ndefault = ["selftest-panic"]\n'),
    "ungated": lambda t: edit(t, "tools/nonos-dev-image", 'if a.profile == "selftest":',
                              "if True:"),
}


def keeps_features() -> bool:
    """The selftest copy adds selftest-panic to capsule_linux's cargo_features
    and drops none it has: a release feature, or one another row gave it."""
    loader = SourceFileLoader("nonos_dev_image", str(ROOT / "tools/nonos-dev-image"))
    tool = module_from_spec(spec_from_loader(loader.name, loader))
    loader.exec_module(tool)
    entries = [{"slug": "linux", "cargo_features": "dev-mirror"}, {"slug": "hello", "cargo_features": ""}]
    once = tool.panicking(json.loads(json.dumps(entries)))
    twice = tool.panicking(json.loads(json.dumps(once)))
    bare = tool.panicking([{"slug": "linux", "cargo_features": ""}])
    return (once == twice == [{"slug": "linux", "cargo_features": "dev-mirror,selftest-panic"},
                              {"slug": "hello", "cargo_features": ""}]
            and bare == [{"slug": "linux", "cargo_features": "selftest-panic"}])


def self_test(problems) -> int:
    bad = 0
    kept = keeps_features()
    print(f"selftest-panic self-test: a feature capsule_linux has: {'kept' if kept else 'DROPPED'}")
    bad += not kept
    with tempfile.TemporaryDirectory() as d:
        if problems(tree(Path(d))):
            print("selftest-panic self-test: the clean tree is refused")
            bad += 1
    for name, mutate in MUTANTS.items():
        with tempfile.TemporaryDirectory() as d:
            t = tree(Path(d))
            mutate(t)
            caught = bool(problems(t))
            print(f"selftest-panic self-test: {name}: {'caught' if caught else 'ESCAPED'}")
            bad += not caught
    return 1 if bad else 0
