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
"""Hold an AP's bring-up to the parking rule.

An AP that misses its online deadline is asked to park and parks at the next
`checkpoint` in its bring-up (`bring_up` in src/smp/ap/bring_up.rs and
`finish` in src/smp/ap/bring_up_late.rs, src/smp/ap/park.rs);
only then does the boot CPU send INIT. A checkpoint inside a lock would park
the AP with the lock held, and every later taker would spin, so:

  1. `bring_up` takes no lock itself: no guard lives across its statements,
     and each step's lock is taken and released inside the step's own call.
  2. Every step is followed by a checkpoint, so an AP asked to park answers
     within one step. The few statements that only read or record (the
     stage word, the local APIC id) are steps that take no lock and are
     exempt.

    scripts/check_ap_checkpoints.py [--self-test]
"""

import re
import sys
from pathlib import Path

# Each function of the bring-up, by file.
BRING_UP = ((Path("src/smp/ap/bring_up.rs"), "bring_up"), (Path("src/smp/ap/bring_up_late.rs"), "finish"))
CHECKPOINT = re.compile(r"^checkpoint\(cpu_id\)$")
# Anything that takes or holds a lock in this body.
LOCKS = re.compile(r"\.lock\(|try_lock\(|lock_responsive\(|\bMutex\b|\bRwLock\b|\bSpin\w*\b|guard\b")
# Statements that take no lock: they record or read, and call nothing that does.
EXEMPT = (re.compile(r"\.set_stage\("), re.compile(r"^let apic_id = crate::arch::interrupt_controller::local_id\(\)$"))


def body(text, name="bring_up"):
    """The statements of `fn name`, comments dropped, at the body's depth."""
    start = text.index(f"fn {name}(")
    open_at = text.index("{", start)
    depth, i, out, cur = 0, open_at, [], ""
    text = re.sub(r"/\*.*?\*/", "", text[open_at:], flags=re.S)
    text = re.sub(r"//[^\n]*", "", text)
    for ch in text:
        if ch == "{":
            depth += 1
            if depth == 1:
                continue
        elif ch == "}":
            depth -= 1
            if depth == 0:
                break
        if depth == 1 and ch == ";":
            out.append(" ".join(cur.split()))
            cur = ""
        elif depth >= 1:
            cur += ch
    tail = " ".join(cur.split())
    # An `unsafe { .. }` block with no trailing `;` ends a statement at its brace.
    stmts = []
    for s in out + ([tail] if tail else []):
        while s.startswith("unsafe {") and "}" in s and s.index("}") < len(s) - 1:
            cut = s.index("}") + 1
            stmts.append(s[:cut].strip())
            s = s[cut:].strip()
        if s:
            stmts.append(s)
    return stmts


def check(text, name="bring_up"):
    stmts = body(text, name)
    failed = []
    for s in stmts:
        if LOCKS.search(s):
            failed.append(f"{name} holds a lock across its statements: `{s}`")
    # The last statement is the returned value.
    steps = stmts[:-1]
    for i, s in enumerate(steps):
        if CHECKPOINT.match(s) or any(e.search(s) for e in EXEMPT):
            continue
        nxt = steps[i + 1] if i + 1 < len(steps) else None
        if nxt is None or not CHECKPOINT.match(nxt):
            failed.append(f"no checkpoint after the step `{s}`")
    if not any(CHECKPOINT.match(s) for s in steps):
        failed.append(f"{name} has no checkpoint at all")
    return failed


SAMPLE = """
pub(super) unsafe fn bring_up(cpu_id: u32) -> Prepared {
    CPU_DESCRIPTORS[cpu_id as usize].set_stage(Stage::ApEntered);
    checkpoint(cpu_id);
    unsafe { a() };
    checkpoint(cpu_id);
    let apic_id = crate::arch::interrupt_controller::local_id();
    unsafe {
        let _ = b(cpu_id, apic_id);
    }
    checkpoint(cpu_id);
    // a comment with a lock() in it
    let user = c();
    checkpoint(cpu_id);
    user
}
"""


def self_test():
    cases = [
        (SAMPLE, True),
        # A step with no checkpoint after it.
        (SAMPLE.replace("    unsafe { a() };\n    checkpoint(cpu_id);\n", "    unsafe { a() };\n"), False),
        # A checkpoint inside a lock: the guard lives across it.
        (SAMPLE.replace("    let user = c();\n    checkpoint(cpu_id);\n",
                        "    let g = PAGING.lock();\n    checkpoint(cpu_id);\n    let user = c(g);\n    checkpoint(cpu_id);\n"), False),
        (SAMPLE.replace("    let user = c();\n", "    let user = lock_responsive(&Q, c);\n"), False),
        # No checkpoints at all.
        (SAMPLE.replace("    checkpoint(cpu_id);\n", ""), False),
        # The last step before the returned value is unchecked.
        (SAMPLE.replace("    let user = c();\n    checkpoint(cpu_id);\n", "    let user = c();\n"), False),
    ]
    wrong = [i for i, (text, ok) in enumerate(cases) if (not check(text)) != ok]
    if wrong:
        print(f"ap-checkpoints: self-test failed on cases {wrong}")
        return 1
    print(f"ap-checkpoints: self-test passed, {sum(not ok for _, ok in cases)} bad bodies rejected "
          f"and {sum(ok for _, ok in cases)} good accepted")
    return 0


def main():
    if "--self-test" in sys.argv[1:]:
        return self_test()
    failed = [f"{path}: {line}" for path, name in BRING_UP for line in check(path.read_text(), name)]
    for line in failed:
        print(f"ap-checkpoints: {line}")
    if failed:
        return 1
    names = ", ".join(f"{name} ({path})" for path, name in BRING_UP)
    print(f"ap-checkpoints: {names}: every step is followed by a checkpoint and none sits inside a lock")
    return 0


if __name__ == "__main__":
    sys.exit(main())
