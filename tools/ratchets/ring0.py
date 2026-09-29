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
"""What ring 0 is, from rustc's dep-info, and which of its lines are code."""

from pathlib import Path


def depinfo_sources(depinfo, root):
    """The .rs files under root/src that the dep-info lists."""
    text = depinfo.read_text()
    first = text.split("\n", 1)[0]
    _, _, deps = first.partition(": ")
    src = (root / "src").resolve()
    out = set()
    # Paths with spaces are escaped with a backslash; the kernel has none, so
    # a plain split is exact here and anything odd is refused below.
    for dep in deps.split():
        p = Path(dep)
        if not p.is_absolute():
            p = root / p
        p = p.resolve()
        if p.suffix == ".rs" and src in p.parents:
            out.add(p)
    return sorted(out)


def code_line_numbers(path):
    """1-based numbers of lines that are neither blank nor comment. Block
    comments may nest in Rust; the tree does not nest them, and a nested one
    only miscounts the lines inside it."""
    out = set()
    in_block = False
    for n, raw in enumerate(path.read_text(errors="replace").splitlines(), 1):
        s = raw.strip()
        if in_block:
            in_block = "*/" not in s
            continue
        if not s or s.startswith("//"):
            continue
        if s.startswith("/*"):
            in_block = "*/" not in s
            continue
        out.add(n)
    return out


def code_lines(path):
    return len(code_line_numbers(path))


def newest_depinfo(target):
    found = []
    for d in target.glob("nonos_kernel-*.d"):
        # The library's dep-info lists every module; the binary's lists two.
        if len(d.read_text().split("\n", 1)[0].split()) > 64:
            found.append(d)
    if not found:
        return None
    return max(found, key=lambda d: d.stat().st_mtime)
