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
from importlib.machinery import SourceFileLoader
from importlib.util import module_from_spec, spec_from_loader
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
            print("copy-only self-test: the clean tree is refused")
            bad += 1
    for name, mutate in MUTANTS.items():
        with tempfile.TemporaryDirectory() as d:
            mutate(t := tree(Path(d)))
            caught = bool(problems(t))
            print(f"copy-only self-test: {name}: {'caught' if caught else 'ESCAPED'}")
            bad += not caught
    return 1 if bad else 0
