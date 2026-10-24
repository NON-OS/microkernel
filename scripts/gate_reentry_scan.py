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
"""What check_gate_reentry.py reads: the surface registry's functions, by
name, each with its body, the calls it makes and where its GATE guard is
live. Comments and string literals are blanked first, so a name in either
is no call."""

import re
from pathlib import Path

FN = re.compile(r"\bfn\s+(\w+)\s*(?:<[^>{]*>)?\s*\(")
# A free or associated function call: `name(` or `Type::name(`, never a
# method through `.`, whose name (collect, take, lock) says nothing of whose
# it is.
CALL = re.compile(r"(?<![.\w])\s*\b([a-z_][a-z0-9_]*)\s*(?:::<[^>]*>)?\(")
GUARD = re.compile(r"\blet\s+(\w+)\s*=\s*(?:crate::smp::)?lock_responsive\(\s*&[\w:]*\bGATE\s*\)")
TAKES = re.compile(r"(?:lock_responsive\(\s*&[\w:]*\bGATE\s*\)|\bGATE\s*\.\s*(?:try_)?lock\s*\()")
BLANK = re.compile(r'//[^\n]*|/\*.*?\*/|"(?:\\.|[^"\\])*"', re.S)


def blank(text: str) -> str:
    """The text with every comment and string literal spaced out, lengths kept."""
    return BLANK.sub(lambda m: re.sub(r"[^\n]", " ", m.group(0)), text)


def close_of(text: str, open_at: int) -> int:
    """The index of the brace that closes the one at `open_at`."""
    depth = 0
    for i in range(open_at, len(text)):
        depth += {"{": 1, "}": -1}.get(text[i], 0)
        if depth == 0:
            return i
    return len(text)


def functions(root: Path, base: str) -> dict:
    """Every fn under `base`: name -> list of (path, body text)."""
    out = {}
    for path in sorted((root / base).rglob("*.rs")):
        text = blank(path.read_text(errors="replace"))
        for m in FN.finditer(text):
            brace = text.find("{", m.end())
            semi = text.find(";", m.end())
            if brace < 0 or (0 <= semi < brace):
                continue
            body = text[brace:close_of(text, brace) + 1]
            out.setdefault(m.group(1), []).append((path.relative_to(root), body))
    return out


def live_spans(body: str):
    """Each (guard name, text while it is live): from the guard to the end
    of its block, or to an explicit drop of it."""
    for m in GUARD.finditer(body):
        opener = body.rfind("{", 0, m.start())
        depth = 0
        for i in range(m.start(), -1, -1):
            depth += {"}": 1, "{": -1}.get(body[i], 0)
            if depth < 0:
                opener = i
                break
        end = close_of(body, opener)
        span = body[m.end():end]
        dropped = re.search(r"\bdrop\(\s*" + re.escape(m.group(1)) + r"\s*\)", span)
        yield m.group(1), span[:dropped.start()] if dropped else span


def calls(text: str) -> set:
    return {m.group(1) for m in CALL.finditer(text)}
