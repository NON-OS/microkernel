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
"""Run nonos-harden's mutants (mutants.json): each is applied to a copy of
the crate, the proofs run, and the mutant is caught only if the proof it
names (`caught_by`, a test path) fails. A mutant that does not build, or
that no proof catches, or that the named proof does not catch, is reported
and the run exits 1. The crate on disk is never changed.

  python3 mutants.py              run every mutant
  python3 mutants.py NAME...      run the named ones
  python3 mutants.py --discover   print which proofs fail for each mutant
"""
import functools, json, os, re, shutil, subprocess, sys, tempfile

HERE = os.path.dirname(os.path.abspath(__file__))
ROOT = os.path.dirname(HERE)
SELF_CHECK = "tests::mutants_tests::every_mutant_applies_exactly_once_and_names_its_proof"

print = functools.partial(print, flush=True)


def copy_crate(dst):
    shutil.copytree(HERE, dst, ignore=shutil.ignore_patterns("target", "mutants.py"))
    toml = os.path.join(dst, "Cargo.toml")
    text = open(toml).read().replace('path = "../', 'path = "%s/' % ROOT)
    open(toml, "w").write(text)
    # Proofs that run the tree's own tools, or read its committed public keys,
    # reach them as ../tools and ../nonos-data.
    for shared in ("tools", "nonos-data"):
        os.symlink(os.path.join(ROOT, shared), os.path.join(os.path.dirname(dst), shared))


def failing(m, target):
    with tempfile.TemporaryDirectory(prefix="nonos-harden-mutant-") as tmp:
        crate = os.path.join(tmp, "nonos-harden")
        copy_crate(crate)
        path = os.path.join(crate, m["file"])
        src = open(path).read()
        if src.count(m["old"]) != 1:
            return None, "`old` does not occur exactly once in %s" % m["file"]
        open(path, "w").write(src.replace(m["old"], m["new"]))
        env = dict(os.environ, CARGO_TARGET_DIR=target)
        run = subprocess.run(["cargo", "test", "--quiet", "--no-fail-fast"], cwd=crate,
                             env=env, capture_output=True, text=True)
        out = run.stdout + run.stderr
        if "error[E" in out or "could not compile" in out:
            return None, "did not build"
        found = set(re.findall(r"^---- (\S+) stdout ----$", out, re.M))
        # The mutant list's own proof fails in every mutated copy, by design.
        return found - {SELF_CHECK}, ""


def main(argv):
    discover = "--discover" in argv
    names = [a for a in argv if not a.startswith("--")]
    mutants = json.load(open(os.path.join(HERE, "mutants.json")))
    if names:
        mutants = [m for m in mutants if m["name"] in names]
    target = os.path.join(ROOT, "target", "nonos-harden-mutants")
    bad = 0
    for m in mutants:
        failed, why = failing(m, target)
        if failed is None:
            print("ESCAPED %s: %s" % (m["name"], why))
            bad += 1
        elif discover:
            print("%s: %s" % (m["name"], " ".join(sorted(failed)) or "-"))
        elif m["caught_by"] in failed:
            print("caught  %s by %s" % (m["name"], m["caught_by"]))
        else:
            print("ESCAPED %s: %s passed (failing: %s)" % (m["name"], m["caught_by"], sorted(failed) or "none"))
            bad += 1
    print("%d mutants, %d escaped" % (len(mutants), bad))
    return 1 if bad else 0


if __name__ == "__main__":
    sys.exit(main(sys.argv[1:]))
