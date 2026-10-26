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
"""PENDING, the IPC pending-reply table, is taken in one order only.

PENDING (ipc/pending_reply/state.rs) is a spin lock, and the served mark
(mark.rs) takes it wherever a server's receive dequeues a request:
  1. A PENDING holder calls only pending_reply's own functions, and none
     that takes PENDING: never the inbox, the scheduler or delivery, whose
     locks are taken before PENDING everywhere else, and never itself.
  2. A function in the IPC syscalls that reaches mark_served takes no lock
     itself, so the mark never runs under the inbox's or anyone's lock.
  3. Every function there that dequeues a request reaches mark_served, but
     recv_reply_correlated, which dequeues a reply, owed nothing.
"""

import argparse
import sys
from pathlib import Path

sys.path.insert(0, str(Path(__file__).parent))
from gate_reentry_scan import calls, functions  # noqa: E402
from pending_order_scan import TAKES_PENDING, held_spans, marking_problems, reaching, takers  # noqa: E402

IPC = "src/syscall/microkernel/ipc"
TABLE = IPC + "/pending_reply"
REPLIES = {"recv_reply_correlated"}


def check(root: Path) -> int:
    table, ipc = functions(root, TABLE), functions(root, IPC)
    retaking = reaching(table, takers(table, TAKES_PENDING))
    outside = {n: d for n, d in ipc.items() if all(TABLE not in str(p) for p, _ in d)}
    problems, holders = [], 0
    for name, defs in sorted(table.items()):
        for path, body in defs:
            for span in held_spans(body):
                holders += 1
                for callee in sorted(calls(span) - set(table)):
                    problems.append(f"{path}: {name} holds PENDING and calls {callee}, outside the table")
                for callee in sorted(calls(span) & retaking):
                    problems.append(f"{path}: {name} holds PENDING and calls {callee}, which takes it")
    if not holders:
        problems.append(f"no PENDING holder under {TABLE}: the scan sees nothing")
    marking, found = marking_problems(outside, REPLIES)
    problems += found
    for p in problems:
        print(f"pending-order: {p}")
    print(f"pending-order: {holders} PENDING holders, {len(marking)} marking, {len(problems)} problems")
    return 1 if problems else 0


def main() -> int:
    ap = argparse.ArgumentParser(description=__doc__)
    ap.add_argument("--root", type=Path, default=Path("."))
    ap.add_argument("--self-test", action="store_true")
    args = ap.parse_args()
    if args.self_test:
        from pending_order_fixture import self_test  # noqa: E402
        return self_test(check, TABLE)
    return check(args.root)


if __name__ == "__main__":
    sys.exit(main())
