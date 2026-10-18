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
"""A crate's cargo targets, found as cargo finds them: the manifest's own
entries over what autodiscovery adds."""

from collections import namedtuple

from .tree import join

Target = namedtuple("Target", "kind name path test")


def discovered(tree, d, sub):
    """name -> file for `sub/*.rs` and `sub/*/main.rs` under crate d."""
    base = tree.root / d / sub
    out = {}
    for e in sorted(base.iterdir()) if base.is_dir() else []:
        f = join(d, f"{sub}/{e.name}")
        if e.name.endswith(".rs") and f in tree.files:
            out[e.name[:-3]] = f
        elif f"{f}/main.rs" in tree.files:
            out[e.name] = f"{f}/main.rs"
    return out


def declared(tree, d, kind, auto):
    """The [[bin]] or [[test]] targets: autodiscovery, then the manifest's own
    entries in place of a discovered one of the same name or file."""
    sub = "src/bin" if kind == "bin" else "tests"
    out = {n: (f, True) for n, f in auto.items()}
    for t in tree.manifests[d].get(kind, []):
        name = t.get("name", "")
        guesses = [f"{sub}/{name}.rs", f"{sub}/{name}/main.rs"] + ["src/main.rs"] * (kind == "bin")
        f = join(d, t["path"]) if "path" in t else next(
            (join(d, g) for g in guesses if join(d, g) in tree.files), "")
        out = {n: v for n, v in out.items() if v[0] != f and n != name}
        out[name] = (f, t.get("test", True))
    return out


def targets(tree, d):
    """Every lib, bin and test target of crate d whose root file is in the tree."""
    m, pkg = tree.manifests[d], tree.manifests[d].get("package", {})
    found = []
    lib = m.get("lib", {})
    if join(d, lib.get("path", "src/lib.rs")) in tree.files:
        found.append(Target("lib", "", join(d, lib.get("path", "src/lib.rs")), lib.get("test", True)))
    bins = {}
    if pkg.get("autobins", True):
        bins = discovered(tree, d, "src/bin")
        if join(d, "src/main.rs") in tree.files:
            bins[pkg.get("name", "")] = join(d, "src/main.rs")
    tests = discovered(tree, d, "tests") if pkg.get("autotests", True) else {}
    for kind, auto in (("bin", bins), ("test", tests)):
        for name, (f, test) in declared(tree, d, kind, auto).items():
            if f in tree.files:
                found.append(Target(kind, name, f, test))
    return found
