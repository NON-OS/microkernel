#!/usr/bin/env python3
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
"""Rust and Cargo, read well enough to say which calls a crate makes and where.
Read by cap_audit.

Comments are blanked before anything is matched, because a doc comment that
names `mk_debug` is not a call to it. String literals are kept apart, because
a service name is evidence only inside one. Every view keeps the original
offsets, so a match maps back to the line a reviewer can open.
"""

import re
from bisect import bisect_left
from pathlib import Path

RAW_OPEN = re.compile(r'r(#*)"')
CHAR_LIT = re.compile(r"'(?:\\(?:x[0-9a-fA-F]{2}|u\{[0-9a-fA-F]+\}|.)|[^\\'\n])'")
FN_DEF = re.compile(r"\bfn\s+(\w+)")
# A call: an optional one-segment qualifier, the name, an optional turbofish,
# then the open parenthesis. A method call (`.name(`) is not a free function.
CALL = re.compile(r"(?<![\w.:])((?:\w+\s*::\s*)*)(\w+)\s*(?:::\s*<[^>()]*>\s*)?\(")
TAG4 = re.compile(r'tag4\(\s*b"(\w{4})"\s*\)')
DEREF_TAG = re.compile(r'\*b"(\w{4})"')
TAG_CONST = re.compile(r'const\s+(\w+)\s*:\s*i64\s*=\s*tag4\(\s*b"(\w{4})"\s*\)')
MOD_DECL = re.compile(r"\bmod\s+(\w+)\s*;")
# The attributes stacked on a declaration, read backwards from it.
ATTRS_BEFORE = re.compile(r"((?:#\[[^\]]*\][ \t\n]*)+)(?:pub(?:\([^)]*\))?[ \t\n]+)?$")
PATH_ATTR = re.compile(r'#\[path\s*=\s*"([^"]+)"\]')
INCLUDE_RS = re.compile(r'include!\(\s*"([^"]+\.rs)"\s*\)')


def _blank(s):
    return re.sub(r"[^\n]", " ", s)


def _ident_char(c):
    return c.isalnum() or c == "_"


def scan(text):
    """(code, plain, literals) for one file.

    `code` has comments and the contents of every string and char literal
    blanked, so braces and names inside them cannot be mistaken for code.
    `plain` has only the comments blanked. `literals` is (offset, contents)
    for each string literal. All three keep the original offsets.
    """
    code, plain, lits = [], [], []
    i, n = 0, len(text)
    while i < n:
        c = text[i]
        if text.startswith("//", i):
            j = text.find("\n", i)
            j = n if j < 0 else j
            code.append(" " * (j - i))
            plain.append(" " * (j - i))
            i = j
            continue
        if text.startswith("/*", i):
            j, depth = i + 2, 1
            while j < n and depth:
                if text.startswith("/*", j):
                    depth, j = depth + 1, j + 2
                elif text.startswith("*/", j):
                    depth, j = depth - 1, j + 2
                else:
                    j += 1
            code.append(_blank(text[i:j]))
            plain.append(_blank(text[i:j]))
            i = j
            continue
        raw = RAW_OPEN.match(text, i) if c == "r" else None
        before = text[i - 1] if i else " "
        if before == "b" and i > 1:
            before = text[i - 2]
        if raw and not _ident_char(before):
            close = '"' + raw.group(1)
            end = text.find(close, raw.end())
            end = n if end < 0 else end
            stop = min(n, end + len(close))
            lits.append((i, text[raw.end():end]))
            code.append(text[i:raw.end()] + _blank(text[raw.end():end]) + text[end:stop])
            plain.append(text[i:stop])
            i = stop
            continue
        if c == '"':
            j = i + 1
            while j < n and text[j] != '"':
                j += 2 if text[j] == "\\" else 1
            j = min(j, n)
            stop = min(n, j + 1)
            lits.append((i, text[i + 1:j]))
            code.append('"' + _blank(text[i + 1:j]) + text[j:stop])
            plain.append(text[i:stop])
            i = stop
            continue
        if c == "'":
            m = CHAR_LIT.match(text, i)
            if m:
                code.append("'" + " " * (m.end() - i - 2) + "'")
                plain.append(text[i:m.end()])
                i = m.end()
                continue
        code.append(c)
        plain.append(c)
        i += 1
    return "".join(code), "".join(plain), lits


class Source:
    """One .rs file, scanned once, with offsets mapped back to lines."""

    def __init__(self, path, text):
        self.path = path
        self.code, self.plain, self.literals = scan(text)
        self._newlines = [i for i, ch in enumerate(text) if ch == "\n"]

    _cache = {}

    @classmethod
    def read(cls, path):
        """The file scanned, once per run whatever reads it."""
        key = Path(path)
        if key not in cls._cache:
            cls._cache[key] = cls(key, key.read_text(errors="replace"))
        return cls._cache[key]

    def line(self, offset):
        return bisect_left(self._newlines, offset) + 1

    def calls(self, start=0, end=None):
        """(path, name, offset) for each free-function call in a span, the
        path being the segments before the name: `a::b::f(` is (a, b)."""
        end = len(self.code) if end is None else end
        for m in CALL.finditer(self.code, start, end):
            head = self.code[max(0, m.start() - 8):m.start()]
            if re.search(r"\bfn\s*$", head):
                continue
            path = tuple(p.strip() for p in m.group(1).split("::") if p.strip())
            yield path, m.group(2), m.start(2)

    def tags(self):
        """(tag, offset) for each syscall tag written into this file."""
        for rx in (TAG4, DEREF_TAG):
            for m in rx.finditer(self.plain):
                yield m.group(1), m.start(1)

    def tag_constants(self):
        """const name -> tag, for `const N: i64 = tag4(b"XXXX")`."""
        return {m.group(1): m.group(2) for m in TAG_CONST.finditer(self.plain)}

    def functions(self):
        """(name, body start, body end) for each fn with a body."""
        for m in FN_DEF.finditer(self.code):
            brace = _body_open(self.code, m.end())
            if brace is None:
                continue
            yield m.group(1), brace, match_brace(self.code, brace)


def _body_open(code, start):
    """The `{` opening a fn body, or None for a declaration ending in `;`.
    A `;` inside the signature's brackets (`&mut [u8; 32]`) ends nothing."""
    depth = 0
    for i in range(start, len(code)):
        c = code[i]
        if c in "([":
            depth += 1
        elif c in ")]":
            depth -= 1
        elif depth == 0 and c == "{":
            return i
        elif depth == 0 and c == ";":
            return None
    return None


def match_brace(code, open_at):
    """Offset just past the brace closing the one at `open_at`."""
    depth = 0
    for i in range(open_at, len(code)):
        if code[i] == "{":
            depth += 1
        elif code[i] == "}":
            depth -= 1
            if depth == 0:
                return i + 1
    return len(code)


def module_files(entry):
    """Every file in the module tree under `entry`, following `mod x;` and
    `#[path]`. A module under `#[cfg(test)]` is not part of the build and is
    left out. A missing file is skipped: cargo would refuse the crate, and
    this reads the crate, it does not build it."""
    # Each entry is a file and the module file whose place in the tree it
    # takes: an `include!`d file declares modules as the file including it.
    seen, todo = {}, [(Path(entry), Path(entry))]
    while todo:
        path, owner = todo.pop()
        if path in seen or not path.is_file():
            continue
        seen[path] = True
        src = Source.read(path)
        here = owner.parent
        owns_dir = owner.name in ("mod.rs", "lib.rs", "main.rs")
        for m in INCLUDE_RS.finditer(src.plain):
            todo.append((path.parent / m.group(1), owner))
        for m in MOD_DECL.finditer(src.code):
            head = src.plain[max(0, m.start() - 400):m.start()].rstrip()
            before = ATTRS_BEFORE.search(head + "\n")
            attrs = before.group(1) if before else ""
            if re.search(r"cfg\(\s*test\s*\)", attrs):
                continue
            name = m.group(1)
            explicit = PATH_ATTR.search(attrs)
            if explicit:
                target = here / explicit.group(1)
                todo.append((target, target))
                continue
            base = here if owns_dir else here / owner.stem
            for target in (base / f"{name}.rs", base / name / "mod.rs"):
                todo.append((target, target))
    return list(seen)


# ---------------------------------------------------------------------------
# Cargo.toml


DEP_HEADER = re.compile(r"^\[(?:target\.[^\]]*?\.)?dependencies(?:\.([\w-]+))?\]$")
DEV_HEADER = re.compile(r"(dev|build)-dependencies")


class Dependency:
    """One path dependency: where it is and which features it asks for."""

    def __init__(self, name, path, default_features, features):
        self.name = name
        self.path = path
        self.default_features = default_features
        self.features = features

    def __repr__(self):
        return f"Dependency({self.name}, {self.path})"


def _toml_line(line):
    """A TOML line without its comment; a `#` inside a string stays."""
    quoted = False
    for i, c in enumerate(line):
        if c == '"':
            quoted = not quoted
        elif c == "#" and not quoted:
            return line[:i].rstrip()
    return line.rstrip()


def _dep_from_table(name, body, base):
    path = re.search(r'\bpath\s*=\s*"([^"]+)"', body)
    if not path:
        return None
    off = re.search(r"default-features\s*=\s*false", body)
    feats = re.search(r"features\s*=\s*\[([^\]]*)\]", body)
    names = set(re.findall(r'"([^"]+)"', feats.group(1))) if feats else set()
    return Dependency(name, (base / path.group(1)).resolve(), not off, names)


def cargo_manifest(toml_path):
    """(path dependencies, [lib] path, {bin name: path}, default features,
    features table) of one Cargo.toml. Dev and build dependencies do not end
    up in the capsule and are not read."""
    toml_path = Path(toml_path)
    base = toml_path.parent
    deps, bins, features = [], {}, {}
    lib = None
    section, table_name, table_body = None, None, []
    lines = toml_path.read_text().splitlines()

    def flush():
        if table_name and table_body:
            dep = _dep_from_table(table_name, "\n".join(table_body), base)
            if dep:
                deps.append(dep)

    current_bin = {}
    i = 0
    while i < len(lines):
        stripped = _toml_line(lines[i]).strip()
        i += 1
        if not stripped:
            continue
        if stripped.startswith("["):
            flush()
            table_name, table_body = None, []
            if current_bin.get("name") and current_bin.get("path"):
                bins[current_bin["name"]] = base / current_bin["path"]
            current_bin = {}
            section = stripped
            dep = DEP_HEADER.match(stripped)
            if dep and not DEV_HEADER.search(stripped) and dep.group(1):
                table_name = dep.group(1)
            continue
        if section is None:
            continue
        if table_name:
            table_body.append(stripped)
            continue
        key, _, value = stripped.partition("=")
        key, value = key.strip(), value.strip()
        # An inline table may run over several lines; read until it closes.
        while value.count("{") > value.count("}") and i < len(lines):
            value += " " + _toml_line(lines[i]).strip()
            i += 1
        while value.count("[") > value.count("]") and i < len(lines):
            value += " " + _toml_line(lines[i]).strip()
            i += 1
        if DEP_HEADER.match(section) and not DEV_HEADER.search(section):
            dep = _dep_from_table(key, value, base)
            if dep:
                deps.append(dep)
        elif section == "[lib]" and key == "path":
            lib = base / value.strip('"')
        elif section == "[[bin]]" and key in ("name", "path"):
            current_bin[key] = value.strip('"')
        elif section == "[features]":
            features[key] = set(re.findall(r'"([^"]+)"', value))
    flush()
    if current_bin.get("name") and current_bin.get("path"):
        bins[current_bin["name"]] = base / current_bin["path"]
    return deps, lib, bins, features.get("default", set()), features


def lock_packages(lock_path):
    """Package names a Cargo.lock lists, for the crates.io tools whose source
    is not in this tree."""
    if not Path(lock_path).is_file():
        return set()
    return set(re.findall(r'^name = "([^"]+)"', Path(lock_path).read_text(), re.M))
