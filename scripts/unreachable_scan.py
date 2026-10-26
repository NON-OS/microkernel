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
"""The scan behind check_unreachable.py: exported functions nobody calls.

A `pub fn` in a library crate is never dead to rustc, because something
outside the crate might call it. Nothing outside this crate ever will: the
kernel is the top of its own tree. So the dead-code lint is silent across the
whole public surface, and a mechanism can be written, reviewed, merged and
shipped without one line of it ever running.

Two things decide whether a name counts as called, and both were wrong in the
first cut of this file. A `pub use` only carries a name up the module tree, so
an import is not a call: the KASLR validator was re-exported through three
levels of mod.rs and invoked from nowhere. And a definition mentions its own
name, so counting that line makes every function look used.

Definitions in architectures the build is not targeting are skipped. Their
code does not compile into anything, so "nothing calls it" says nothing about
it. Their calls still count: a shared function that only the aarch64 DMA
backend calls is called, on aarch64, and moving that backend under src/arch
must not make it read as dead.

Three more limits, each a site read wrongly. An import is every line of it:
the names on the continuation lines of a multi-line `pub use` were counted as
calls, so a function only ever re-exported through one read as live. A `use`
that renames carries the call: `brightness as brightness_key` is called where
brightness_key is. And a crate outside src/ that mounts a kernel file with
#[path] calls what it names (unreachable_mounts.py): libc and the proofs call
dgon_parse that way, and nothing in src/ does.
"""

import re

from unreachable_mounts import mounted

DEFINITION = re.compile(
    r"^\s*pub(?:\(crate\))?\s+(?:const\s+|async\s+|unsafe\s+|extern\s+\"C\"\s+)*fn\s+([a-z_][A-Za-z0-9_]*)"
)
IMPORT = re.compile(r"^\s*(pub\s+)?use\s")
COMMENT = re.compile(r"^\s*(//|/\*|\*)")
WORD = re.compile(r"\b([a-z_][A-Za-z0-9_]*)\b")
RENAME = re.compile(r"\b([a-z_][A-Za-z0-9_]*)\s+as\s+([a-z_][A-Za-z0-9_]*)\b")
# Reached from assembly or from the linker, never by name from Rust.
EXPORTED_TO_LINKER = re.compile(r"#\[(no_mangle|export_name)")
INACTIVE = ("src/arch/aarch64/", "src/arch/riscv64/")


def sources(root, roots=("src",), skip=INACTIVE):
    for base in roots:
        for path in sorted((root / base).rglob("*.rs")):
            rel = path.relative_to(root)
            if any(str(rel).startswith(s) for s in skip):
                continue
            yield rel, path.read_text(errors="replace").splitlines()


def definitions(root, roots=("src",)):
    """(name, "path:line") for every exported function that builds."""
    found = []
    for rel, lines in sources(root, roots):
        for n, line in enumerate(lines, 1):
            match = DEFINITION.match(line)
            if not match:
                continue
            if EXPORTED_TO_LINKER.search("\n".join(lines[max(0, n - 4):n - 1])):
                continue
            found.append((match.group(1), f"{rel}:{n}"))
    return found


def kinds(lines):
    """Each line with what it is: "import" for every line of a `use`, from
    its first through the one that ends it; "skip" for a comment or a
    definition; "code" for the rest."""
    inside = False
    for line in lines:
        if inside or IMPORT.match(line):
            inside = ";" not in line
            yield "import", line
        elif COMMENT.match(line) or DEFINITION.match(line):
            yield "skip", line
        else:
            yield "code", line


def mentions(files):
    """Every name the code lines of `files` mention, and every name a `use`
    renames to one of those, through any chain of renames."""
    seen, renames = set(), []
    for lines in files:
        for kind, line in kinds(lines):
            if kind == "code":
                seen.update(WORD.findall(line))
            elif kind == "import":
                renames.extend(RENAME.findall(line))
    grew = True
    while grew:
        grew = False
        for name, alias in renames:
            if alias in seen and name not in seen:
                seen.add(name)
                grew = True
    return seen


def called(root, roots=("src",)):
    """Every name mentioned somewhere that is not an import or a definition,
    the inactive architectures' call sites included."""
    return mentions(lines for _, lines in sources(root, roots, skip=()))


def unreachable(root, roots=("src",)):
    """Exported functions whose name appears at no call site in the tree, nor
    in any crate that mounts their file."""
    live, mounts = called(root, roots), mounted(root, roots, mentions)
    return sorted(
        f"{site} {name}"
        for name, site in definitions(root, roots)
        if name not in live and name not in mounts.get(site.rsplit(":", 1)[0], ())
    )
