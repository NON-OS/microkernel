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
"""What a lane runs: its crates, the workspace members a root's --workspace
or -p takes along, and the targets its arguments select."""

import posixpath
import shlex
from fnmatch import fnmatch

from .lanes import marked, proofs
from .tree import join

PICKS = ("--lib", "--bins", "--bin", "--tests", "--test", "--all-targets",
         "--examples", "--example", "--benches", "--bench", "--doc")


def words(args):
    return [w for a in shlex.split(args) for w in a.split("=", 1)]


def packages(args):
    w = words(args)
    return {b for a, b in zip(w, w[1:]) if a in ("-p", "--package")}


def lanes(tree):
    """dir -> cargo test arguments for every crate a lane runs, the members a
    workspace root's --workspace or -p runs included."""
    out = {d: "" for d in tree.crates if proofs(tree, d)}
    out.update((d, a) for d, a in marked(tree).items()
               if isinstance(a, str) and not proofs(tree, d) and join(d, "Cargo.lock") in tree.files)
    for root, args in sorted(out.items()):
        wanted = packages(args)
        ws = tree.manifests[root].get("workspace")
        if not ws or not (wanted or {"--workspace", "--all"} & set(words(args))):
            continue
        for d in tree.crates:
            rel = posixpath.relpath(d, root)
            if any(fnmatch(rel, p) for p in ws.get("members", [])) \
                    and not any(fnmatch(rel, p) for p in ws.get("exclude", [])) \
                    and (not wanted or tree.manifests[d]["package"].get("name") in wanted):
                out.setdefault(d, args)
    return out


def selected(args, name, targets):
    """The targets of package `name` that `cargo test <args>` runs."""
    w, wanted = words(args), packages(args)
    if wanted and name not in wanted:
        return []
    picks = [(a, b) for a, b in zip(w, w[1:] + [""]) if a in PICKS]
    if not picks or {"--tests", "--all-targets"} & {a for a, _ in picks}:
        return [t for t in targets if t.test]
    return [t for t in targets if any(
        (a, t.kind) in (("--lib", "lib"), ("--bins", "bin")) or a == f"--{t.kind}" and fnmatch(t.name, b)
        for a, b in picks)]
