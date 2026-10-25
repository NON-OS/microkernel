#!/usr/bin/env python3
# NONOS Operating System (AGPL-3.0-or-later)
"""A build script writes only under its OUT_DIR, so it can never put a key in
nonos-bootloader/keys/ or .keys/. Every call that writes, creates, copies or
renames a file or a directory must take, as its target, a name the script
bound from OUT_DIR; any other target fails, and OpenOptions fails outright.

Usage: check-build-writes.py <build.rs>      check that file
       check-build-writes.py --selftest <build.rs>
                                             also show each mutant fails it
"""
import re
import sys

WRITES = re.compile(r"\b(?:fs::)?(write|create_dir_all|create_dir|copy|rename|File::create)\s*\(")
OPEN_OPTIONS = re.compile(r"\bOpenOptions\b")
OUT_DIR = re.compile(r"let\s+(\w+)\s*=\s*env::var\(\s*\"OUT_DIR\"\s*\)")
JOINED = re.compile(r"let\s+(?:mut\s+)?(\w+)\s*=\s*Path::new\(\s*&?(\w+)\s*\)\.join\(")


def code(text):
    return "\n".join(line.split("//")[0] for line in text.split("\n"))


def bad_writes(text):
    src = code(text)
    roots = set(OUT_DIR.findall(src))
    safe = {name for name, base in JOINED.findall(src) if base in roots}
    out = []
    for n, line in enumerate(src.split("\n"), 1):
        if OPEN_OPTIONS.search(line):
            out.append(f"{n}: OpenOptions")
        for m in WRITES.finditer(line):
            # A bare write( is a method on a file already opened, checked there.
            if m.group(1) in ("write", "copy", "rename") and "fs::" not in line[: m.end()][-12:]:
                continue
            args = line[m.end():].split(")")[0].split(",")
            # copy and rename write their second argument; the rest their first.
            arg = args[1 if m.group(1) in ("copy", "rename") and len(args) > 1 else 0]
            arg = arg.strip().lstrip("&").strip()
            if arg not in safe:
                out.append(f"{n}: {m.group(1)}({arg}) is not a path under OUT_DIR")
    return out


MUTANTS = [
    'fs::write("keys/signing_key_v1.bin", &key).unwrap();',
    'fs::create_dir_all(Path::new(".keys")).unwrap();',
    'let default = "keys/signing_key_v1.bin"; fs::write(default, &key).unwrap();',
    'let mut f = fs::File::create(&default).unwrap();',
    'let f = std::fs::OpenOptions::new().write(true).open("keys/k");',
    'fs::copy(&dest, "keys/signing_key_v1.bin").unwrap();',
    'let d = Path::new("keys").join("k"); fs::write(&d, &key).unwrap();',
]


def main(argv):
    selftest = argv[:1] == ["--selftest"]
    path = argv[-1]
    text = open(path, encoding="utf-8").read()
    bad = bad_writes(text)
    for b in bad:
        print(f"::error::{path}:{b}")
    if selftest:
        for m in MUTANTS:
            if not bad_writes(text + "\nfn mutant() {\n    " + m + "\n}\n"):
                print(f"::error::check-build-writes passes a mutant: {m}")
                bad.append(m)
        print(f"check-build-writes: {len(MUTANTS)} mutants checked")
    return 1 if bad else 0


if __name__ == "__main__":
    sys.exit(main(sys.argv[1:]))
