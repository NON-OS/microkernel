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
"""Judge a boot log of a kernel built with nonos-smp-shootdown-proof.

The kernel (src/memory/paging/manager/shootdown/proof.rs, with
nonos-shootdown-trace) runs two rounds once the APs are up. The log passes
only if, after `[SMP] answered=A online=N` with N >= 2:

  1. a round with asid=0 pages=0 reached all N-1 other CPUs: the full flush;
  2. a round with asid=4294967295 pages=1 reached one CPU, the AP the proof
     line names, and its set holds that AP's bit alone: the user-asid filter;
  3. no round ran for asid=4294967294, which nobody holds;
  4. then `[SHOOTDOWN-PROOF] ... held_targets=1 unheld_targets=0 restored=1 PASS`;
  5. and no shootdown left acks outstanding, no panic, no fatal trap.

    scripts/check_shootdown_proof_log.py --log LOG
    scripts/check_shootdown_proof_log.py --self-test
"""

import re
import sys

HELD, UNHELD = 4294967295, 4294967294
ONLINE = re.compile(r"^\[SMP\] answered=\d+ online=(\d+)$")
ROUND = re.compile(r"^\[SHOOTDOWN\] round=\d+ cpu=(\d+) asid=(\d+) pages=(\d+) targets=(\d+) "
                   r"starting=\d+ answered_for=\d+ set=((?:\d+:0x[0-9A-F]{16} )*)$")
PROOF = re.compile(r"^\[SHOOTDOWN-PROOF\] full_flush=sent ap=(\d+) held_targets=(\d+) "
                   r"unheld_targets=(\d+) restored=(\d+) (PASS|FAIL)$")
NEVER = ("[SMP] acks outstanding=", "KERNEL PANIC", "KERNEL FATAL TRAP", "[SHOOTDOWN-PROOF] FAIL")


def cpus(field):
    out = set()
    for word, bits in re.findall(r"(\d+):0x([0-9A-F]{16})", field):
        out |= {int(word) * 64 + b for b in range(64) if int(bits, 16) >> b & 1}
    return out


def judge(lines):
    lines = [t.rstrip("\r") for t in lines]
    bad = [n for t in lines for n in NEVER if n in t]
    if bad:
        return f"the log carries {bad[0]!r}"
    at = next((i for i, t in enumerate(lines) if ONLINE.match(t)), None)
    if at is None:
        return "no [SMP] answered= line"
    online = int(ONLINE.match(lines[at]).group(1))
    if online < 2:
        return f"online={online}: no AP to send a round to"
    rounds = [(i, ROUND.match(t)) for i, t in enumerate(lines) if i > at and ROUND.match(t)]
    proof = [(i, PROOF.match(t)) for i, t in enumerate(lines) if i > at and PROOF.match(t)]
    if len(proof) != 1:
        return f"{len(proof)} [SHOOTDOWN-PROOF] lines, not one"
    p_at, p = proof[0]
    ap = int(p.group(1))
    if (p.group(2), p.group(3), p.group(4), p.group(5)) != ("1", "0", "1", "PASS"):
        return f"the proof line does not pass: {lines[p_at]}"
    full = [(i, m) for i, m in rounds if i < p_at and m.group(2) == "0" and m.group(3) == "0"]
    if not full or int(full[0][1].group(4)) != online - 1 or len(cpus(full[0][1].group(5))) != online - 1:
        return f"no full-flush round (asid=0 pages=0) reached the {online - 1} other CPUs"
    held = [(i, m) for i, m in rounds if i < p_at and int(m.group(2)) == HELD]
    if len(held) != 1:
        return f"{len(held)} rounds for the held asid, not one"
    i, m = held[0]
    if i < full[0][0] or m.group(3) != "1" or m.group(4) != "1" or cpus(m.group(5)) != {ap}:
        return f"the held-asid round did not reach ap={ap} alone: {lines[i]}"
    if any(int(m.group(2)) == UNHELD for _, m in rounds):
        return "a round ran for the asid nobody holds"
    return None


def self_test():
    def setw(*c):
        return f"0:0x{sum(1 << x for x in c):016X} "
    good = ["[SMP] answered=3 online=4",
            f"[SHOOTDOWN] round=18 cpu=0 asid=0 pages=0 targets=3 starting=0 answered_for=0 set={setw(1, 2, 3)}",
            f"[SHOOTDOWN] round=19 cpu=0 asid={HELD} pages=1 targets=1 starting=0 answered_for=0 set={setw(1)}",
            "[SHOOTDOWN-PROOF] full_flush=sent ap=1 held_targets=1 unheld_targets=0 restored=1 PASS"]
    def swap(i, old, new):
        return [t.replace(old, new) if j == i else t for j, t in enumerate(good)]
    cases = [
        (good, True),
        (good[:1] + good[2:], False),                                  # no full flush
        (swap(1, "targets=3", "targets=2"), False),          # full flush missed one
        (swap(1, "pages=0", "pages=16"), False),                        # a range, not a full flush
        (swap(2, "targets=1", "targets=3"), False),          # filter let every cpu in
        (swap(2, setw(1), setw(2)), False),                             # reached the wrong AP
        (good[:2] + good[3:], False),                                   # held round never ran
        (good[:3] + [good[2].replace(str(HELD), str(UNHELD)).replace("round=19", "round=20")]
         + good[3:], False),                                            # the unheld asid got a round
        (swap(3, "unheld_targets=0", "unheld_targets=1"), False),
        (swap(3, "PASS", "FAIL"), False),
        (good + ["[SMP] acks outstanding=1 va=0x0000000000400000 pages=1"], False),
        (swap(0, "online=4", "online=1"), False),
        ([good[0], good[2], good[1], good[3]], False),                  # held before full
        ([good[3]] + good[:3], False),                                  # proof line before the rounds
        (swap(1, setw(1, 2, 3), setw(1, 2)), False),              # set short of targets
    ]
    wrong = [i for i, (lines, ok) in enumerate(cases) if (judge(list(lines)) is None) != ok]
    print(f"shootdown-proof-log: self-test {'failed on ' + str(wrong) if wrong else 'passed'}, "
          f"{sum(not ok for _, ok in cases)} bad logs rejected")
    return 1 if wrong else 0


def main(argv):
    if argv == ["--self-test"]:
        return self_test()
    if len(argv) != 2 or argv[0] != "--log":
        print(__doc__)
        return 2
    with open(argv[1], errors="replace") as f:
        why = judge(f.read().split("\n"))
    print(f"shootdown-proof-log: {'FAIL ' + why if why else 'PASS: a full flush reached every AP, a user-asid round reached its holder alone'}")
    return 1 if why else 0


if __name__ == "__main__":
    sys.exit(main(sys.argv[1:]))
