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
"""The scans check_pending_order.py runs over the IPC syscalls. PENDING
(pending_reply/state.rs) is a spin lock: a holder that reaches a function
taking it again spins forever, and one that reaches the inbox, the
scheduler or delivery orders PENDING before their locks, where every
other path takes them first."""

import re

from gate_reentry_scan import calls, close_of

TAKES_PENDING = re.compile(r"\bPENDING\s*\.\s*(?:try_)?lock\s*\(")
TAKES_ANY = re.compile(r"\.\s*(?:try_)?lock(?:_irqsave)?\s*\(|\block_responsive\s*\(")
DEQUEUE = re.compile(r"\btry_dequeue_existing\s*\(")
LET = re.compile(r"\blet\s+(?:mut\s+)?(\w+)\s*=\s*[^;]*$")


def held_spans(body: str):
    """The text after each PENDING take while its guard is live: to the end
    of the enclosing block, or to drop(guard) when the guard is named."""
    for m in TAKES_PENDING.finditer(body):
        depth, opener = 0, 0
        for i in range(m.start(), -1, -1):
            depth += {"}": 1, "{": -1}.get(body[i], 0)
            if depth < 0:
                opener = i
                break
        span = body[m.end():close_of(body, opener)]
        stmt = body[max(body.rfind(";", 0, m.start()), body.rfind("{", 0, m.start())) + 1:m.start()]
        named = LET.search(stmt)
        if named:
            dropped = re.search(r"\bdrop\(\s*" + named.group(1) + r"\s*\)", span)
            span = span[:dropped.start()] if dropped else span
        yield span


def reaching(fns: dict, seeds: set) -> set:
    """Every name in `fns` that is in `seeds` or calls one that is."""
    graph = {n: set().union(*(calls(b) for _, b in d)) for n, d in fns.items()}
    found, grew = set(seeds), True
    while grew:
        grew = False
        for name, callees in graph.items():
            if name not in found and callees & found:
                found.add(name)
                grew = True
    return found


def takers(fns: dict, pattern) -> set:
    return {n for n, d in fns.items() if any(pattern.search(b) for _, b in d)}


def marking_problems(outside: dict, replies: set):
    """Rules 2 and 3 over the IPC syscalls outside the table: the names
    that reach mark_served, and what is wrong with them."""
    marking = reaching(outside, {"mark_served"}) - {"mark_served"}
    found = [f"{n} reaches mark_served and takes a lock" for n in sorted(marking & takers(outside, TAKES_ANY))]
    for name in sorted(takers(outside, DEQUEUE) - marking - replies):
        found.append(f"{name} dequeues a request and never reaches mark_served")
    return marking, found
