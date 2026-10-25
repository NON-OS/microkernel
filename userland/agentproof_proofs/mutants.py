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

"""Each control in agentproof_policy, removed one at a time: the proofs must
then fail. mutants.json names each by one exact substitution; a substitution
that no longer applies exactly once is itself a failure, so a mutant cannot
rot into testing nothing. The source is put back after each, whatever the
result.

    python3 mutants.py            run every mutant
    python3 mutants.py NAME ...   run those named
"""

import json
import subprocess
import sys
from pathlib import Path

HERE = Path(__file__).resolve().parent
POLICY = HERE.parent / "agentproof_policy"


def caught(m):
    path = POLICY / m["file"]
    before = path.read_text()
    if before.count(m["old"]) != 1:
        return "stale"
    path.write_text(before.replace(m["old"], m["new"]))
    try:
        run = subprocess.run(["cargo", "test", "--release", "-q"], cwd=HERE,
                             capture_output=True, text=True)
        return "caught" if run.returncode != 0 else "ESCAPED"
    finally:
        path.write_text(before)


def main():
    mutants = json.loads((HERE / "mutants.json").read_text())
    named = set(sys.argv[1:])
    bad = 0
    for m in mutants:
        if named and m["name"] not in named:
            continue
        verdict = caught(m)
        bad += verdict != "caught"
        print(f"[mutant] {m['name']}: {verdict}")
    print(f"[mutant] {bad} not caught")
    return 1 if bad else 0


if __name__ == "__main__":
    sys.exit(main())
