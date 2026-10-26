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
"""Kernel files mounted by crates outside src/, and the names those crates use.

A crate may build a kernel file into itself with `#[path = "..."]`: libc
mounts the broker's wire formats, the proof crates mount the rules they
prove, the extraction crates the functions they extract. A function such a
crate calls is called, although nothing in src/ names it. The crate is the
nearest directory above the mounting file with a Cargo.toml, below the root;
all of its Rust counts, the mounted file's other definitions do not. A mount
of a directory, or of a mod.rs, brings the files under it, whose modules the
crate declares from there: rng_callers_proofs mounts src/security whole.
"""

import os
import re
from pathlib import Path

PATH = re.compile(r'#\[path\s*=\s*"([^"]+)"\s*\]')
# Build output, and the repository's own metadata.
SKIP = {"target", ".git"}


def rust_files(top, skip_top=()):
    """Every .rs file under `top` with its text, build output skipped, and
    the directories in `skip_top` skipped at the top alone."""
    for here, dirs, files in os.walk(top):
        first = Path(here) == top
        dirs[:] = sorted(d for d in dirs if d not in SKIP and not (first and d in skip_top))
        for name in sorted(files):
            if name.endswith(".rs"):
                path = Path(here) / name
                yield path, path.read_text(errors="replace")


def crate_of(root, path):
    """The crate a file outside the kernel belongs to; None for the root's."""
    for d in path.parents:
        if d == root or root not in d.parents:
            return None
        if (d / "Cargo.toml").is_file():
            return d
    return None


def brought(root, rel):
    """The kernel files a mount of `rel` brings: the file, or every file
    under the directory it names or whose mod.rs it is."""
    full = root / rel
    if not (full.is_dir() or full.name == "mod.rs"):
        return {rel.as_posix()}
    top = full if full.is_dir() else full.parent
    return {path.relative_to(root).as_posix() for path, _ in rust_files(top)}


def mounted(root, roots, mentions):
    """For each file under `roots` some crate elsewhere mounts, every name
    those crates mention, as `mentions` reads them."""
    root = Path(os.path.abspath(root))
    by_crate = {}
    for path, text in rust_files(root, skip_top=roots):
        for target in PATH.findall(text) if "#[path" in text else ():
            rel = Path(os.path.relpath(os.path.normpath(path.parent / target), root))
            crate = crate_of(root, path)
            if crate is not None and rel.parts[:1] and rel.parts[0] in roots:
                by_crate.setdefault(crate, set()).update(brought(root, rel))
    names = {}
    for crate, files in by_crate.items():
        said = mentions(text.splitlines() for _, text in rust_files(crate))
        for rel in files:
            names.setdefault(rel, set()).update(said)
    return names
