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
#[path] or an include! reaches from it, as rustc resolves them.

A file is read with the module directory it sits in: `dir`, and `stem` for
a plain x.rs, whose children live in x/ (rustc's relative offset). An inline
`mod m { .. }` moves its body to dir/stem/m, or, under #[path = "p"], to
dir/p with the offset dropped; a #[path] on `mod x;` is taken from dir.
rustc reads a #[path] file, and an include!d one whatever module includes
it, as a mod.rs in its own directory. A `mod x;` with a #[cfg_attr(.., path
= ..)] reaches each file it names, and its own x.rs too when no plain
#[path] overrides that: the walk does not judge cfgs."""

import posixpath
import re

from .tree import join

# Comments and literals first, so a brace or a `mod` inside one is skipped.
# A comment between attributes ends at its newline, so matching stays linear.
TOKEN = re.compile(
    r"//[^\n]*|/\*.*?\*/|\bb?r(#*)\".*?\"\1|\"(?:\\.|[^\"\\])*\"|'(?:\\.|[^'\\\n])'"
    r'|\binclude!\s*\(\s*"(?P<inc>[^"]+\.rs)"\s*\)'
    r"|(?P<attrs>(?:#\[[^\]]*\]\s*(?://[^\n]*\n\s*)*)*)(?:\bpub(?:\([^)]*\))?\s+)?\bmod\s+(?:r#)?"
    r"(?P<name>\w+)\s*(?P<end>[;{])|(?P<open>\{)|(?P<close>\})", re.S)
PATH = re.compile(r'#\[[ \t]*(cfg_attr[ \t]*\(.*?,[ \t]*)?path[ \t]*=[ \t]*"([^"]+)"')


def alone(f):
    """f, read as a mod.rs in its own directory."""
    return (f, (posixpath.dirname(f) or ".", None))


def inside(at, name, path):
    """The module directory of an inline `mod name { .. }` read at `at`."""
    d, stem = at
    if path:
        return (join(d, path), None)
    return (join(join(d, stem) if stem else d, name), None)


def beside(tree, at, name):
    """The file of a `mod name;` with no #[path]: name.rs, else name/mod.rs."""
    d, stem = at
    base = join(d, stem) if stem else d
    flat = join(base, f"{name}.rs")
    if flat in tree.files:
        return (flat, (base, name))
    return (join(base, f"{name}/mod.rs"), (join(base, name), None))


def children(tree, f, at):
    """(file, directory) for each file f's declarations and include!s reach."""
    if (f, at) not in tree.edges:
        here, kids, stack = posixpath.dirname(f) or ".", [], [at]
        for m in TOKEN.finditer(tree.text(f)):
            cur = stack[-1]
            if m["inc"]:
                kids.append(alone(join(here, m["inc"])))
            elif m["open"]:
                stack.append(cur)
            elif m["close"]:
                stack.pop() if len(stack) > 1 else None
            elif m["end"]:
                paths = PATH.findall(m["attrs"])
                plain = [p for cfg, p in paths if not cfg]
                if m["end"] == "{":
                    stack.append(inside(cur, m["name"], plain[0] if plain else None))
                    continue
                kids += [alone(join(cur[0], p)) for _, p in paths]
                kids += [] if plain else [beside(tree, cur, m["name"])]
        tree.edges[(f, at)] = kids
    return tree.edges[(f, at)]


def reach(tree, roots):
    """Every file in the tree the roots' module trees reach."""
    seen, queue = set(), [alone(r) for r in roots]
    while queue:
        f, at = queue.pop()
        if (f, at) in seen or f not in tree.files:
            continue
        seen.add((f, at))
        queue.extend(children(tree, f, at))
    return {f for f, _ in seen}
