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
"""What each cargo build in the flake reads, written to tools/nix/inputs.json.

For every crate root the flake builds or tests (ROOTS), the files it can
read: the crates its path dependencies reach, each whole but for the crates
nested inside it that it does not use; every file a #[path], include_bytes!,
include_str! or other path literal in their Rust names, followed through the
modules a mounted file declares; the .cargo directories cargo reads on the
way up; and the files a marker in the source says the build script hands
it. The repository root is a crate whose directory holds everything, so of
it only the manifest, the build scripts and src/ count.

tools/nix/src.nix turns an entry into the derivation's source, without *.md
unless the Rust names one. `--check` fails when the committed file is not
what this prints, which `nix flake check` runs (inputs drift check).
"""

import glob
import json
import os
import re
import subprocess
import sys
import tomllib

HERE = os.path.dirname(os.path.abspath(__file__))
TOP = os.path.normpath(os.path.join(HERE, "..", ".."))
OUT = os.path.join(HERE, "inputs.json")

# The crate roots outside the capsule catalogue and the *_proofs crates
# (tools/nix/checks.nix and image.nix name them).
EXTRA_ROOTS = [
    ".",
    "nonos-bootloader",
    "nonos-bootloader/boot_proofs",
    "nonos-bootloader/tools/embed-trailer",
    "nonos-bootloader/tools/sign-kernel",
    "nonos-sign",
    "nonos-stark-enroll",
    "nonos-mk",
    "nonos-verify",
    "nonos-attest-path",
    "security/nonos-secops",
    "tools/nonos-pack",
    "toolchain/nonos-rt",
    "userland/nonos_secp256k1",
    "userland/nonos_ed25519",
    "userland/nonos_i2cmodel",
    "userland/attest_receipt",
    "userland/upstream-src/sd-1.0.0",
    "userland/upstream-src/tokio-smoke",
    "userland/upstream-src/grex",
    "userland/upstream-src/dotenv-linter",
    "userland/upstream-src/pastel",
    "userland/upstream-src/jsonxf",
    "userland/upstream-src/tokei",
    "userland/upstream-src/huniq",
    "userland/upstream-src/csview",
]

# What the repository root crate is, beyond the files its Rust names.
ROOT_CRATE = ["Cargo.toml", "Cargo.lock", "build.rs", "src"]

# A marker in a crate's source, and the files the build hands it for that
# marker (tools/nix/capsules.nix, signedInputs).
MARKERS = {
    "NONOS_BUILD_SHA": ["nonos-data/trust/COMMIT"],
    "market/index.bin": ["nonos-data/market/index.bin"],
    "models/catalogue.bin": ["nonos-data/models/catalogue.bin"],
}

# The trust set the seal rewrites as it signs and enrolls. A capsule that read
# any of it would be rebuilt between its signing and the kernel's build, and
# the kernel would embed bytes its signed manifest does not name, which the
# spawn gate refuses (vfs_pool, which names hello's trust files for a
# development seed it does not build here). So no capsule's build sees it but
# for what a marker hands it.
TRUST = "nonos-data/trust"

DEP_TABLES = ("dependencies", "dev-dependencies", "build-dependencies")
LITERAL = re.compile(r'"((?:[^"\\\n]|\\.){1,300})"')
PATHLIKE = re.compile(r"^[\w./+@-]+$")
PY_IMPORT = re.compile(r"^\s*(?:from|import)\s+(\w+)", re.M)
RERUN = re.compile(r"rerun-if-changed=([\w./+@-]+)")
MODULE = re.compile(r"^\s*(?:pub(?:\([\w:]+\))?\s+)?mod\s+(\w+)\s*;", re.M)


def tracked():
    out = subprocess.run(["git", "-C", TOP, "ls-files", "-z"], check=True,
                         capture_output=True).stdout.decode()
    files = {f for f in out.split("\0") if f and os.path.lexists(os.path.join(TOP, f))}
    dirs = set()
    for f in files:
        d = os.path.dirname(f)
        while d and d not in dirs:
            dirs.add(d)
            d = os.path.dirname(d)
    return files, dirs


# What `nix flake check` copies into its sandbox (src.nix, everything), so the
# check computes the same table there as here.
UNSEEN = ("media/", "screenshots/", "wallpapers/")
FILES, DIRS = tracked()
FILES = {f for f in FILES if not f.startswith(UNSEEN)}
DIRS = {d for d in DIRS if not (d + "/").startswith(UNSEEN)}
CRATES = {os.path.dirname(f) or "." for f in FILES if os.path.basename(f) == "Cargo.toml"}


def norm(p):
    p = os.path.normpath(p)
    return "." if p in ("", ".") else p


def under(p, d):
    return d == "." or p == d or p.startswith(d + "/")


def manifest(d):
    path = os.path.join(TOP, d, "Cargo.toml")
    with open(path, "rb") as f:
        return tomllib.load(f)


def dep_paths(d, t):
    """Every crate directory a manifest names by path."""
    tables = [t.get(k, {}) for k in DEP_TABLES]
    for target in t.get("target", {}).values():
        tables += [target.get(k, {}) for k in DEP_TABLES]
    tables += list(t.get("patch", {}).values())
    tables.append(t.get("replace", {}))
    ws = t.get("workspace", {})
    tables.append(ws.get("dependencies", {}))
    out = []
    for table in tables:
        for spec in table.values():
            if isinstance(spec, dict) and "path" in spec:
                out.append(norm(os.path.join(d, spec["path"])))
    for pattern in ws.get("members", []):
        for m in glob.glob(os.path.join(TOP, d, pattern)):
            out.append(norm(os.path.relpath(m, TOP)))
    pkg_ws = t.get("package", {}).get("workspace")
    if isinstance(pkg_ws, str):
        out.append(norm(os.path.join(d, pkg_ws)))
    return out


def config_patches(root):
    """The [patch] paths of the cargo configs cargo reads, run from `root`."""
    out, d = [], root
    while True:
        for name in ("config.toml", "config"):
            cfg = norm(os.path.join(d, ".cargo", name))
            if cfg in FILES:
                with open(os.path.join(TOP, cfg), "rb") as f:
                    t = tomllib.load(f)
                for table in t.get("patch", {}).values():
                    for spec in table.values():
                        if isinstance(spec, dict) and "path" in spec:
                            out.append(norm(os.path.join(d, spec["path"])))
        if d == ".":
            return out
        d = norm(os.path.dirname(d))


def closure(root):
    seen, todo = set(), [root] + config_patches(root)
    while todo:
        d = todo.pop()
        if d in seen or d not in CRATES:
            continue
        seen.add(d)
        todo += dep_paths(d, manifest(d))
    return seen


class Inputs:
    def __init__(self, root):
        self.root = root
        self.crates = closure(root)
        self.include = set()
        self.exclude = set()
        self.scanned = set()
        self.queue = []
        # Files the seal writes and stages before it builds (MARKERS): they
        # are not committed on a fresh clone, so they are kept by name and
        # src.nix takes each one when it is there.
        self.signed = set()
        self.capsule = root in capsule_dirs()

    def covered(self, p):
        best = None
        for i in self.include:
            if under(p, i) and (best is None or len(i) > len(best)):
                best = i
        if best is None:
            return False
        return not any(under(p, x) and under(x, best) and x != best for x in self.exclude)

    def rs_under(self, d):
        return [f for f in FILES if f.endswith(".rs") and under(f, d) and self.covered(f)]

    def add(self, p):
        if self.covered(p) or (self.capsule and under(p, TRUST)):
            return
        self.include.add(p)
        if p in FILES:
            if self.python(p):
                self.imports(p)
            if p.endswith(".rs"):
                self.queue.append(p)
                stem, name = p[:-3], os.path.basename(p)
                # A mounted module's own modules: beside it, or in its directory.
                if name in ("mod.rs", "lib.rs", "main.rs"):
                    self.add(norm(os.path.dirname(p)))
                elif stem in DIRS:
                    self.add(stem)
        else:
            self.queue += self.rs_under(p)

    @staticmethod
    def python(p):
        if p.endswith(".py"):
            return True
        with open(os.path.join(TOP, p), "rb") as f:
            return b"python" in f.readline()

    def imports(self, p):
        """The modules and packages beside a Python tool that it imports."""
        with open(os.path.join(TOP, p), encoding="utf-8", errors="replace") as f:
            body = f.read()
        here = norm(os.path.dirname(p))
        for name in PY_IMPORT.findall(body):
            for q in (f"{here}/{name}", f"{here}/{name}.py"):
                q = norm(q)
                if q in DIRS:
                    self.add(q)
                    for f in sorted(FILES):
                        if under(f, q) and f.endswith(".py") and f not in self.scanned:
                            self.scanned.add(f)
                            self.imports(f)
                elif q in FILES and q not in self.scanned:
                    self.scanned.add(q)
                    self.add(q)
                    self.imports(q)

    def crate(self, d):
        if d == ".":
            for p in ROOT_CRATE:
                if p in FILES or p in DIRS:
                    self.add(p)
            return
        self.include.add(d)
        for nested in CRATES:
            if nested != d and under(nested, d) and nested not in self.crates:
                self.exclude.add(nested)
        self.queue += self.rs_under(d)

    def literal(self, src, crate, text):
        # A path built with format!: what precedes the first hole, or after a
        # leading one ("{root}/../x", the crate's directory), up to the next.
        if "{" in text:
            lead = re.match(r"^\{\w*\}/([^{]+)", text)
            text = lead.group(1) if lead else text[: text.index("{")]
            if "{" not in text and not lead:
                text = text.rpartition("/")[0]
            text = text.rstrip("/")
            if not text:
                return
        if not PATHLIKE.match(text):
            return
        # A bare name counts only as a file beside the source or in the crate.
        bare = "/" not in text and "." not in text.strip(".")
        bases = [os.path.dirname(src), crate]
        for base in bases:
            for t in (text, text.lstrip("/")):
                p = norm(os.path.join(base, t))
                if p.startswith("..") or p == "." or under(crate, p):
                    continue
                if p in FILES or (p in DIRS and not bare):
                    self.add(p)
                elif p.startswith("nonos-data/") and not bare:
                    # Not on a fresh clone, as what the seal writes and
                    # stages there before it builds is not: taken when there.
                    self.signed.add(p)

    def modules(self, f, body):
        """`mod x;` in a file no crate directory holds, as a build script's."""
        if any(under(f, c) for c in self.crates if c != "."):
            return
        here, stem = os.path.dirname(f), f[:-3]
        homes = [here] if os.path.basename(f) in ("mod.rs", "lib.rs", "main.rs", "build.rs") else [stem, here]
        for name in MODULE.findall(body):
            for home in homes:
                for p in (f"{home}/{name}.rs", f"{home}/{name}/mod.rs"):
                    p = norm(p.lstrip("/"))
                    if p in FILES:
                        self.add(p)

    def scan(self):
        while self.queue:
            f = self.queue.pop()
            if f in self.scanned:
                continue
            self.scanned.add(f)
            with open(os.path.join(TOP, f), encoding="utf-8", errors="replace") as h:
                body = h.read()
            crate = max((c for c in self.crates if under(f, c)), key=len, default=self.root)
            for m in LITERAL.finditer(body):
                self.literal(f, crate, m.group(1))
            for m in RERUN.finditer(body):
                self.literal(f, crate, m.group(1))
            self.modules(f, body)
            for marker, files in MARKERS.items():
                if marker in body:
                    self.signed.update(files)

    def cargo_configs(self):
        for c in self.crates:
            d = c
            while True:
                cfg = norm(os.path.join(d, ".cargo"))
                if cfg in DIRS:
                    self.add(cfg)
                if d == ".":
                    break
                d = norm(os.path.dirname(d))

    def collapse(self):
        """A directory whose every file is named already is named instead."""
        while True:
            parents = {norm(os.path.dirname(p)) for p in self.include}
            full = [d for d in parents if d != "." and d not in self.include
                    and not any(under(c, d) for c in self.crates)
                    and all(self.covered(f) for f in FILES if under(f, d))]
            if not full:
                return
            for d in full:
                self.include = {p for p in self.include if not under(p, d)}
                self.include.add(d)

    def run(self):
        for c in sorted(self.crates, key=len):
            self.crate(c)
        self.cargo_configs()
        self.scan()
        self.collapse()
        inc = sorted(p for p in self.include if not any(
            under(p, q) and q != p and not any(under(p, x) and under(x, q) for x in self.exclude)
            for q in self.include))
        exc = sorted(x for x in self.exclude if any(under(x, i) for i in inc))
        md = sorted(p for p in FILES if p.endswith(".md") and p in self.include)
        # Named inside a nested crate the build does not use: kept past the
        # exclusion (src.nix adds these after it).
        named = sorted(p for p in inc if any(under(p, x) for x in exc))
        return {"include": inc, "exclude": exc, "md": sorted(set(md) | set(named)),
                "signed": sorted(self.signed)}


def capsule_dirs():
    with open(os.path.join(HERE, "capsules.json")) as f:
        return {e["dir"] for e in json.load(f) if not e["prebuilt"]}


def roots():
    found = capsule_dirs()
    for d in sorted(DIRS):
        if d.startswith("userland/") and d.count("/") == 1 and d.endswith("_proofs") \
                and f"{d}/Cargo.lock" in FILES:
            found.add(d)
    found.update(r for r in EXTRA_ROOTS if r == "." or r in CRATES)
    return sorted(found)


def main():
    table = {r: Inputs(r).run() for r in roots()}
    text = json.dumps(table, indent=1, sort_keys=True) + "\n"
    if "--check" in sys.argv:
        with open(OUT) as f:
            old = json.loads(f.read())
        if old != table:
            stale = sorted(k for k in set(old) | set(table) if old.get(k) != table.get(k))
            sys.exit("tools/nix/inputs.json is not what the tree reads, for: "
                     + " ".join(stale) + "\nrun tools/nix/inputs.py")
        return
    with open(OUT, "w") as f:
        f.write(text)


if __name__ == "__main__":
    main()
