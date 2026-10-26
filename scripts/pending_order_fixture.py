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
"""check_pending_order.py's self-test: a small table and receive path,
clean, then each rule broken once. A holder calling out, at one hop or
through a table function that retakes PENDING, under a named guard or a
temporary one; a marking receive under a lock; a dequeue never marked.
Called after drop(guard), or named only in a comment or a string, passes,
and so does the reply receive, which is owed nothing."""

import tempfile
from pathlib import Path

MARK = ("pub(crate) fn mark_served(pid: u32, token: u64) {\n"
        "    if let Some(q) = PENDING.lock().get_mut(&pid) {\n        mark(q, token);\n    }\n}\n")
RULE = "pub(crate) fn mark(q: &mut Q, token: u64) {\n    q.touch(token);\n}\n"
RECV = ("pub(super) fn recv(pid: u32) -> i64 {\n    let msg = nonos_inbox::try_dequeue_existing(\"x\");\n"
        "    taken(pid, msg)\n}\nfn taken(pid: u32, msg: M) -> i64 {\n"
        "    super::pending_reply::mark_served(pid, msg.correlation);\n    0\n}\n")
REPLY = "pub(super) fn recv_reply_correlated() {\n    let _ = nonos_inbox::try_dequeue_existing(\"r\");\n}\n"
POP = "pub(super) fn pop(pid: u32) {\n    let mut map = PENDING.lock();\n    BODY\n}\n"

CASES = [
    ({}, 0),
    ({"pop.rs": POP.replace("BODY", "crate::sched::wake(pid);")}, 1),
    ({"pop.rs": POP.replace("BODY", "tidy(pid);") + "fn tidy(pid: u32) {\n    mark_served(pid, 1);\n}\n"}, 1),
    ({"mark.rs": MARK.replace("mark(q, token)", "nonos_inbox::wake(pid)")}, 1),
    ({"../recv.rs": RECV.replace("    super::pending", "    let _g = ROUTE.lock();\n    super::pending")}, 1),
    ({"../recv.rs": RECV.replace("    super::pending_reply::mark_served(pid, msg.correlation);\n", "")}, 1),
    ({"pop.rs": POP.replace("BODY", "drop(map);\n    crate::sched::wake(pid);")}, 0),
    ({"pop.rs": POP.replace("BODY", "// crate::sched::wake(pid);\n    say(\"wake()\");")
      .replace("say(", "mark(")}, 0),
]


def self_test(check, table: str) -> int:
    for i, (over, want) in enumerate(CASES):
        files = {"mark.rs": MARK, "served_rule.rs": RULE, "pop.rs": POP.replace("BODY", ""),
                 "../recv.rs": RECV, "../recv_reply.rs": REPLY, **over}
        with tempfile.TemporaryDirectory() as d:
            for name, text in files.items():
                path = (Path(d) / table / name).resolve()
                path.parent.mkdir(parents=True, exist_ok=True)
                path.write_text(text)
            if check(Path(d)) != want:
                print(f"pending-order: self-test failed on case {i}: want exit {want}")
                return 1
    print("pending-order: self-test passed: a holder calling out or retaking PENDING, a marking "
          "receive under a lock and an unmarked dequeue are each caught")
    return 0
