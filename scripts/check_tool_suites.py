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
"""Every Python test under tools/ runs in a lane.

A lane here is a unittest command one of LANES runs: a discover
(`-m unittest discover -s DIR [-t TOP] [-p PATTERN]`) reaches each file under
DIR, through packages only, whose name PATTERN matches (test*.py when none is
given); a module run (`(cd tools && python3 -m unittest pkg.module)`) reaches
that one file. A tools/**/test_*.py that no lane reaches fails, named: a suite
that runs nowhere rots unseen, as the market catalogue's once did. It reads
the lanes as text; it never runs a test.
"""

import argparse
import fnmatch
import re
import sys
import tempfile
from pathlib import Path

LANES = ("tools/nix/checks.nix", "nonos-ci/run-static-checks.sh")
DISCOVER = re.compile(r"-m unittest discover\s+-s\s+(\S+)((?:\s+-[tp]\s+\S+)*)")
OPTION = re.compile(r"-([tp])\s+(\S+)")
MODULE = re.compile(r"cd tools && python3 -m unittest(?:\s+-q)?\s+([\w.]+)")


def commands(text):
    """(dir, pattern) for each discover and each module run, in tools/ terms."""
    text = text.replace("\\\n", " ")
    out = []
    for m in DISCOVER.finditer(text):
        opts = dict(OPTION.findall(m.group(2)))
        out.append((m.group(1).strip("'\""), opts.get("p", "test*.py").strip("'\"")))
    for m in MODULE.finditer(text):
        *pkg, mod = m.group(1).split(".")
        out.append(("tools/" + "/".join(pkg), mod + ".py"))
    return out


def reached(root, start, pattern):
    """The files a discover from `start` with `pattern` runs."""
    base = root / start
    if not base.is_dir():
        return set()
    found, todo = set(), [base]
    while todo:
        d = todo.pop()
        for p in sorted(d.iterdir()):
            if p.is_dir() and (p / "__init__.py").is_file():
                todo.append(p)
            elif p.is_file() and fnmatch.fnmatch(p.name, pattern):
                found.add(p.relative_to(root).as_posix())
    return found


def unreached(root):
    tests = {p.relative_to(root).as_posix() for p in (root / "tools").rglob("test_*.py")
             if "__pycache__" not in p.parts}
    ran = set()
    for lane in LANES:
        at = root / lane
        for start, pattern in commands(at.read_text() if at.is_file() else ""):
            ran |= reached(root, start, pattern)
    return sorted(tests - ran)


def fixture(lanes):
    """A tree of four suites under tools/, and `lanes` as the nix lane."""
    root = Path(tempfile.mkdtemp())
    for f in ("a/test_x.py", "b/test_y.py", "b/test_mutants.py", "c/deep/test_z.py", "d/test_w.py"):
        (root / "tools" / f).parent.mkdir(parents=True, exist_ok=True)
        (root / "tools" / f).write_text("")
    for pkg in ("a", "b", "c", "c/deep", "d"):
        (root / "tools" / pkg / "__init__.py").write_text("")
    (root / LANES[0]).parent.mkdir(parents=True)
    (root / LANES[0]).write_text(lanes)
    return root


def self_test(find=unreached, say=True):
    """Each rule bites: an unlisted suite fails, a pattern that leaves a file
    out fails it, a package below a discover is reached, a module run reaches
    its one file. `find` is the rule under test, so a mutant is proved caught."""
    every = ("-m unittest discover -s tools/a -p 'test_*.py'\n"
             "-m unittest discover -s tools/b -t tools \\\n  -p 'test_[!m]*.py'\n"
             "-m unittest discover -s tools/b -t tools -p 'test_mutants.py'\n"
             "-m unittest discover -s tools/c\n(cd tools && python3 -m unittest -q d.test_w)\n")
    cases = [
        (every, []),
        (every.replace("-p 'test_mutants.py'", "-p 'test_none.py'"), ["tools/b/test_mutants.py"]),
        (every.replace("-s tools/c", "-s tools/nowhere"), ["tools/c/deep/test_z.py"]),
        (every.replace("d.test_w", "d.test_v"), ["tools/d/test_w.py"]),
    ]
    bad = [lanes for lanes, want in cases if find(fixture(lanes)) != want]
    for lanes in bad if say else ():
        print(f"tool-suites: self-test failed on lanes:\n{lanes}")
    return 1 if bad else 0


def mutant_caught():
    """The rule with every file taken as reached: the self-test must fail it."""
    return self_test(lambda root: [], say=False) != 0


def main():
    ap = argparse.ArgumentParser(description=__doc__.splitlines()[0])
    ap.add_argument("--root", type=Path, default=Path("."))
    ap.add_argument("--self-test", action="store_true", help="prove each rule bites, and its mutant is caught")
    args = ap.parse_args()
    if args.self_test:
        failed = self_test()
        if not mutant_caught():
            print("tool-suites: a rule that reaches every file passed the self-test")
            failed = 1
        print(f"tool-suites: self-test {'failed' if failed else 'passed'}")
        return failed
    left = unreached(args.root)
    for f in left:
        print(f"tool-suites: {f} runs in no lane ({', '.join(LANES)})")
    print(f"tool-suites: {len(left)} suite file(s) in no lane")
    return 1 if left else 0


if __name__ == "__main__":
    sys.exit(main())
