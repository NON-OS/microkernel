# Which functions take which locks, and where one is held across its own take.
#
# By name, within the kernel crate. A lock is a `static` Mutex or RwLock whose
# name no other static in src/ has. A call is followed when it names a
# function by a path or bare name, `f(` or `a::b::f(`, and exactly one
# function in src/ has that name; a method call `.f(`, a function value, a
# trait call and Drop are not followed. What it finds is real or a false alarm
# to fix in the checker; what it cannot see the runtime report catches
# (smp-lock-waits.md).

import re

from .holds import holds
from .text import close, line_of

FN = re.compile(r"\bfn\s+(\w+)\b")
STATIC = re.compile(r"\bstatic\s+(\w+)\s*:\s*(?:spin::)?(?:Mutex|RwLock)\s*<")
CALL = re.compile(r"(?<![\w.!])(?:\w+::)*(\w+)\s*(?:::\s*<[^()]*>)?\s*\(")
NOT_CALLS = {"if", "while", "match", "for", "return", "loop", "Some", "Ok", "Err", "drop"}


def functions(files):
    """(name, path, text, body start, body end) for every fn with a body."""
    out = []
    for path, text in files:
        for m in FN.finditer(text):
            j = m.end()
            while j < len(text) and text[j] not in "{;":
                j += 1
            if j < len(text) and text[j] == "{":
                out.append((m.group(1), path, text, j, close(text, j)))
    return out


def unique(names):
    seen = {}
    for name in names:
        seen[name] = seen.get(name, 0) + 1
    return {name for name, n in seen.items() if n == 1}


def calls(text, start, end):
    return [(c.group(1), c.start()) for c in CALL.finditer(text, start, end)
            if c.group(1) not in NOT_CALLS]


def findings(files):
    locks = unique(m.group(1) for _, text in files for m in STATIC.finditer(text))
    fns = [(*f, holds(*f[2:])) for f in functions(files)]
    once = unique(f[0] for f in fns)
    named = {f[0]: f for f in fns if f[0] in once}
    takes = {n: {h[0] for h in f[5] if h[0] in locks} for n, f in named.items()}
    callees = {n: {c for c, _ in calls(*f[2:5]) if c in named} for n, f in named.items()}
    grew = True
    while grew:
        grew = False
        for name, out in callees.items():
            for callee in out:
                if not takes[callee] <= takes[name]:
                    takes[name] |= takes[callee]
                    grew = True
    found = []
    for _, path, text, start, end, taken in fns:
        for lock, at, held, to in taken:
            if lock not in locks:
                continue
            where = f"{path}:{line_of(text, at)}: {lock}"
            for again, again_at, _, _ in taken:
                if again == lock and held <= again_at < to:
                    found.append(f"{where} taken again at line {line_of(text, again_at)}")
            for callee, call_at in calls(text, held, to):
                if callee in named and lock in takes[callee]:
                    line = line_of(text, call_at)
                    found.append(f"{where} held across {callee}() at line {line}, which takes it")
    return found
