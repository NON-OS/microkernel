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
"""The files a target compiles: its root and every file a `mod x;`, a
#[path] or an include! reaches from it, as rustc resolves them."""

import posixpath
import re

from .tree import join

DECL = re.compile(r"[ \t]*((?:#\[[^\]]*\][ \t]*)*)(?:pub(?:\([^)]*\))?[ \t]+)?mod[ \t]+(?:r#)?(\w+)[ \t]*;")
PATH = re.compile(r'#\[[ \t]*(?:cfg_attr[ \t]*\(.*?,[ \t]*)?path[ \t]*=[ \t]*"([^"]+)"')
INCLUDE = re.compile(r'\binclude!\s*\(\s*"([^"]+\.rs)"\s*\)')


def declared(text):
    """(#[path] or None, name) for each `mod name;`. Its attributes are those on
    its own line and on the attribute lines straight above it, comments between."""
    lines = text.split("\n")
    out = []
    for n, line in enumerate(lines):
        m = DECL.match(line) if "mod" in line else None
        if not m:
            continue
        attrs, k = m[1], n
        while k > 0 and lines[k - 1].lstrip().startswith(("#[", "//")):
            k -= 1
            attrs += "" if lines[k].lstrip().startswith("//") else lines[k]
        p = PATH.search(attrs)
        out.append((p[1] if p else None, m[2]))
    return out


def children(tree, f, owns):
    """The files f's declarations name. A crate root, a mod.rs or a #[path]
    file holds its children beside it; any other x.rs holds them in x/."""
    if (f, owns) not in tree.edges:
        here = posixpath.dirname(f) or "."
        base = here if owns else join(here, posixpath.splitext(posixpath.basename(f))[0])
        kids = []
        text = tree.text(f)
        for path, name in declared(text):
            if path:
                kids.append((join(here, path), True))
                continue
            flat = join(base, f"{name}.rs")
            kids.append((flat, False) if flat in tree.files else (join(base, f"{name}/mod.rs"), True))
        kids += [(join(here, p), owns) for p in ("include!" in text and INCLUDE.findall(text) or [])]
        tree.edges[(f, owns)] = kids
    return tree.edges[(f, owns)]


def reach(tree, roots):
    """Every file in the tree the roots' module trees reach."""
    seen, queue = set(), [(r, True) for r in roots]
    while queue:
        f, owns = queue.pop()
        if (f, owns) in seen or f not in tree.files:
            continue
        seen.add((f, owns))
        queue.extend(children(tree, f, owns))
    return {f for f, _ in seen}
