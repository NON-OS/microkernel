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
"""The tree as the flake copies it: its files, its crates, and which crate
owns each Rust file."""

import os
import posixpath
import subprocess
import tomllib


def join(d, p):
    return posixpath.normpath(p if d == "." else posixpath.join(d, p))


def listed(root):
    """Tracked files and new ones git does not ignore: what `git add -A` hands the flake."""
    out = subprocess.run(["git", "-C", str(root), "ls-files", "-z", "--cached", "--others",
                          "--exclude-standard"], check=True, capture_output=True).stdout.decode()
    return {f for f in out.split("\0") if f and os.path.isfile(os.path.join(root, f))}


class Tree:
    def __init__(self, root):
        self.root = root
        self.files = listed(root)
        self.texts, self.edges = {}, {}
        self.manifests = {posixpath.dirname(f) or ".": self.load(f) for f in self.files
                          if posixpath.basename(f) == "Cargo.toml"}
        self.crates = sorted(d for d, m in self.manifests.items() if "package" in m)
        self.owned = {}
        for f in sorted(f for f in self.files if f.endswith(".rs")):
            self.owned.setdefault(self.owner(f), []).append(f)

    def load(self, f):
        try:
            return tomllib.loads(self.text(f))
        except tomllib.TOMLDecodeError:
            return {}

    def owner(self, f):
        """The nearest directory above f that holds a Cargo.toml."""
        d = posixpath.dirname(f)
        while d and d not in self.manifests:
            d = posixpath.dirname(d)
        return d or "."

    def text(self, f):
        if f not in self.texts:
            with open(os.path.join(self.root, f), errors="replace") as fh:
                self.texts[f] = fh.read()
        return self.texts[f]
