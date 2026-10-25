# Where a take of a lock is held, read by Rust 2021's rules for when a guard
# is dropped:
#   - `let g = L.lock();`, the guard the whole initializer: to the end of the
#     block, or to `drop(g)` before it;
#   - `match`, `if let` and `while let` on a take: through the whole body, and
#     an `else` after it;
#   - any other take, a temporary: to the end of its statement, or to the `{`
#     of a plain `if` or `while` whose condition it is in.
# `let _ = L.lock();` drops at once. try_lock and its kin never wait.

import re

from .text import close, close_of

TAKE = re.compile(
    r"(?:(?:\w+::)*(?:lock|read|write)_responsive\s*\(\s*&\s*(?:\w+::)*(\w+)\s*\))"
    r"|(?:(?<![\w.])(?:\w+::)*([A-Z][A-Z0-9_]+)\s*\.\s*(?:lock|read|write)\s*\(\s*\))"
)
SCRUTINEE = re.compile(r"\s*(?:match\b|if\s+let\b|while\s+let\b)")
LET = re.compile(r"\s*let\s+(?:mut\s+)?(\w+)[^=;]*=\s*$")
WHOLE = re.compile(r"\s*[;?]")
ELSE = re.compile(r"\s*else\b")


def stmt_end(text: str, i: int, end: int) -> int:
    """Where a temporary made at `i` is dropped."""
    depth = 0
    for j in range(i, end):
        c = text[j]
        if c == "{" and depth == 0:
            return j
        if c in "({[":
            depth += 1
        elif c in ")}]":
            if depth == 0:
                return j
            depth -= 1
        elif c == ";" and depth == 0:
            return j
    return end


def body_end(text: str, i: int, end: int) -> int:
    """The end of the body a scrutinee at `i` opens, with an `else` after it."""
    body = text.find("{", stmt_end(text, i, end))
    if not 0 <= body < end:
        return end
    to = close(text, body)
    if ELSE.match(text, to + 1, end):
        nxt = text.find("{", to + 1)
        to = close(text, nxt) if 0 <= nxt < end else to
    return to


def holds(text: str, start: int, end: int) -> list[tuple[str, int, int, int]]:
    """(lock, taken at, held from, held to) for every take in [start, end)."""
    out = []
    for m in TAKE.finditer(text, start, end):
        lock = m.group(1) or m.group(2)
        stmt = max(text.rfind(c, start, m.start()) for c in ";{}") + 1
        head = text[stmt : m.start()]
        let = LET.match(head)
        whole = WHOLE.match(text, m.end())
        if let and whole and let.group(1) != "_":
            to = close_of(text, m.end())
            gone = re.compile(r"\bdrop\s*\(\s*" + let.group(1) + r"\s*\)").search(text, m.end(), to)
            to = gone.start() if gone else to
        elif SCRUTINEE.match(head):
            to = body_end(text, m.end(), end)
        else:
            to = stmt_end(text, m.end(), end)
        out.append((lock, m.start(), m.end(), to))
    return out

