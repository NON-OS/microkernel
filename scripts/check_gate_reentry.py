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
"""No surface registry function takes GATE while it already holds it.

GATE (src/kernel_core/surface_registry/pin/gate.rs) is a spin lock, and a
spin lock taken twice on one CPU never opens: that CPU spins with
interrupts off, and every attach, DMA release and exit after it spins
behind it. orphans::reclaim held GATE and called dma_held::reclaim_dma,
which takes it, so the first surface drop, munmap or exit after boot froze
the desktop. For every function that holds a GATE guard, no call made while
the guard is live may reach, through the registry's own functions, one that
takes GATE. A name defined twice counts as taking GATE if either does.
"""

import argparse
import sys
from pathlib import Path

sys.path.insert(0, str(Path(__file__).parent))
from gate_reentry_scan import TAKES, calls, functions, live_spans  # noqa: E402

BASE = "src/kernel_core/surface_registry"


def takers(fns: dict) -> set:
    """Every name that takes GATE, itself or through a call it makes."""
    direct = {n for n, defs in fns.items() if any(TAKES.search(b) for _, b in defs)}
    graph = {n: set().union(*(calls(b) for _, b in defs)) & fns.keys() for n, defs in fns.items()}
    found, todo = set(direct), list(direct)
    while todo:
        target = todo.pop()
        for name, callees in graph.items():
            if target in callees and name not in found:
                found.add(name)
                todo.append(name)
    return found


def check(root: Path) -> int:
    fns = functions(root, BASE)
    if not fns:
        print(f"gate-reentry: no functions under {BASE}")
        return 1
    taking = takers(fns)
    holders, problems = 0, []
    for name, defs in sorted(fns.items()):
        for path, body in defs:
            for guard, span in live_spans(body):
                holders += 1
                if TAKES.search(span):
                    problems.append(f"{path}: {name} holds GATE ({guard}) and takes it again")
                for callee in sorted(calls(span) & taking):
                    problems.append(f"{path}: {name} holds GATE ({guard}) and calls {callee}, which takes it")
    for p in problems:
        print(f"gate-reentry: {p}")
    print(f"gate-reentry: {holders} GATE holders, {len(taking)} takers, {len(problems)} problems")
    return 1 if problems else 0


def main() -> int:
    ap = argparse.ArgumentParser(description=__doc__)
    ap.add_argument("--root", type=Path, default=Path("."))
    ap.add_argument("--self-test", action="store_true")
    args = ap.parse_args()
    if args.self_test:
        from gate_reentry_fixture import self_test  # noqa: E402
        return self_test(check, BASE)
    return check(args.root)


if __name__ == "__main__":
    sys.exit(main())
