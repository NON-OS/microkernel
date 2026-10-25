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

"""The UI copy rules of the design notes (Voice), as a static check.

    nonos-ci/check-ui-copy.py              every UI string in the tree
    nonos-ci/check-ui-copy.py --self-test  the rules on strings made to fail

A UI string is a Rust string or byte-string literal in the source of a
windowed capsule (one that links app_skeleton), the desktop shell, setup,
login or the toolkit, that reads as prose: it has a space, or starts with a
capital. The Terminal is the owner's and is not read. A serial line (one
starting with "[") is a log, not copy. Each string is refused for:

  a hype word: seamless, unlock, empower, elevate, dive in, supercharge,
      effortless, magic, revolutionary, next-level, in any inflection;
  an emoji or pictograph, or the sparkle U+2728;
  an exclamation mark ending a word ("n!", one lower-case token, is not copy);
  Title Case: three or more words each capitalised, unless the whole string
      is a proper name in PROPER (a font, a product, a standard).

Every refusal prints its file, line and string; the exit is 1 if any.
"""

import argparse
import glob
import os
import re
import sys

ROOT = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
ALWAYS = ("capsule_desktop_shell", "capsule_setup_wizard", "capsule_login", "toolkit")
NEVER = ("capsule_terminal", "app_skeleton")

HYPE = re.compile(r"\b(seamless(ly)?|unlock(s|ed|ing)?|empower(s|ed|ing|ment)?|elevat(e|es|ed|ing|ion)|"
                  r"dive in|supercharg(e|es|ed|ing)|effortless(ly)?|magic(al|ally)?|revolutionary|next-level)\b",
                  re.I)
EMOJI = re.compile("[☀-➿⭐⭕\U0001f000-\U0001faff]")
SHOUT = re.compile(r"[A-Za-z]![\s\"']|[A-Za-z]!$")
TITLE = re.compile(r"^[A-Z][a-z]+( [A-Z][a-z]+){2,}$")
PROPER = {"Noto Sans Mono", "Noto Sans Arabic", "Noto Sans Hebrew", "Noto Sans Devanagari", "JetBrains Mono Regular"}
LITERAL = re.compile(r'b?"((?:[^"\\]|\\.)*)"')


def crates():
    """The UI crates: every capsule that links app_skeleton, and ALWAYS."""
    found = set(ALWAYS)
    for toml in glob.glob(os.path.join(ROOT, "userland", "*", "Cargo.toml")):
        name = os.path.basename(os.path.dirname(toml))
        if "app_skeleton" in open(toml).read() and not name.endswith("_proofs"):
            found.add(name)
    return sorted(found - set(NEVER))


def refusals(s):
    """Why `s` breaks the copy rules, if it is UI copy at all."""
    if s.startswith("[") or not (" " in s or s[:1].isupper()):
        return []
    why = []
    if HYPE.search(s):
        why.append(f"hype word {HYPE.search(s).group(0)!r}")
    if EMOJI.search(s):
        why.append("emoji")
    if SHOUT.search(s):
        why.append("exclamation mark")
    if TITLE.match(s) and s not in PROPER:
        why.append("Title Case")
    return why


def scan():
    bad = 0
    for crate in crates():
        for path in sorted(glob.glob(os.path.join(ROOT, "userland", crate, "src", "**", "*.rs"), recursive=True)):
            for n, line in enumerate(open(path, errors="replace"), 1):
                if line.lstrip().startswith("//"):
                    continue
                for m in LITERAL.finditer(line):
                    for why in refusals(m.group(1)):
                        bad += 1
                        print(f"{os.path.relpath(path, ROOT)}:{n}: {why}: {m.group(1)!r}")
    print(f"check-ui-copy: {len(crates())} crates, {bad} refused")
    return 1 if bad else 0


def self_test():
    refused = ["Unlock your privacy", "Seamlessly synced", "A magical desktop", "next-level speed",
               "Saved ✨", "Done \U0001f389", "Installed!", "Saved! Close it.", "Show All Apps",
               "Elevate your work", "Dive in now"]
    taken = ["n!", "Next level", "the volume is locked; open it with its passphrase", "unlocked",
             "Noto Sans Mono", "Show all apps", "[SETUP] Done!", "Privileged", "Not now", "a != b"]
    wrong = [s for s in refused if not refusals(s)] + [s for s in taken if refusals(s)]
    if wrong:
        sys.exit(f"check-ui-copy self-test: rules wrong on {wrong}")
    print(f"check-ui-copy self-test: {len(refused)} refused, {len(taken)} taken")
    return 0


def main():
    ap = argparse.ArgumentParser(description=__doc__.split("\n", 1)[0])
    ap.add_argument("--self-test", action="store_true")
    a = ap.parse_args()
    return self_test() if a.self_test else scan()


if __name__ == "__main__":
    sys.exit(main())
