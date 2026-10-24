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
# along with this program. If not, see <https://www.gnu.org/licenses/>.
"""Hold scripts/baselines/declared-limits.txt to what it may say.

nonos-harden reads GO over a nonos-verify gap only when the gap is on that
list, so the list is part of what GO means. This check fails when:
  - the file is not in its format (`# format: nonos-harden-declared-limits/1`
    first, then `<module>: <what>` lines, sorted, none twice);
  - a line names a module nonos-harden does not run, or a gap that module's
    source no longer declares (the first literal argument of a `.gap(` call);
    such a line must be deleted, so the list shrinks as gaps are closed;
  - a line is not in the list as the previous commit had it (or the
    merge-base with the ref in NONOS_BASELINE_BASE, when set): the list may
    lose lines and never gain them.
--self-test proves each rule bites on a decoy.
"""

import os
import re
import subprocess
import sys
import tempfile
from pathlib import Path

BASELINE = "scripts/baselines/declared-limits.txt"
HEADER = "# format: nonos-harden-declared-limits/1"
MODULES = {
    "hygiene": "nonos-verify/src/hygiene",
    "trust-chain": "nonos-verify/src/trust_chain.rs",
    "supply-chain": "nonos-verify/src/supply_chain.rs",
}
GAP = re.compile(r'\.gap\(\s*"((?:[^"\\]|\\.)*)"')


def entries(text):
    lines = text.splitlines()
    if not lines or lines[0] != HEADER:
        return None, [f"the first line is not `{HEADER}`"]
    out, bad = [], []
    for n, line in enumerate(lines[1:], 2):
        if not line.strip() or line.startswith("#"):
            continue
        module, sep, what = line.partition(": ")
        if not sep or not what.strip():
            bad.append(f"line {n} is not `<module>: <what>`")
        else:
            out.append((module, what))
    if out != sorted(set(out)):
        bad.append("the lines are not sorted, or one is there twice")
    return out, bad


def declared(root, module):
    p = root / MODULES[module]
    files = sorted(p.rglob("*.rs")) if p.is_dir() else [p] if p.is_file() else []
    return {m for f in files for m in GAP.findall(f.read_text(errors="replace"))}


def previous(root):
    def git(*a):
        r = subprocess.run(["git", "-C", str(root), *a], capture_output=True, text=True)
        return r.stdout.strip() if r.returncode == 0 else None
    base_ref = os.environ.get("NONOS_BASELINE_BASE")
    base = git("merge-base", "HEAD", base_ref) if base_ref else None
    if not base or base == git("rev-parse", "HEAD"):
        base = git("rev-parse", "--verify", "-q", "HEAD^")
    if not base:
        return None
    text = git("show", f"{base}:{BASELINE}")
    return None if text is None else set(entries(text + "\n")[0] or [])


def check(root, before):
    text = (root / BASELINE).read_text() if (root / BASELINE).is_file() else ""
    have, bad = entries(text)
    for module, what in have or []:
        if module not in MODULES:
            bad.append(f"{module}: nonos-harden runs no such nonos-verify module")
        elif what not in declared(root, module):
            bad.append(f"{module}: {what}: the source declares no such gap; delete the line")
        elif before is not None and (module, what) not in before:
            bad.append(f"{module}: {what}: added; the list may only shrink")
    return bad


def self_test():
    with tempfile.TemporaryDirectory() as t:
        root = Path(t)
        (root / "nonos-verify/src").mkdir(parents=True)
        (root / "nonos-verify/src/trust_chain.rs").write_text('rpt.gap(\n "a b", "x");')
        (root / "scripts/baselines").mkdir(parents=True)
        cases = [
            (f"{HEADER}\ntrust-chain: a b\n", {("trust-chain", "a b")}, 0),
            ("trust-chain: a b\n", None, 1),
            (f"{HEADER}\ntrust-chain: gone\n", None, 1),
            (f"{HEADER}\nhygiene-x: a b\n", None, 1),
            (f"{HEADER}\ntrust-chain: a b\ntrust-chain: a b\n", None, 1),
            (f"{HEADER}\ntrust-chain: a b\n", set(), 1),
        ]
        for text, before, want in cases:
            (root / BASELINE).write_text(text)
            got = len(check(root, before)) > 0
            assert got == bool(want), f"self-test: {text!r} before={before} gave {got}"
    print("declared-limits: self-test bites on each decoy")


def main(argv):
    if "--self-test" in argv:
        self_test()
        return 0
    root = Path(".")
    bad = check(root, previous(root))
    for b in bad:
        print(f"declared-limits: {b}", file=sys.stderr)
    if bad:
        return 1
    print(f"declared-limits: {len(entries((root / BASELINE).read_text())[0])} declared, all still in the source")
    return 0


if __name__ == "__main__":
    sys.exit(main(sys.argv[1:]))
