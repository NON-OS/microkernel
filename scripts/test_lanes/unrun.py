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
"""What no lane runs: each crate's tests set against the files every lane's
selected targets compile."""

import re

from .modules import reach
from .runs import lanes, selected
from .targets import targets
from .tree import join

# A test fn or a property block, at a line start: a doc comment quoting one is not.
TEST = re.compile(r"^[ \t]*#\[\s*(?:tokio::|async_std::)?test\b|\bproptest!\s*[{(]", re.M)
# A test program under tests/: a harness = false target, or a host program.
MAIN = re.compile(r"^[ \t]*(?:pub\s+)?fn\s+main\s*\(", re.M)


def bearing(tree, f, crate):
    """f holds a test: a test fn anywhere, or a program under the crate's tests/."""
    text = tree.text(f)
    return bool("test" in text and TEST.search(text)
                or f.startswith(join(crate, "tests") + "/") and MAIN.search(text))


def unrun(tree):
    """The lane map, and {path: (kind, crate, files)} for every test no lane runs.
    A test a lane reaches through a #[path] mount counts as run."""
    lane, full, ran = lanes(tree), {}, set()
    for d in tree.crates:
        tests = [f for f in tree.owned.get(d, ()) if bearing(tree, f, d)]
        if d in lane or tests:
            tg = targets(tree, d)
            full[d] = (tests, reach(tree, [t.path for t in tg]))
            if d in lane:
                name = tree.manifests[d]["package"].get("name")
                ran |= reach(tree, [t.path for t in selected(lane[d], name, tg)])
    found = {}
    for d, (tests, inside) in full.items():
        left = [f for f in tests if f not in ran]
        if d in lane:
            found.update((f, ("dropped" if f in inside else "orphan", d, [f])) for f in left)
        elif [f for f in left if f in inside or f.startswith(join(d, "tests") + "/")]:
            found[d] = ("crate", d, left)
    return lane, found
