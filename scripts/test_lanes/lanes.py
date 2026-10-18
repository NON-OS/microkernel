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
"""Which crates ask for a lane: the *_proofs rule tools/nix/checks.nix keeps,
and the marker a crate's manifest carries."""

import posixpath

from .tree import join


def marker(tree, d):
    return tree.manifests[d].get("package", {}).get("metadata", {}).get("nonos", {})


def proofs(tree, d):
    return (posixpath.dirname(d) == "userland" and d.endswith("_proofs")
            and join(d, "Cargo.lock") in tree.files)


def marked(tree):
    """dir -> test-args of every crate whose manifest asks for the lane."""
    return {d: marker(tree, d).get("test-args", "") for d in tree.crates
            if marker(tree, d).get("test-lane") is True}


def misuse(tree):
    """A marker the lane cannot act on as written."""
    for d in tree.crates:
        m = marker(tree, d)
        on = m.get("test-lane", False)
        why = ("test-lane must be true or false" if type(on) is not bool else
               "test-args without test-lane = true" if not on and "test-args" in m else
               None if not on else
               "test-args must be one string of cargo test arguments"
               if not isinstance(m.get("test-args", ""), str) else
               "a *_proofs crate is in the lane by its name; drop its marker" if proofs(tree, d) else
               None if join(d, "Cargo.lock") in tree.files else
               "marked, but the lane runs cargo test --frozen against the crate's own "
               "Cargo.lock and it has none; mark its workspace root with --workspace")
        if why:
            yield f"{d}: {why}"
